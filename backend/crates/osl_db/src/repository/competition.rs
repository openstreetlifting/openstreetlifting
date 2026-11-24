use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::collections::HashMap;
use uuid::Uuid;

use crate::error::{Result, StorageError};
use crate::filter::{Filters, Predicate};
use crate::params::{CompetitionBy, CompetitionFilter, Page};
use crate::projections::competition::{
    AttemptSummary, CategoryParticipants, CompetitionDetail, CompetitionEdition, CompetitionFacets,
    CompetitionSummaryRow, Contest, LiftDetail, ParticipantDetail,
};
use crate::rows::{
    athlete::AthleteRow, competition::CompetitionRow, competition_movement::CompetitionMovementRow,
    federation::FederationRow,
};
use osl_domain::{AthleteStatus, CompetitionStatus, Gender, Movement, RecordMark, RisSource};

impl Predicate for CompetitionBy {
    fn push(&self, query: &mut QueryBuilder<Postgres>) {
        match self {
            Self::Status(status) => query.push("c.status = ").push_bind(status.as_str()),
            Self::Event(event) => query.push("c.event_code = ").push_bind(event.clone()),
            Self::Federation(federation) => query.push("f.name = ").push_bind(federation.clone()),
            Self::Country(country) => query.push("c.country = ").push_bind(country.clone()),
            Self::Year(year) => query
                .push("EXTRACT(YEAR FROM c.start_date)::int = ")
                .push_bind(*year),
        };
    }
}

