use rust_decimal::Decimal;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::error::{Result, StorageError};
use crate::params::{CompetitionUpdate, NewCompetition};
use crate::projections::competition::{
    AttemptSummary, CategoryParticipants, CompetitionDetail, CompetitionListItem, LiftDetail,
    ParticipantDetail,
};
use crate::rows::{
    athlete::AthleteRow, category::CategoryRow, competition::CompetitionRow,
    competition_movement::CompetitionMovementRow, federation::FederationRow, lift::LiftRow,
};

pub struct CompetitionRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CompetitionRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<CompetitionRow>> {
        let competitions = sqlx::query_as!(
            CompetitionRow,
            r#"
            SELECT competition_id, name, created_at, slug, status, federation_id,
                   venue, city, country, start_date, end_date, number_of_judge
            FROM competitions
            ORDER BY start_date DESC, created_at DESC
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(competitions)
    }

    pub async fn list_with_details(&self) -> Result<Vec<CompetitionListItem>> {
        let competitions = self.list().await?;
        let mut results = Vec::with_capacity(competitions.len());

        for competition in competitions {
            let federation = sqlx::query_as!(
                FederationRow,
                "SELECT federation_id, name, rulebook_id, country, abbreviation
                 FROM federations
                 WHERE federation_id = $1",
                competition.federation_id
            )
            .fetch_one(self.pool)
            .await?;

            let movements = sqlx::query_as!(
                CompetitionMovementRow,
                "SELECT competition_id, movement_name, is_required, display_order
                 FROM competition_movements
                 WHERE competition_id = $1
                 ORDER BY display_order",
                competition.competition_id
            )
            .fetch_all(self.pool)
            .await?;

            results.push(CompetitionListItem {
                competition,
                federation,
                movements,
            });
        }

        Ok(results)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<CompetitionRow> {
        let competition = sqlx::query_as!(
            CompetitionRow,
            r#"
            SELECT competition_id, name, created_at, slug, status, federation_id,
                   venue, city, country, start_date, end_date, number_of_judge
            FROM competitions
            WHERE competition_id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?
        .ok_or(StorageError::NotFound)?;

        Ok(competition)
    }

    /// Get a competition by slug
    pub async fn find_by_slug(&self, slug: &str) -> Result<CompetitionRow> {
        let competition = sqlx::query_as!(
            CompetitionRow,
            r#"
            SELECT competition_id, name, created_at, slug, status, federation_id,
                   venue, city, country, start_date, end_date, number_of_judge
            FROM competitions
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(self.pool)
        .await?
        .ok_or(StorageError::NotFound)?;

        Ok(competition)
    }

    pub async fn find_by_slug_detailed(&self, slug: &str) -> Result<CompetitionDetail> {
        let competition = self.find_by_slug(slug).await?;
        self.get_detailed_competition(competition).await
    }

    pub async fn find_by_id_detailed(&self, id: Uuid) -> Result<CompetitionDetail> {
        let competition = self.find_by_id(id).await?;
        self.get_detailed_competition(competition).await
    }

    /// Compute category rankings for all participants in a competition
    async fn compute_category_rankings(&self, competition_id: Uuid) -> Result<HashMap<Uuid, i32>> {
        let rankings = sqlx::query!(
            r#"
            WITH participant_totals AS (
                SELECT
                    cp.participant_id,
                    cp.category_id,
                    cp.bodyweight,
                    COALESCE(SUM(l.max_weight), 0) as total
                FROM competition_participants cp
                LEFT JOIN lifts l ON l.participant_id = cp.participant_id
                WHERE cp.competition_id = $1
                GROUP BY cp.participant_id, cp.category_id, cp.bodyweight
            )
            SELECT
                participant_id,
                ROW_NUMBER() OVER (
                    PARTITION BY category_id
                    ORDER BY
                        CASE WHEN total = 0 THEN 1 ELSE 0 END,
                        total DESC,
                        bodyweight ASC NULLS LAST
                )::int as "rank!"
            FROM participant_totals
            "#,
            competition_id
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rankings
            .into_iter()
            .map(|r| (r.participant_id, r.rank))
            .collect())
    }

    async fn get_detailed_competition(
        &self,
        competition: CompetitionRow,
    ) -> Result<CompetitionDetail> {
        // Compute category rankings for all participants
        let ranking_map = self
            .compute_category_rankings(competition.competition_id)
            .await?;

        let federation = sqlx::query_as!(
            FederationRow,
            "SELECT federation_id, name, rulebook_id, country, abbreviation
             FROM federations
             WHERE federation_id = $1",
            competition.federation_id
        )
        .fetch_one(self.pool)
        .await?;

        let categories = sqlx::query_as!(
            CategoryRow,
            "SELECT DISTINCT c.category_id, c.name, c.gender, c.weight_class_min, c.weight_class_max
             FROM categories c
             JOIN competition_participants cp ON c.category_id = cp.category_id
             WHERE cp.competition_id = $1",
            competition.competition_id
        )
        .fetch_all(self.pool)
        .await?;

        let mut category_details = Vec::with_capacity(categories.len());

        for category in categories {
            let participants = sqlx::query!(
                "SELECT participant_id, competition_id, category_id, athlete_id, bodyweight, rank, is_disqualified,
                        created_at, disqualified_reason, ris_score
                 FROM competition_participants
                 WHERE competition_id = $1 AND category_id = $2
                 ORDER BY rank NULLS LAST",
                competition.competition_id,
                category.category_id
            )
            .fetch_all(self.pool)
            .await?;

            let mut participant_details = Vec::with_capacity(participants.len());

            for participant in participants {
                let athlete = sqlx::query_as!(
                    AthleteRow,
                    r#"SELECT athlete_id, first_name, last_name, gender, created_at,
                            nationality, country, profile_picture_url, slug,
                            COALESCE(slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
                     FROM athletes
                     WHERE athlete_id = $1"#,
                    participant.athlete_id
                )
                .fetch_one(self.pool)
                .await?;

                let lifts = sqlx::query_as!(
                    LiftRow,
                    "SELECT lift_id, participant_id, movement_name, max_weight,
                            equipment_setting, updated_at
                     FROM lifts
                     WHERE participant_id = $1",
                    participant.participant_id
                )
                .fetch_all(self.pool)
                .await?;

                let mut lift_details = Vec::with_capacity(lifts.len());
                let mut total = Decimal::ZERO;

                for lift in lifts {
                    let attempts = sqlx::query!(
                        "SELECT attempt_number, weight, is_successful, passing_judges, no_rep_reason
                         FROM attempts
                         WHERE lift_id = $1
                         ORDER BY attempt_number",
                        lift.lift_id
                    )
                    .fetch_all(self.pool)
                    .await?;

                    total += lift.max_weight;

                    lift_details.push(LiftDetail {
                        movement_name: lift.movement_name.clone(),
                        best_weight: lift.max_weight,
                        attempts: attempts
                            .into_iter()
                            .map(|a| AttemptSummary {
                                attempt_number: a.attempt_number,
                                weight: a.weight,
                                is_successful: a.is_successful,
                                passing_judges: a.passing_judges,
                                no_rep_reason: a.no_rep_reason,
                            })
                            .collect(),
                    });
                }

                // Get computed category rank for this participant
                let rank = ranking_map.get(&participant.participant_id).copied();

                participant_details.push(ParticipantDetail {
                    athlete,
                    bodyweight: participant.bodyweight,
                    rank,
                    ris_score: participant.ris_score,
                    is_disqualified: participant.is_disqualified,
                    disqualified_reason: participant.disqualified_reason.clone(),
                    lifts: lift_details,
                    total,
                });
            }

            category_details.push(CategoryParticipants {
                category,
                participants: participant_details,
            });
        }

        category_details.sort_by(|a, b| a.category.name.cmp(&b.category.name));

        Ok(CompetitionDetail {
            competition,
            federation,
            categories: category_details,
        })
    }

    pub async fn create(&self, new: &NewCompetition) -> Result<CompetitionRow> {
        let competition = sqlx::query_as!(
            CompetitionRow,
            r#"
            INSERT INTO competitions (
                name, slug, status, federation_id, venue, city, country,
                start_date, end_date, number_of_judge
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING competition_id, name, created_at, slug, status, federation_id,
                      venue, city, country, start_date, end_date, number_of_judge
            "#,
            new.name,
            new.slug,
            new.status,
            new.federation_id,
            new.venue,
            new.city,
            new.country,
            new.start_date,
            new.end_date,
            new.number_of_judge
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| {
            // Handle unique constraint violations for slug
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code().as_deref() == Some("23505")
            {
                return StorageError::ConstraintViolation("Slug already exists".to_string());
            }
            StorageError::from(e)
        })?;

        Ok(competition)
    }

    pub async fn update(
        &self,
        id: Uuid,
        existing: &CompetitionRow,
        update: &CompetitionUpdate,
    ) -> Result<CompetitionRow> {
        let name = update.name.as_ref().unwrap_or(&existing.name);
        let slug = update.slug.as_ref().unwrap_or(&existing.slug);
        let status = update.status.as_ref().unwrap_or(&existing.status);
        let federation_id = update.federation_id.unwrap_or(existing.federation_id);
        let venue = update.venue.as_ref().or(existing.venue.as_ref());
        let city = update.city.as_ref().or(existing.city.as_ref());
        let country = update.country.as_ref().or(existing.country.as_ref());
        let start_date = update.start_date.or(existing.start_date);
        let end_date = update.end_date.or(existing.end_date);
        let number_of_judge = update.number_of_judge.or(existing.number_of_judge);

        let competition = sqlx::query_as!(
            CompetitionRow,
            r#"
            UPDATE competitions
            SET
                name = $2,
                slug = $3,
                status = $4,
                federation_id = $5,
                venue = $6,
                city = $7,
                country = $8,
                start_date = $9,
                end_date = $10,
                number_of_judge = $11
            WHERE competition_id = $1
            RETURNING competition_id, name, created_at, slug, status, federation_id,
                      venue, city, country, start_date, end_date, number_of_judge
            "#,
            id,
            name,
            slug,
            status,
            federation_id,
            venue,
            city,
            country,
            start_date,
            end_date,
            number_of_judge
        )
        .fetch_optional(self.pool)
        .await?
        .ok_or(StorageError::NotFound)?;

        Ok(competition)
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM competitions
            WHERE competition_id = $1
            "#,
            id
        )
        .execute(self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }

        Ok(())
    }
}
