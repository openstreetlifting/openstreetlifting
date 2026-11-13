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
            WITH pool AS (
                SELECT r.movement_name, r.gender, wc.slug AS weight_class, wc.max_kg, wc.min_kg,
                       r.athlete_id, r.competition_id, r.weight, r.date
                FROM records r
                JOIN weight_classes wc ON wc.weight_class_id = r.weight_class_id
                WHERE wc.slug IS NOT NULL
                UNION ALL
                SELECT * FROM (
                    SELECT DISTINCT ON (r.movement_name, r.athlete_id)
                           r.movement_name, r.gender, NULL::VARCHAR AS weight_class,
                           NULL::DECIMAL AS max_kg, NULL::DECIMAL AS min_kg,
                           r.athlete_id, r.competition_id, r.weight, r.date
                    FROM records r
                    ORDER BY r.movement_name, r.athlete_id, r.weight DESC, r.date
                ) open
            ),
            ranked AS (
                SELECT pool.movement_name, pool.gender, pool.weight_class, pool.max_kg, pool.min_kg,
                       pool.weight, pool.date,
                       a.first_name, a.last_name, a.slug, a.country,
                       c.name AS competition_name, c.slug AS competition_slug,
                       ROW_NUMBER() OVER (
                           PARTITION BY pool.movement_name, pool.gender, pool.weight_class
                           ORDER BY pool.weight DESC, pool.date, a.last_name, a.first_name, a.athlete_id
                       ) AS rank
                FROM pool
                JOIN athletes a ON a.athlete_id = pool.athlete_id
                JOIN competitions c ON c.competition_id = pool.competition_id
                WHERE TRUE
            ",
        );

        if let Some(movement) = filter.movement {
            query.push(" AND pool.movement_name = ");
            query.push_bind(movement.as_str());
        }
        if let Some(gender) = filter.gender {
            query.push(" AND pool.gender = ");
            query.push_bind(gender.as_str());
        }
        match &filter.weight_class {
            Some(RecordClass::Open) => {
                query.push(" AND pool.weight_class IS NULL ");
            }
            Some(RecordClass::Slug(slug)) => {
                query.push(" AND pool.weight_class = ");
                query.push_bind(slug.clone());
            }
            None => {}
        }
        if let Some(country) = &filter.country {
            query.push(" AND a.country = ");
            query.push_bind(country.clone());
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
}
