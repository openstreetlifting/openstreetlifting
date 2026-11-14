use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::error::Result;
use crate::params::{RecordClass, RecordsFilter};
use crate::projections::records::RecordRow;

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
                FROM records r
                JOIN athletes a ON a.athlete_id = r.athlete_id
                JOIN competitions c ON c.competition_id = r.competition_id
                JOIN federations f ON f.federation_id = c.federation_id
                WHERE TRUE
            ",
        );
        Self::push_scope(&mut query, filter);
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

    fn push_scope(query: &mut QueryBuilder<Postgres>, filter: &RecordsFilter) {
        if let Some(movement) = filter.movement {
            query.push(" AND r.movement_name = ");
            query.push_bind(movement.as_str());
        }
        if let Some(gender) = filter.gender {
            query.push(" AND r.gender = ");
            query.push_bind(gender.as_str());
        }
        if let Some(country) = &filter.country {
            query.push(" AND a.country = ");
            query.push_bind(country.clone());
        }
        if let Some(federation) = &filter.federation {
            query.push(" AND f.name = ");
            query.push_bind(federation.clone());
        }
        if let Some(year) = filter.year {
            query.push(" AND EXTRACT(YEAR FROM r.date)::int = ");
            query.push_bind(year);
        }
    }

    #[tracing::instrument(name = "repository.records.list_years", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn list_years(&self) -> Result<Vec<i32>> {
        Ok(sqlx::query_scalar(
            "SELECT DISTINCT EXTRACT(YEAR FROM date)::int AS year FROM records ORDER BY year DESC",
        )
        .fetch_all(self.pool)
        .await?)
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
