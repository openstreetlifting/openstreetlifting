use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::collections::HashMap;
use uuid::Uuid;

use crate::error::{Result, StorageError};
use crate::params::{CompetitionFilter, Page};
use crate::projections::competition::{
    AttemptSummary, CategoryParticipants, CompetitionDetail, CompetitionListItem,
    CompetitionSummaryRow, Contest, LiftDetail, ParticipantDetail,
};
use crate::repository::parse_gender;
use crate::rows::{
    athlete::AthleteRow, competition::CompetitionRow, competition_movement::CompetitionMovementRow,
    federation::FederationRow, lift::LiftRow,
};
use osl_domain::Gender;

pub struct CompetitionRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CompetitionRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Every filter is applied here rather than on the page, so the total the
    /// pager divides counts the same rows it pages through.
    pub async fn list(
        &self,
        page: &Page,
        filter: &CompetitionFilter,
    ) -> Result<(Vec<CompetitionSummaryRow>, i64)> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT c.competition_id, c.name, c.created_at, c.slug, c.status,
                   c.federation_id, c.city, c.region, c.country,
                   c.start_date, c.end_date,
                   COUNT(p.participant_id) AS lifter_count
            FROM competitions c
            JOIN federations f ON f.federation_id = c.federation_id
            LEFT JOIN competition_participants p ON p.competition_id = c.competition_id
            "#,
        );

        Self::push_filters(&mut query, filter);

        query.push(" GROUP BY c.competition_id ORDER BY c.start_date ");
        query.push(filter.direction.as_sql());
        query.push(" NULLS LAST, c.created_at DESC LIMIT ");
        query.push_bind(page.limit);
        query.push(" OFFSET ");
        query.push_bind(page.offset);

        let competitions: Vec<CompetitionSummaryRow> =
            query.build_query_as().fetch_all(self.pool).await?;

        let mut counter = QueryBuilder::new(
            r#"
            SELECT COUNT(*)
            FROM competitions c
            JOIN federations f ON f.federation_id = c.federation_id
            "#,
        );

        Self::push_filters(&mut counter, filter);

        let total: i64 = counter.build_query_scalar().fetch_one(self.pool).await?;

        Ok((competitions, total))
    }

    /// Shared by the page and its total so the two can never disagree about
    /// which rows are in the list.
    fn push_filters(query: &mut QueryBuilder<Postgres>, filter: &CompetitionFilter) {
        query.push(" WHERE TRUE ");

        if let Some(status) = &filter.status {
            query.push(" AND c.status = ");
            query.push_bind(status.clone());
        }

        if let Some(federation) = &filter.federation {
            query.push(" AND f.name = ");
            query.push_bind(federation.clone());
        }

        if let Some(country) = &filter.country {
            query.push(" AND c.country = ");
            query.push_bind(country.clone());
        }

        if let Some(year) = filter.year {
            query.push(" AND EXTRACT(YEAR FROM c.start_date) = ");
            query.push_bind(f64::from(year));
        }

        if let Some(search) = &filter.search {
            let pattern = format!("%{search}%");
            query.push(" AND (c.name ILIKE ");
            query.push_bind(pattern.clone());
            query.push(" OR f.name ILIKE ");
            query.push_bind(pattern.clone());
            query.push(" OR c.city ILIKE ");
            query.push_bind(pattern);
            query.push(")");
        }
    }

    pub async fn list_with_details(
        &self,
        page: &Page,
        filter: &CompetitionFilter,
    ) -> Result<(Vec<CompetitionListItem>, i64)> {
        let (competitions, total) = self.list(page, filter).await?;
        let mut results = Vec::with_capacity(competitions.len());

        for summary in competitions {
            let CompetitionSummaryRow {
                competition,
                lifter_count,
            } = summary;
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
                "SELECT competition_id, movement_name, display_order
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
                lifter_count,
            });
        }

        Ok((results, total))
    }

    /// Federations that have run at least one competition, alphabetical, so the
    /// dropdown never offers a filter that returns nothing.
    pub async fn list_distinct_federations(&self) -> Result<Vec<String>> {
        let federations: Vec<String> = sqlx::query_scalar(
            r#"
            SELECT DISTINCT f.name
            FROM federations f
            JOIN competitions c ON c.federation_id = f.federation_id
            ORDER BY f.name
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(federations)
    }

    /// Years a competition was held in, most recent first.
    pub async fn list_distinct_years(&self) -> Result<Vec<i32>> {
        let years: Vec<i32> = sqlx::query_scalar(
            r#"
            SELECT DISTINCT EXTRACT(YEAR FROM start_date)::int as year
            FROM competitions
            WHERE start_date IS NOT NULL
            ORDER BY year DESC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(years)
    }

    /// Countries a competition was held in, alphabetical.
    pub async fn list_distinct_countries(&self) -> Result<Vec<String>> {
        let countries: Vec<String> = sqlx::query_scalar(
            r#"
            SELECT DISTINCT country
            FROM competitions
            WHERE country IS NOT NULL
            ORDER BY country
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(countries)
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
                    cp.weight_class_id,
                    cp.division_id,
                    cp.bodyweight,
                    COALESCE(SUM(l.max_weight), 0) as total
                FROM competition_participants cp
                LEFT JOIN lifts l ON l.participant_id = cp.participant_id
                WHERE cp.competition_id = $1
                  AND cp.status = 'competed'
                GROUP BY cp.participant_id, cp.weight_class_id, cp.division_id, cp.bodyweight
            )
            SELECT
                participant_id,
                ROW_NUMBER() OVER (
                    PARTITION BY weight_class_id, division_id
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

        let contests = sqlx::query!(
            r#"SELECT DISTINCT cp.weight_class_id, cp.division_id, d.name AS "division?", wc.gender,
                    wc.min_kg AS weight_class_min, wc.max_kg AS weight_class_max
             FROM competition_participants cp
             JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
             LEFT JOIN divisions d ON d.division_id = cp.division_id
             WHERE cp.competition_id = $1"#,
            competition.competition_id
        )
        .fetch_all(self.pool)
        .await?;

        let categories = contests
            .into_iter()
            .map(|row| {
                Ok(Contest {
                    weight_class_id: row.weight_class_id,
                    division_id: row.division_id,
                    division: row.division,
                    gender: parse_gender(&row.gender)?,
                    weight_class_min: row.weight_class_min,
                    weight_class_max: row.weight_class_max,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let mut category_details = Vec::with_capacity(categories.len());

        for category in categories {
            let participants = sqlx::query!(
                "SELECT participant_id, competition_id, athlete_id, bodyweight, status,
                        created_at, status_reason, ris_score, ris_source
                 FROM competition_participants
                 WHERE competition_id = $1
                   AND weight_class_id = $2
                   AND division_id IS NOT DISTINCT FROM $3",
                competition.competition_id,
                category.weight_class_id,
                category.division_id
            )
            .fetch_all(self.pool)
            .await?;

            let mut participant_details = Vec::with_capacity(participants.len());

            for participant in participants {
                let athlete = sqlx::query_as!(
                    AthleteRow,
                    r#"SELECT athlete_id, first_name, last_name, gender, created_at,
                            country, profile_picture_url, slug,
                            COALESCE(slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
                     FROM athletes
                     WHERE athlete_id = $1"#,
                    participant.athlete_id
                )
                .fetch_one(self.pool)
                .await?;

                let lifts = sqlx::query_as!(
                    LiftRow,
                    "SELECT lift_id, participant_id, movement_name, max_weight, updated_at
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
                        "SELECT attempt_number, weight, is_successful
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
                    ris_source: participant.ris_source.clone(),
                    status: participant.status.clone(),
                    status_reason: participant.status_reason.clone(),
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

        category_details
            .sort_by(|a, b| category_order(&a.category).cmp(&category_order(&b.category)));

        Ok(CompetitionDetail {
            competition,
            federation,
            categories: category_details,
        })
    }
}

fn category_order(category: &Contest) -> (Option<&str>, u8, Decimal, Decimal) {
    let gender = match category.gender {
        Gender::M => 0,
        Gender::F => 1,
        Gender::Mx => 2,
    };

    (
        category.division.as_deref(),
        gender,
        category.weight_class_max.unwrap_or(Decimal::MAX),
        category.weight_class_min.unwrap_or(Decimal::ZERO),
    )
}
