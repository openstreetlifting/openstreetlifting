use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Result, StorageError};
use crate::params::{AthleteUpdate, NewAthlete};
use crate::projections::athlete::{AthleteCompetitionRow, AthleteDetail, PersonalRecordRow};
use crate::rows::athlete::AthleteRow;

pub struct AthleteRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AthleteRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<AthleteRow>> {
        let athletes = sqlx::query_as!(
            AthleteRow,
            r#"
            SELECT athlete_id, first_name, last_name, gender, created_at,
                   nationality, country, profile_picture_url, slug,
                   COALESCE(slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
            FROM athletes
            ORDER BY last_name, first_name
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(athletes)
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<AthleteRow> {
        let athlete = sqlx::query_as!(
            AthleteRow,
            r#"
            SELECT athlete_id, first_name, last_name, gender, created_at,
                   nationality, country, profile_picture_url, slug,
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
                   nationality, country, profile_picture_url, slug,
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
                   nationality, country, profile_picture_url, slug,
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
                COALESCE(SUM(l.max_weight), 0) as "total!: Decimal",
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
                l.max_weight,
                c.name as competition_name,
                c.slug as competition_slug,
                c.start_date as date
            FROM lifts l
            JOIN competition_participants cp ON l.participant_id = cp.participant_id
            JOIN competitions c ON cp.competition_id = c.competition_id
            WHERE cp.athlete_id = $1
            ORDER BY l.movement_name, l.max_weight DESC
            "#,
            athlete.athlete_id
        )
        .fetch_all(self.pool)
        .await?;

        // Count total competitions
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

        Ok(AthleteDetail {
            athlete,
            competitions,
            personal_records,
            total_competitions,
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

    pub async fn create(&self, new: &NewAthlete) -> Result<AthleteRow> {
        let slug = self
            .generate_unique_slug(&new.first_name, &new.last_name)
            .await?;

        let athlete = sqlx::query_as!(
            AthleteRow,
            r#"
            INSERT INTO athletes (first_name, last_name, gender, nationality, country, profile_picture_url, slug)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING athlete_id, first_name, last_name, gender, created_at,
                      nationality, country, profile_picture_url, slug,
                      COALESCE(slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
            "#,
            new.first_name,
            new.last_name,
            new.gender,
            new.nationality,
            new.country,
            new.profile_picture_url,
            slug
        )
        .fetch_one(self.pool)
        .await?;

        Ok(athlete)
    }

    pub async fn update(
        &self,
        id: Uuid,
        existing: &AthleteRow,
        update: &AthleteUpdate,
    ) -> Result<AthleteRow> {
        let first_name = update.first_name.as_ref().unwrap_or(&existing.first_name);
        let last_name = update.last_name.as_ref().unwrap_or(&existing.last_name);
        let gender = update.gender.as_ref().unwrap_or(&existing.gender);
        let nationality = update.nationality.as_ref().or(existing.nationality.as_ref());
        let country = update.country.as_ref().unwrap_or(&existing.country);
        let profile_picture_url = update
            .profile_picture_url
            .as_ref()
            .or(existing.profile_picture_url.as_ref());

        let (slug, slug_history) = if update.first_name.is_some() || update.last_name.is_some() {
            let new_slug = self.generate_unique_slug(first_name, last_name).await?;
            let mut history = existing.slug_history.0.clone();
            history.push(existing.slug.clone());
            (new_slug, sqlx::types::Json(history))
        } else {
            (existing.slug.clone(), existing.slug_history.clone())
        };

        let athlete = sqlx::query_as!(
            AthleteRow,
            r#"
            UPDATE athletes
            SET first_name = $2,
                last_name = $3,
                gender = $4,
                nationality = $5,
                country = $6,
                profile_picture_url = $7,
                slug = $8,
                slug_history = $9
            WHERE athlete_id = $1
            RETURNING athlete_id, first_name, last_name, gender, created_at,
                      nationality, country, profile_picture_url, slug,
                      COALESCE(slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
            "#,
            id,
            first_name,
            last_name,
            gender,
            nationality,
            country,
            profile_picture_url,
            slug,
            slug_history as _
        )
        .fetch_optional(self.pool)
        .await?
        .ok_or(StorageError::NotFound)?;

        Ok(athlete)
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query!("DELETE FROM athletes WHERE athlete_id = $1", id)
            .execute(self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }

        Ok(())
    }
}
