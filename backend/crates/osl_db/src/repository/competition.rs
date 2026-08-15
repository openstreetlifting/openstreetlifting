use rust_decimal::Decimal;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::error::{Result, StorageError};
use crate::params::Page;
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

    /// Returns one page of competitions plus the unpaginated total.
    pub async fn list(&self, page: &Page) -> Result<(Vec<CompetitionRow>, i64)> {
        let competitions = sqlx::query_as!(
            CompetitionRow,
            r#"
            SELECT competition_id, name, created_at, slug, status, federation_id,
                   city, region, country, start_date, end_date
            FROM competitions
            ORDER BY start_date DESC, created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            page.limit,
            page.offset
        )
        .fetch_all(self.pool)
        .await?;

        let total = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM competitions"#)
            .fetch_one(self.pool)
            .await?;

        Ok((competitions, total))
    }

    pub async fn list_with_details(&self, page: &Page) -> Result<(Vec<CompetitionListItem>, i64)> {
        let (competitions, total) = self.list(page).await?;
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

        Ok((results, total))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<CompetitionRow> {
        let competition = sqlx::query_as!(
            CompetitionRow,
            r#"
            SELECT competition_id, name, created_at, slug, status, federation_id,
                   city, region, country, start_date, end_date
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

    pub async fn find_by_slug(&self, slug: &str) -> Result<CompetitionRow> {
        let competition = sqlx::query_as!(
            CompetitionRow,
            r#"
            SELECT competition_id, name, created_at, slug, status, federation_id,
                   city, region, country, start_date, end_date
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
                  AND NOT cp.is_disqualified
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
            "SELECT DISTINCT c.category_id, c.name, c.gender,
                    wc.min_kg AS weight_class_min, wc.max_kg AS weight_class_max
             FROM categories c
             JOIN competition_participants cp ON c.category_id = cp.category_id
             JOIN weight_classes wc ON wc.weight_class_id = c.weight_class_id
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

                    total += lift.max_weight.unwrap_or(Decimal::ZERO);

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

                let rank = ranking_map.get(&participant.participant_id).copied();

                participant_details.push(ParticipantDetail {
                    athlete,
                    bodyweight: participant.bodyweight,
                    rank,
                    ris_score: participant.ris_score,
                    is_disqualified: participant.is_disqualified,
                    disqualified_reason: participant.disqualified_reason.clone(),
                    total: (!lift_details.is_empty()).then_some(total),
                    lifts: lift_details,
                });
            }

            participant_details.sort_by_key(|p| (p.rank.is_none(), p.rank));

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
}
