use osl_domain::{AthleteStatus, Gender, Movement, RisSource};
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

use crate::error::{Result, StorageError};
use crate::params::Page;
use crate::projections::athlete::{
    AthleteCompetitionRow, AthleteDetail, AthleteLiftRow, AthleteStrengthRow, PersonalRecordRow,
};
use crate::rows::athlete::AthleteRow;

pub struct AthleteRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AthleteRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

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

    pub async fn find_by_slug_detailed(&self, slug: &str) -> Result<AthleteDetail> {
        let athlete = self.find_by_slug(slug).await?;
        self.get_detailed_athlete(athlete).await
    }

    pub async fn strength_profile(&self, athlete_id: Uuid) -> Result<Vec<AthleteStrengthRow>> {
        let rows = sqlx::query!(
            r#"
            WITH latest_category AS (
                SELECT wc.weight_class_id, wc.gender, wc.min_kg, wc.max_kg
                FROM competition_participants cp
                JOIN competitions c ON c.competition_id = cp.competition_id
                JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
                WHERE cp.athlete_id = $1 AND cp.status = 'competed'
                ORDER BY c.start_date DESC, c.competition_id DESC, cp.participant_id DESC
                LIMIT 1
            ), eligible AS (
                SELECT cp.participant_id, cp.athlete_id
                FROM competition_participants cp
                JOIN latest_category category ON category.weight_class_id = cp.weight_class_id
                JOIN athletes a ON a.athlete_id = cp.athlete_id AND a.gender = category.gender
                WHERE cp.status = 'competed'
            ), bests AS (
                SELECT cp.athlete_id, l.movement_name, MAX(l.max_weight) AS value
                FROM eligible cp
                JOIN lifts l ON l.participant_id = cp.participant_id
                WHERE l.max_weight IS NOT NULL
                GROUP BY cp.athlete_id, l.movement_name
            ), standings AS (
                SELECT m.name AS movement_name, target.value,
                       COUNT(other.athlete_id) AS field,
                       (100.0 * AVG(CASE
                           WHEN target.value IS NULL THEN NULL
                           WHEN other.value < target.value THEN 1.0
                           WHEN other.value = target.value THEN 0.5
                           ELSE 0.0 END) FILTER (WHERE other.athlete_id IS NOT NULL))::double precision AS percentile
                FROM movements m
                LEFT JOIN bests target ON target.athlete_id = $1 AND target.movement_name = m.name
                LEFT JOIN bests other ON other.athlete_id <> $1 AND other.movement_name = m.name
                GROUP BY m.name, target.value
            )
            SELECT category.gender AS "category_gender: Gender",
                   category.min_kg AS weight_class_min, category.max_kg AS weight_class_max,
                   m.name AS "movement_name: Movement", standing.value AS "value?",
                   standing.field AS "field!", standing.percentile AS "percentile?"
            FROM latest_category category
            CROSS JOIN movements m
            JOIN standings standing ON standing.movement_name = m.name
            WHERE m.name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat')
            ORDER BY m.display_order
            "#,
            athlete_id
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| AthleteStrengthRow {
                category_gender: row.category_gender,
                weight_class_min: row.weight_class_min,
                weight_class_max: row.weight_class_max,
                movement_name: row.movement_name,
                value: row.value,
                percentile: row.percentile,
                field: row.field,
            })
            .collect())
    }

    pub async fn get_detailed_athlete(&self, athlete: AthleteRow) -> Result<AthleteDetail> {
        let rows = sqlx::query!(
            r#"
            WITH entered AS (
                SELECT DISTINCT competition_id, weight_class_id, division_id
                FROM competition_participants
                WHERE athlete_id = $1
            ),
            placed AS (
                SELECT
                    cp.participant_id,
                    ROW_NUMBER() OVER (
                        PARTITION BY cp.competition_id, cp.weight_class_id, cp.division_id
                        ORDER BY
                            CASE WHEN COALESCE(SUM(l.max_weight), 0) = 0 THEN 1 ELSE 0 END,
                            COALESCE(SUM(l.max_weight), 0) DESC,
                            cp.bodyweight ASC NULLS LAST
                    )::int as place
                FROM competition_participants cp
                JOIN entered e
                    ON e.competition_id = cp.competition_id
                   AND e.weight_class_id = cp.weight_class_id
                   AND e.division_id IS NOT DISTINCT FROM cp.division_id
                LEFT JOIN lifts l ON l.participant_id = cp.participant_id
                WHERE cp.status = 'competed'
                GROUP BY cp.participant_id, cp.competition_id, cp.weight_class_id,
                         cp.division_id, cp.bodyweight
            )
            SELECT
                c.competition_id,
                c.name as competition_name,
                c.slug as competition_slug,
                c.start_date as competition_date,
                d.name as "division?",
                wc.gender as "category_gender: Gender",
                wc.min_kg as weight_class_min,
                wc.max_kg as weight_class_max,
                placed.place as "rank?",
                CASE WHEN COUNT(l.lift_id) = 0 THEN NULL
                     ELSE COALESCE(SUM(l.max_weight), 0)
                END as "total: Decimal",
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
            JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
            LEFT JOIN divisions d ON d.division_id = cp.division_id
            LEFT JOIN lifts l ON l.participant_id = cp.participant_id
            LEFT JOIN movements m ON m.name = l.movement_name
            LEFT JOIN placed ON placed.participant_id = cp.participant_id
            WHERE cp.athlete_id = $1
            GROUP BY c.competition_id, c.name, c.slug, c.start_date, c.event_code, d.name,
                     wc.gender, wc.min_kg, wc.max_kg, placed.place, cp.ris_score,
                     cp.ris_source, cp.status
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

    pub async fn generate_unique_slug(&self, first_name: &str, last_name: &str) -> Result<String> {
        let base_slug = format!("{}-{}", first_name, last_name)
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-");

        let base_slug = if base_slug.is_empty() {
            "athlete".to_string()
        } else {
            base_slug
        };

        let mut final_slug = base_slug.clone();
        let mut counter = 2;

        while sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM athletes WHERE slug = $1)",
            final_slug
        )
        .fetch_one(self.pool)
        .await?
        .unwrap_or(false)
        {
            final_slug = format!("{}-{}", base_slug, counter);
            counter += 1;
        }

        Ok(final_slug)
    }
}
