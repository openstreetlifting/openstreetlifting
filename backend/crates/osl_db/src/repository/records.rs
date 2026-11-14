use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::error::Result;
use crate::filter::{Filters, Predicate};
use crate::params::{RecordBy, RecordClass, RecordsFilter};
use crate::projections::records::{RecordFacets, RecordRow};

const SET: &str = "
    FROM records r
    JOIN athletes a ON a.athlete_id = r.athlete_id
    JOIN competitions c ON c.competition_id = r.competition_id
    JOIN federations f ON f.federation_id = c.federation_id
    WHERE TRUE
";

impl Predicate for RecordBy {
    fn push(&self, query: &mut QueryBuilder<Postgres>) {
        match self {
            Self::Movement(movement) => query
                .push("r.movement_name = ")
                .push_bind(movement.as_str()),
            Self::Gender(gender) => query.push("r.gender = ").push_bind(gender.as_str()),
            Self::Country(country) => query.push("a.country = ").push_bind(country.clone()),
            Self::Federation(federation) => query.push("f.name = ").push_bind(federation.clone()),
            Self::Year(year) => query
                .push("EXTRACT(YEAR FROM r.date)::int = ")
                .push_bind(*year),
        };
    }
}

pub struct RecordsRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> RecordsRepository<'a> {
    #[must_use]
    pub const fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "repository.records.list", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn list(&self, filter: &RecordsFilter) -> Result<Vec<RecordRow>> {
        let mut query = QueryBuilder::<Postgres>::new(
            r"
            WITH scoped AS (
                SELECT r.*, a.first_name, a.last_name, a.slug, a.country,
                       c.name AS competition_name, c.slug AS competition_slug
            ",
        );
        query.push(SET);
        filter.by.push_and(&mut query);
        query.push(
            r"
            ),
            pool AS (
                SELECT * FROM (
                    SELECT DISTINCT ON (s.movement_name, s.weight_class_id, s.athlete_id)
                           s.*, wc.slug AS weight_class, wc.max_kg, wc.min_kg
                    FROM scoped s
                    JOIN weight_classes wc ON wc.weight_class_id = s.weight_class_id
                    WHERE wc.slug IS NOT NULL
                    ORDER BY s.movement_name, s.weight_class_id, s.athlete_id, s.weight DESC, s.date
                ) classes
                UNION ALL
                SELECT * FROM (
                    SELECT DISTINCT ON (s.movement_name, s.athlete_id)
                           s.*, NULL::VARCHAR AS weight_class,
                           NULL::DECIMAL AS max_kg, NULL::DECIMAL AS min_kg
                    FROM scoped s
                    ORDER BY s.movement_name, s.athlete_id, s.weight DESC, s.date
                ) open
            ),
            ranked AS (
                SELECT *, ROW_NUMBER() OVER (
                    PARTITION BY movement_name, gender, weight_class
                    ORDER BY weight DESC, date, last_name, first_name, athlete_id
                ) AS rank
                FROM pool
                WHERE TRUE
            ",
        );

        match &filter.weight_class {
            Some(RecordClass::Open) => {
                query.push(" AND weight_class IS NULL ");
            }
            Some(RecordClass::Slug(slug)) => {
                query.push(" AND weight_class = ");
                query.push_bind(slug.clone());
            }
            None => {}
        }

        query.push(
            r"
            )
            SELECT movement_name, gender, weight_class, rank, first_name, last_name, slug, country,
                   weight, date, competition_name, competition_slug
            FROM ranked
            WHERE rank <= ",
        );
        query.push_bind(filter.limit);
        query.push(
            r"
            ORDER BY array_position(ARRAY['Muscle-up', 'Pull-up', 'Dips', 'Squat']::VARCHAR[], movement_name),
                     gender DESC, weight_class IS NOT NULL, max_kg NULLS LAST, min_kg, rank
            ",
        );

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    #[tracing::instrument(name = "repository.records.facets", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn facets(&self, by: &Filters<RecordBy>) -> Result<RecordFacets> {
        let mut countries = Self::set(
            "SELECT DISTINCT a.country",
            &by.without(|by| matches!(by, RecordBy::Country(_))),
        );
        countries.push(" AND a.country IS NOT NULL ORDER BY 1 ");

        let mut federations = Self::set(
            "SELECT DISTINCT f.name",
            &by.without(|by| matches!(by, RecordBy::Federation(_))),
        );
        federations.push(" ORDER BY 1 ");

        let mut years = Self::set(
            "SELECT DISTINCT EXTRACT(YEAR FROM r.date)::int",
            &by.without(|by| matches!(by, RecordBy::Year(_))),
        );
        years.push(" ORDER BY 1 DESC ");

        Ok(RecordFacets {
            countries: countries.build_query_scalar().fetch_all(self.pool).await?,
            federations: federations
                .build_query_scalar()
                .fetch_all(self.pool)
                .await?,
            years: years.build_query_scalar().fetch_all(self.pool).await?,
        })
    }

    fn set(select: &str, by: &Filters<RecordBy>) -> QueryBuilder<Postgres> {
        let mut query = QueryBuilder::new(select);
        query.push(SET);
        by.push_and(&mut query);
        query
    }

    #[tracing::instrument(name = "repository.records.marks", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn marks(
        &self,
        participant_ids: &[uuid::Uuid],
    ) -> Result<Vec<(uuid::Uuid, osl_domain::Movement, osl_domain::RecordMark)>> {
        Ok(sqlx::query_as(
            "SELECT participant_id, movement_name, mark FROM record_marks WHERE participant_id = ANY($1)",
        )
        .bind(participant_ids)
        .fetch_all(self.pool)
        .await?)
    }
}
