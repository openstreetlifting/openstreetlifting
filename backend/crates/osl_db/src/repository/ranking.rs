use sqlx::{PgPool, QueryBuilder};

use crate::error::Result;
use crate::params::RankingFilter;
use crate::projections::ranking::RankingRow;

pub struct RankingRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> RankingRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Returns one page of the global ranking plus the unpaginated total.
    pub async fn get_global_ranking(
        &self,
        filter: &RankingFilter,
    ) -> Result<(Vec<RankingRow>, i64)> {
        let total_items = self.count_participants(filter).await?;
        let entries = self.fetch_ranked_entries(filter).await?;

        Ok((entries, total_items))
    }

    async fn count_participants(&self, filter: &RankingFilter) -> Result<i64> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT COUNT(DISTINCT cp.participant_id)
            FROM competition_participants cp
            INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
            INNER JOIN lifts l ON cp.participant_id = l.participant_id
            WHERE 1=1
            "#,
        );

        if let Some(ref gender) = filter.gender {
            query.push(" AND a.gender = ");
            query.push_bind(gender);
        }

        if let Some(ref country) = filter.country {
            query.push(" AND a.country = ");
            query.push_bind(country);
        }

        let count = query
            .build_query_scalar::<i64>()
            .fetch_one(self.pool)
            .await?;

        Ok(count)
    }

    async fn fetch_ranked_entries(&self, filter: &RankingFilter) -> Result<Vec<RankingRow>> {
        let sort_column = filter.movement.as_column();

        let mut query = QueryBuilder::new(
            r#"
            WITH movement_weights AS (
                SELECT
                    cp.participant_id,
                    a.athlete_id,
                    a.first_name,
                    a.last_name,
                    a.slug,
                    a.country,
                    a.gender,
                    cp.bodyweight,
                    c.competition_id,
                    c.name as competition_name,
                    c.start_date,
                    COALESCE(MAX(CASE WHEN l.movement_name = 'Muscle-up' THEN l.max_weight END), 0) as muscleup,
                    COALESCE(MAX(CASE WHEN l.movement_name = 'Pull-up' THEN l.max_weight END), 0) as pullup,
                    COALESCE(MAX(CASE WHEN l.movement_name = 'Dips' THEN l.max_weight END), 0) as dips,
                    COALESCE(MAX(CASE WHEN l.movement_name = 'Squat' THEN l.max_weight END), 0) as squat,
                    COALESCE(SUM(l.max_weight), 0) as total,
                    MAX(rsh.ris_score) as ris_score
                FROM competition_participants cp
                INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
                INNER JOIN competitions c ON cp.competition_id = c.competition_id
                INNER JOIN lifts l ON cp.participant_id = l.participant_id
                LEFT JOIN ris_scores_history rsh ON rsh.participant_id = cp.participant_id
                WHERE 1=1
            "#,
        );

        if let Some(ref gender) = filter.gender {
            query.push(" AND a.gender = ");
            query.push_bind(gender);
        }

        if let Some(ref country) = filter.country {
            query.push(" AND a.country = ");
            query.push_bind(country);
        }

        query.push(
            r#"
                GROUP BY cp.participant_id, a.athlete_id, a.first_name, a.last_name,
                         a.slug, a.country, a.gender, cp.bodyweight, c.competition_id, c.name, c.start_date
            ),
            ranked_movements AS (
                SELECT *, ROW_NUMBER() OVER (ORDER BY
            "#,
        );
        query.push(sort_column);
        query.push(
            r#"
                DESC) as rank
                FROM movement_weights
            )
            SELECT * FROM ranked_movements
            ORDER BY rank
            LIMIT
            "#,
        );
        query.push_bind(filter.limit);
        query.push(" OFFSET ");
        query.push_bind(filter.offset);

        let rows: Vec<RankingRow> = query.build_query_as().fetch_all(self.pool).await?;

        Ok(rows)
    }
}
