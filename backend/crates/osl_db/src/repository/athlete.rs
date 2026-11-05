use osl_domain::{AthleteStatus, Gender, Movement, RisSource};
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

use crate::error::{Result, StorageError};
use crate::params::Page;
use crate::projections::athlete::{
    AthleteBalanceRow, AthleteCompetitionRow, AthleteDetail, AthleteLiftRow, PersonalRecordRow,
};
use crate::rows::athlete::AthleteRow;

pub struct AthleteRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AthleteRepository<'a> {
    #[must_use]
    pub const fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "repository.athlete.list", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn list(&self, page: &Page) -> Result<(Vec<AthleteRow>, i64)> {
        let athletes = sqlx::query_as!(
            AthleteRow,
            r#"
            SELECT athlete_id, first_name, last_name, native_name, gender as "gender: Gender", created_at,
                   country, profile_picture_url, slug,
                   COALESCE(slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
            FROM athletes
            ORDER BY last_name, first_name
            LIMIT $1 OFFSET $2
            "#,
            page.limit,
            page.offset
        )
        .fetch_all(self.pool)
        .await?;

        let total = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM athletes"#)
            .fetch_one(self.pool)
            .await?;

        Ok((athletes, total))
    }

    #[tracing::instrument(name = "repository.athlete.find_by_slug", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn find_by_slug(&self, slug: &str) -> Result<AthleteRow> {
        let athlete = sqlx::query_as!(
            AthleteRow,
            r#"
            SELECT athlete_id, first_name, last_name, native_name, gender as "gender: Gender", created_at,
                   country, profile_picture_url, slug,
                   COALESCE(slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
            FROM athletes
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(self.pool)
        .await?;

        if let Some(athlete) = athlete {
            return Ok(athlete);
        }

        let athlete_from_history = sqlx::query_as!(
            AthleteRow,
            r#"
            SELECT athlete_id, first_name, last_name, native_name, gender as "gender: Gender", created_at,
                   country, profile_picture_url, slug,
                   COALESCE(slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
            FROM athletes
            WHERE slug_history @> to_jsonb($1::text)
            "#,
            slug
        )
        .fetch_optional(self.pool)
        .await?
        .ok_or(StorageError::NotFound)?;

        Ok(athlete_from_history)
    }

    #[tracing::instrument(name = "repository.athlete.find_by_id", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn find_by_id(&self, id: Uuid) -> Result<AthleteRow> {
        let athlete = sqlx::query_as!(
            AthleteRow,
            r#"
            SELECT athlete_id, first_name, last_name, native_name, gender as "gender: Gender", created_at,
                   country, profile_picture_url, slug,
                   COALESCE(slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
            FROM athletes
            WHERE athlete_id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?
        .ok_or(StorageError::NotFound)?;

        Ok(athlete)
    }

    #[tracing::instrument(name = "repository.athlete.find_by_slug_detailed", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn find_by_slug_detailed(&self, slug: &str) -> Result<AthleteDetail> {
        let athlete = self.find_by_slug(slug).await?;
        self.get_detailed_athlete(athlete).await
    }

    #[tracing::instrument(name = "repository.athlete.lift_balance", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn lift_balance(&self, athlete_id: Uuid) -> Result<Vec<AthleteBalanceRow>> {
        let rows = sqlx::query!(
            r#"
            WITH latest_category AS (
                SELECT wc.weight_class_id, a.gender, wc.min_kg, wc.max_kg
                FROM competition_participants cp
                JOIN athletes a ON a.athlete_id = cp.athlete_id
                JOIN competitions c ON c.competition_id = cp.competition_id
                JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
                WHERE cp.athlete_id = $1 AND cp.status = 'competed'
                ORDER BY c.start_date DESC, c.competition_id DESC, cp.participant_id DESC
                LIMIT 1
            ), complete AS (
                SELECT cp.participant_id, cp.athlete_id, c.name AS competition_name,
                       c.slug AS competition_slug, c.start_date, SUM(l.max_weight) AS total
                FROM competition_participants cp
                JOIN latest_category category ON category.weight_class_id = cp.weight_class_id
                JOIN athletes a ON a.athlete_id = cp.athlete_id AND a.gender = category.gender
                JOIN competitions c ON c.competition_id = cp.competition_id
                JOIN lifts l ON l.participant_id = cp.participant_id
                WHERE cp.status = 'competed'
                  AND l.movement_name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat')
                  AND l.max_weight > 0
                GROUP BY cp.participant_id, cp.athlete_id, c.name, c.slug, c.start_date
                HAVING COUNT(*) = 4
            ), best AS (
                SELECT DISTINCT ON (athlete_id) *
                FROM complete
                ORDER BY athlete_id, total DESC, start_date DESC
            ), shares AS (
                SELECT b.athlete_id, l.movement_name, l.max_weight AS value,
                       (100 * l.max_weight / b.total)::double precision AS share
                FROM best b
                JOIN lifts l ON l.participant_id = b.participant_id
                WHERE l.movement_name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat')
            ), peers AS (
                SELECT s.movement_name, COUNT(*) AS field, AVG(s.share) AS class_share,
                       STDDEV_SAMP(s.share) AS class_spread
                FROM shares s
                WHERE s.athlete_id <> $1
                GROUP BY s.movement_name
            )
            SELECT category.gender AS "category_gender: Gender",
                   category.min_kg AS weight_class_min, category.max_kg AS weight_class_max,
                   s.movement_name AS "movement_name!: Movement", s.value AS "value!",
                   s.share AS "share!", b.total AS "total!",
                   b.competition_name AS "competition_name!",
                   b.competition_slug AS "competition_slug!",
                   b.start_date AS "competition_date!",
                   COALESCE(p.field, 0) AS "field!",
                   p.class_share AS "class_share?", p.class_spread AS "class_spread?"
            FROM shares s
            JOIN best b ON b.athlete_id = s.athlete_id
            CROSS JOIN latest_category category
            LEFT JOIN peers p ON p.movement_name = s.movement_name
            JOIN movements m ON m.name = s.movement_name
            WHERE s.athlete_id = $1
            ORDER BY m.display_order
            "#,
            athlete_id
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| AthleteBalanceRow {
                category_gender: row.category_gender,
                weight_class_min: row.weight_class_min,
                weight_class_max: row.weight_class_max,
                movement_name: row.movement_name,
                value: row.value,
                share: row.share,
                total: row.total,
                competition_name: row.competition_name,
                competition_slug: row.competition_slug,
                competition_date: row.competition_date,
                field: row.field,
                class_share: row.class_share,
                class_spread: row.class_spread,
            })
            .collect())
    }

    #[expect(clippy::too_many_lines)]
    #[tracing::instrument(name = "repository.athlete.get_detailed_athlete", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn get_detailed_athlete(&self, athlete: AthleteRow) -> Result<AthleteDetail> {
        let rows = sqlx::query!(
            r#"
            SELECT
                c.competition_id,
                c.name as competition_name,
                c.slug as competition_slug,
                c.start_date as competition_date,
                d.name as "division?",
                COALESCE(cp.category_gender, wc.gender, a.gender) as "category_gender!: Gender",
                wc.min_kg as weight_class_min,
                wc.max_kg as weight_class_max,
                placed.rank as "rank?",
                cp.total as "total: Decimal",
                cp.ris_score,
                cp.ris_source as "ris_source: RisSource",
                cp.status as "status: AthleteStatus",
                c.event_code,
                COALESCE(
                    jsonb_agg(
                        jsonb_build_object(
                            'movement_name', l.movement_name,
                            'best_weight', l.max_weight::text,
                            'attempts', COALESCE(
                                (
                                    SELECT jsonb_agg(
                                               jsonb_build_object(
                                                   'attempt_number', a.attempt_number,
                                                   'weight', a.weight::text,
                                                   'is_successful', a.is_successful
                                               )
                                               ORDER BY a.attempt_number
                                           )
                                    FROM attempts a
                                    WHERE a.lift_id = l.lift_id
                                ),
                                '[]'::jsonb
                            )
                        )
                        ORDER BY m.display_order
                    ) FILTER (WHERE l.lift_id IS NOT NULL),
                    '[]'::jsonb
                ) as "lifts!: sqlx::types::Json<Vec<AthleteLiftRow>>"
            FROM competition_participants cp
            JOIN competitions c ON cp.competition_id = c.competition_id
            INNER JOIN athletes a ON a.athlete_id = cp.athlete_id
            LEFT JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
            LEFT JOIN divisions d ON d.division_id = cp.division_id
            LEFT JOIN lifts l ON l.participant_id = cp.participant_id
            LEFT JOIN movements m ON m.name = l.movement_name
            LEFT JOIN participant_standings placed ON placed.participant_id = cp.participant_id
            WHERE cp.athlete_id = $1
            GROUP BY c.competition_id, c.name, c.slug, c.start_date, c.event_code, d.name,
                     COALESCE(cp.category_gender, wc.gender, a.gender), wc.min_kg, wc.max_kg, placed.rank,
                     cp.ris_score, cp.ris_source, cp.status, cp.total
            ORDER BY c.start_date DESC NULLS LAST
            "#,
            athlete.athlete_id
        )
        .fetch_all(self.pool)
        .await?;

        let competitions: Vec<AthleteCompetitionRow> = rows
            .into_iter()
            .map(|row| AthleteCompetitionRow {
                competition_id: row.competition_id,
                competition_name: row.competition_name,
                competition_slug: row.competition_slug,
                competition_date: Some(row.competition_date),
                division: row.division,
                category_gender: row.category_gender,
                weight_class_min: row.weight_class_min,
                weight_class_max: row.weight_class_max,
                rank: row.rank,
                total: row.total,
                ris_score: row.ris_score,
                ris_source: row.ris_source,
                status: row.status,
                event_code: row.event_code,
                lifts: row.lifts.0,
            })
            .collect();

        let personal_records = Movement::ALL
            .into_iter()
            .filter_map(|movement| {
                competitions
                    .iter()
                    .filter(|competition| competition.status.competed())
                    .filter_map(|competition| {
                        let lift = competition
                            .lifts
                            .iter()
                            .find(|lift| lift.movement_name == movement)?;
                        Some((competition, lift.best_weight?))
                    })
                    .max_by_key(|(competition, weight)| (*weight, competition.competition_date))
                    .map(|(competition, max_weight)| PersonalRecordRow {
                        movement_name: movement,
                        max_weight,
                        competition_name: competition.competition_name.clone(),
                        competition_slug: competition.competition_slug.clone(),
                        date: competition.competition_date,
                    })
            })
            .collect();

        // Entering multiple divisions at one meet still counts as one competition.
        #[expect(clippy::cast_possible_wrap)]
        let total_competitions = competitions
            .iter()
            .map(|competition| competition.competition_id)
            .collect::<HashSet<_>>()
            .len() as i64;

        let instagram_handle = sqlx::query_scalar!(
            r#"
            SELECT ats.handle
            FROM athlete_socials ats
            JOIN socials s ON s.social_id = ats.social_id
            WHERE ats.athlete_id = $1
              AND s.name = 'instagram'
            "#,
            athlete.athlete_id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(AthleteDetail {
            athlete,
            competitions,
            personal_records,
            total_competitions,
            instagram_handle,
        })
    }
}