pub struct CompetitionRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CompetitionRepository<'a> {
    #[must_use]
    pub const fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "repository.competition.editions", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn editions(&self, slug: &str) -> Result<Vec<CompetitionEdition>> {
        Ok(sqlx::query_as(
            "SELECT edition.slug, edition.name, edition.start_date
             FROM competitions current
             JOIN competitions edition ON edition.series = current.series
             WHERE current.slug = $1
               AND (edition.competition_id = current.competition_id
                    OR edition.status IN ('completed', 'live', 'upcoming'))
             ORDER BY edition.start_date, edition.slug",
        )
        .bind(slug)
        .fetch_all(self.pool)
        .await?)
    }

    #[tracing::instrument(name = "repository.competition.list", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn list(
        &self,
        page: &Page,
        filter: &CompetitionFilter,
    ) -> Result<(Vec<CompetitionSummaryRow>, i64)> {
        let mut query = QueryBuilder::new(
            r"
            SELECT c.competition_id, c.name, c.created_at, c.slug, c.status, c.scoring,
                   c.federation_id, c.venue, c.city, c.region, c.country,
                   c.start_date, c.end_date,
                   COUNT(p.participant_id) AS lifter_count
            FROM competitions c
            JOIN federations f ON f.federation_id = c.federation_id
            LEFT JOIN competition_participants p ON p.competition_id = c.competition_id
            ",
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
            r"
            SELECT COUNT(*)
            FROM competitions c
            JOIN federations f ON f.federation_id = c.federation_id
            ",
        );

        Self::push_filters(&mut counter, filter);

        let total: i64 = counter.build_query_scalar().fetch_one(self.pool).await?;

        Ok((competitions, total))
    }

    fn push_filters(query: &mut QueryBuilder<Postgres>, filter: &CompetitionFilter) {
        query.push(" WHERE TRUE ");
        filter.by.push_and(query);

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

    #[tracing::instrument(name = "repository.competition.facets", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn facets(&self, by: &Filters<CompetitionBy>) -> Result<CompetitionFacets> {
        let mut federations = Self::listed(
            "SELECT DISTINCT f.name",
            &by.without(|by| matches!(by, CompetitionBy::Federation(_))),
        );
        federations.push(" ORDER BY 1 ");

        let mut countries = Self::listed(
            "SELECT DISTINCT c.country",
            &by.without(|by| matches!(by, CompetitionBy::Country(_))),
        );
        countries.push(" AND c.country IS NOT NULL ORDER BY 1 ");

        let mut years = Self::listed(
            "SELECT DISTINCT EXTRACT(YEAR FROM c.start_date)::int",
            &by.without(|by| matches!(by, CompetitionBy::Year(_))),
        );
        years.push(" AND c.start_date IS NOT NULL ORDER BY 1 DESC ");

        let mut formats = Self::listed(
            "SELECT DISTINCT c.event_code",
            &by.without(|by| matches!(by, CompetitionBy::Event(_))),
        );
        formats.push(" AND c.event_code IS NOT NULL ORDER BY 1 ");

        Ok(CompetitionFacets {
            federations: federations
                .build_query_scalar()
                .fetch_all(self.pool)
                .await?,
            countries: countries.build_query_scalar().fetch_all(self.pool).await?,
            years: years.build_query_scalar().fetch_all(self.pool).await?,
            formats: formats.build_query_scalar().fetch_all(self.pool).await?,
        })
    }

    fn listed(select: &str, by: &Filters<CompetitionBy>) -> QueryBuilder<Postgres> {
        let mut query = QueryBuilder::new(select);
        query.push(
            " FROM competitions c JOIN federations f ON f.federation_id = c.federation_id WHERE TRUE ",
        );
        by.push_and(&mut query);
        query
    }

    #[tracing::instrument(name = "repository.competition.find_by_id", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn find_by_id(&self, id: Uuid) -> Result<CompetitionRow> {
        let competition = sqlx::query_as!(
            CompetitionRow,
            r#"
            SELECT competition_id, name, created_at, slug,
                   status as "status: CompetitionStatus", scoring as "scoring: osl_domain::Scoring", federation_id,
                   city, venue, region, country, start_date, end_date
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

    #[tracing::instrument(name = "repository.competition.find_by_slug", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn find_by_slug(&self, slug: &str) -> Result<CompetitionRow> {
        let competition = sqlx::query_as!(
            CompetitionRow,
            r#"
            SELECT competition_id, name, created_at, slug,
                   status as "status: CompetitionStatus", scoring as "scoring: osl_domain::Scoring", federation_id,
                   city, venue, region, country, start_date, end_date
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

    #[tracing::instrument(name = "repository.competition.find_by_slug_detailed", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn find_by_slug_detailed(&self, slug: &str) -> Result<CompetitionDetail> {
        let competition = self.find_by_slug(slug).await?;
        self.get_detailed_competition(competition).await
    }

    #[tracing::instrument(name = "repository.competition.find_by_id_detailed", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn find_by_id_detailed(&self, id: Uuid) -> Result<CompetitionDetail> {
        let competition = self.find_by_id(id).await?;
        self.get_detailed_competition(competition).await
    }

    async fn get_detailed_competition(
        &self,
        competition: CompetitionRow,
    ) -> Result<CompetitionDetail> {
        let federation = self.federation(competition.federation_id).await?;
        let movements = self.movements(competition.competition_id).await?;
        let categories = self.results(competition.competition_id).await?;

        Ok(CompetitionDetail {
            competition,
            federation,
            movements,
            categories,
        })
    }

    #[tracing::instrument(name = "repository.competition.federations", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn federations(&self, federation_ids: &[Uuid]) -> Result<Vec<FederationRow>> {
        Ok(sqlx::query_as(
            "SELECT federation_id, name, rulebook_id, country, abbreviation
             FROM federations
             WHERE federation_id = ANY($1)",
        )
        .bind(federation_ids)
        .fetch_all(self.pool)
        .await?)
    }

    #[tracing::instrument(name = "repository.competition.competition_movements", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn competition_movements(
        &self,
        competition_ids: &[Uuid],
    ) -> Result<Vec<CompetitionMovementRow>> {
        Ok(sqlx::query_as(
            "SELECT competition_id, movement_name, display_order
             FROM competition_movements
             WHERE competition_id = ANY($1)
             ORDER BY competition_id, display_order",
        )
        .bind(competition_ids)
        .fetch_all(self.pool)
        .await?)
    }

    #[tracing::instrument(name = "repository.competition.federation", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn federation(&self, federation_id: Uuid) -> Result<FederationRow> {
        Ok(sqlx::query_as!(
            FederationRow,
            "SELECT federation_id, name, rulebook_id, country, abbreviation
             FROM federations
             WHERE federation_id = $1",
            federation_id
        )
        .fetch_one(self.pool)
        .await?)
    }

    #[tracing::instrument(name = "repository.competition.movements", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn movements(&self, competition_id: Uuid) -> Result<Vec<CompetitionMovementRow>> {
        Ok(sqlx::query_as!(
            CompetitionMovementRow,
            r#"SELECT competition_id, movement_name as "movement_name: Movement",
                    display_order
             FROM competition_movements
             WHERE competition_id = $1
             ORDER BY display_order"#,
            competition_id
        )
        .fetch_all(self.pool)
        .await?)
    }

    #[expect(clippy::too_many_lines)]
    #[tracing::instrument(name = "repository.competition.results", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn results(&self, competition_id: Uuid) -> Result<Vec<CategoryParticipants>> {
        let contests = sqlx::query!(
            r#"SELECT DISTINCT cp.weight_class_id, cp.division_id, d.name AS "division?",
                    COALESCE(cp.category_gender, wc.gender, a.gender) as "gender!: Gender",
                    wc.min_kg AS weight_class_min, wc.max_kg AS weight_class_max,
                    COALESCE(cp.scoring, c.scoring,
                             CASE WHEN cp.weight_class_id IS NULL THEN 'ris' ELSE 'total' END)
                        AS "scoring!: osl_domain::Scoring"
             FROM competition_participants cp
             INNER JOIN competitions c ON c.competition_id = cp.competition_id
             INNER JOIN athletes a ON a.athlete_id = cp.athlete_id
             LEFT JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
             LEFT JOIN divisions d ON d.division_id = cp.division_id
             WHERE cp.competition_id = $1"#,
            competition_id
        )
        .fetch_all(self.pool)
        .await?;

        let categories = contests.into_iter().map(|row| Contest {
            weight_class_id: row.weight_class_id,
            division_id: row.division_id,
            division: row.division,
            gender: row.gender,
            weight_class_min: row.weight_class_min,
            weight_class_max: row.weight_class_max,
            scoring: row.scoring,
        });

        let participants = sqlx::query!(
            r#"SELECT cp.participant_id, cp.weight_class_id, cp.division_id,
                    COALESCE(cp.category_gender, wc.gender, a.gender) as "category_gender!: Gender",
                    cp.bodyweight, cp.age, cp.total,
                    total_mark.mark as "total_record?: RecordMark",
                    cp.status as "status: AthleteStatus", cp.status_reason, placed.rank as "rank?",
                    cp.ris_score, cp.ris_source as "ris_source: RisSource",
                    a.athlete_id, a.first_name, a.last_name, a.native_name,
                    a.gender as "gender: Gender", a.created_at, a.country, a.profile_picture_url,
                    a.slug,
                    COALESCE(a.slug_history, '[]'::jsonb) as "slug_history!: sqlx::types::Json<Vec<String>>"
             FROM competition_participants cp
             INNER JOIN athletes a ON a.athlete_id = cp.athlete_id
             LEFT JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
             LEFT JOIN participant_standings placed ON placed.participant_id = cp.participant_id
                                                   AND placed.competition_id = cp.competition_id
             LEFT JOIN record_marks total_mark ON total_mark.participant_id = cp.participant_id
                                              AND total_mark.movement_name = 'Total'
             WHERE cp.competition_id = $1
             ORDER BY placed.rank NULLS LAST, cp.participant_id"#,
            competition_id
        )
        .fetch_all(self.pool)
        .await?;

        let participant_ids: Vec<Uuid> = participants
            .iter()
            .map(|participant| participant.participant_id)
            .collect();
        let lifts = sqlx::query!(
            r#"SELECT l.participant_id, l.lift_id,
                    l.movement_name as "movement_name: Movement",
                    l.max_weight,
                    rm.mark as "record?: RecordMark"
             FROM lifts l
             LEFT JOIN record_marks rm
                 ON rm.participant_id = l.participant_id
                AND rm.movement_name = l.movement_name
             LEFT JOIN movements m ON m.name = l.movement_name
             WHERE l.participant_id = ANY($1)
             ORDER BY l.participant_id, m.display_order"#,
            &participant_ids
        )
        .fetch_all(self.pool)
        .await?;

        let lift_ids: Vec<Uuid> = lifts.iter().map(|lift| lift.lift_id).collect();
        let attempts = sqlx::query!(
            "SELECT lift_id, attempt_number, weight, is_successful
             FROM attempts
             WHERE lift_id = ANY($1)
             ORDER BY lift_id, attempt_number",
            &lift_ids
        )
        .fetch_all(self.pool)
        .await?;

        let mut attempts_by_lift: HashMap<Uuid, Vec<AttemptSummary>> = HashMap::new();
        for attempt in attempts {
            attempts_by_lift
                .entry(attempt.lift_id)
                .or_default()
                .push(AttemptSummary {
                    attempt_number: attempt.attempt_number,
                    weight: attempt.weight,
                    is_successful: attempt.is_successful,
                });
        }

        let mut lifts_by_participant: HashMap<Uuid, Vec<LiftDetail>> = HashMap::new();
        for lift in lifts {
            lifts_by_participant
                .entry(lift.participant_id)
                .or_default()
                .push(LiftDetail {
                    movement_name: lift.movement_name,
                    best_weight: lift.max_weight,
                    record: lift.record,
                    attempts: attempts_by_lift.remove(&lift.lift_id).unwrap_or_default(),
                });
        }

        let mut by_category: HashMap<(Option<Uuid>, Option<Uuid>, Gender), Vec<ParticipantDetail>> =
            HashMap::new();
        for participant in participants {
            by_category
                .entry((
                    participant.weight_class_id,
                    participant.division_id,
                    participant.category_gender,
                ))
                .or_default()
                .push(ParticipantDetail {
                    participant_id: participant.participant_id,
                    athlete: AthleteRow {
                        athlete_id: participant.athlete_id,
                        first_name: participant.first_name,
                        last_name: participant.last_name,
                        native_name: participant.native_name,
                        gender: participant.gender,
                        created_at: participant.created_at,
                        country: participant.country,
                        profile_picture_url: participant.profile_picture_url,
                        slug: participant.slug,
                        slug_history: participant.slug_history,
                    },
                    bodyweight: participant.bodyweight,
                    age: participant.age,
                    rank: participant.rank,
                    ris_score: participant.ris_score,
                    ris_source: participant.ris_source,
                    status: participant.status,
                    status_reason: participant.status_reason,
                    total: participant.total,
                    total_record: participant.total_record,
                    lifts: lifts_by_participant
                        .remove(&participant.participant_id)
                        .unwrap_or_default(),
                });
        }

        let mut category_details: Vec<CategoryParticipants> = categories
            .into_iter()
            .map(|category| CategoryParticipants {
                participants: by_category
                    .remove(&(
                        category.weight_class_id,
                        category.division_id,
                        category.gender,
                    ))
                    .unwrap_or_default(),
                category,
            })
            .collect();

        category_details
            .sort_by(|a, b| category_order(&a.category).cmp(&category_order(&b.category)));

        Ok(category_details)
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
