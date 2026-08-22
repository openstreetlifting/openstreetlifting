use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Result, StorageError};
use crate::params::Page;
use crate::projections::athlete::{AthleteCompetitionRow, AthleteDetail, PersonalRecordRow};
use crate::rows::athlete::AthleteRow;

pub struct AthleteRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AthleteRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Returns one page of athletes plus the unpaginated total.
    pub async fn list(&self, page: &Page) -> Result<(Vec<AthleteRow>, i64)> {
        let athletes = sqlx::query_as!(
            AthleteRow,
            r#"
            SELECT athlete_id, first_name, last_name, gender, created_at,
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
            SELECT athlete_id, first_name, last_name, gender, created_at,
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
            SELECT athlete_id, first_name, last_name, gender, created_at,
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
            SELECT athlete_id, first_name, last_name, gender, created_at,
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

    async fn get_detailed_athlete(&self, athlete: AthleteRow) -> Result<AthleteDetail> {
        let competitions = sqlx::query_as!(
            AthleteCompetitionRow,
            r#"
            SELECT
                c.competition_id,
                c.name as competition_name,
                c.slug as competition_slug,
                c.start_date as competition_date,
                cat.name as category_name,
                cp.rank,
                CASE WHEN COUNT(l.lift_id) = 0 THEN NULL
                     ELSE COALESCE(SUM(l.max_weight), 0)
                END as "total: Decimal",
                cp.ris_score,
                cp.is_disqualified
            FROM competition_participants cp
            JOIN competitions c ON cp.competition_id = c.competition_id
            JOIN categories cat ON cp.category_id = cat.category_id
            LEFT JOIN lifts l ON l.participant_id = cp.participant_id
            WHERE cp.athlete_id = $1
            GROUP BY c.competition_id, c.name, c.slug, c.start_date, cat.name, cp.rank, cp.ris_score, cp.is_disqualified
            ORDER BY c.start_date DESC NULLS LAST
            "#,
            athlete.athlete_id
        )
        .fetch_all(self.pool)
        .await?;

        let personal_records = sqlx::query_as!(
            PersonalRecordRow,
            r#"
            SELECT DISTINCT ON (l.movement_name)
                l.movement_name,
                -- Not null thanks to the filter below, which sqlx cannot infer.
                l.max_weight as "max_weight!",
                c.name as competition_name,
                c.slug as competition_slug,
                c.start_date as date
            FROM lifts l
            JOIN competition_participants cp ON l.participant_id = cp.participant_id
            JOIN competitions c ON cp.competition_id = c.competition_id
            WHERE cp.athlete_id = $1
              -- A movement where every attempt failed is not a record, and
              -- DESC would otherwise sort its NULL to the front and pick it.
              AND l.max_weight IS NOT NULL
              AND NOT cp.is_disqualified
            ORDER BY l.movement_name, l.max_weight DESC
            "#,
            athlete.athlete_id
        )
        .fetch_all(self.pool)
        .await?;

        let total_competitions = sqlx::query_scalar!(
            r#"
            SELECT COUNT(DISTINCT cp.competition_id)::bigint as "count!"
            FROM competition_participants cp
            WHERE cp.athlete_id = $1
            "#,
            athlete.athlete_id
        )
        .fetch_one(self.pool)
        .await?;

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
