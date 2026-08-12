use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::error::Result;
use crate::params::{RankingFilter, RankingMovement};
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

    /// Every result with its per-movement bests, before anything is excluded.
    ///
    /// A movement nobody contested is NULL rather than 0, so "did not do this
    /// lift" stops looking like "lifted nothing". `total` is the sum of what
    /// the athlete actually did, which only means something next to another
    /// total from the same event.
    fn movement_weights(filter: &RankingFilter) -> QueryBuilder<Postgres> {
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
                    c.event_code,
                    MAX(CASE WHEN l.movement_name = 'Muscle-up' THEN l.max_weight END) as muscleup,
                    MAX(CASE WHEN l.movement_name = 'Pull-up' THEN l.max_weight END) as pullup,
                    MAX(CASE WHEN l.movement_name = 'Dips' THEN l.max_weight END) as dips,
                    MAX(CASE WHEN l.movement_name = 'Squat' THEN l.max_weight END) as squat,
                    SUM(l.max_weight) as total,
                    cp.ris_score,
                    cp.ris_source
                FROM competition_participants cp
                INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
                INNER JOIN competitions c ON cp.competition_id = c.competition_id
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

        query.push(
            r#"
                GROUP BY cp.participant_id, a.athlete_id, a.first_name, a.last_name,
                         a.slug, a.country, a.gender, cp.bodyweight, cp.ris_score, cp.ris_source,
                         c.competition_id, c.name, c.start_date, c.event_code
            ),
            "#,
        );

        query
    }

    /// Who belongs in this ranking at all.
    ///
    /// Ranking by one movement spans every event, because a muscle-up is a
    /// muscle-up whatever else the meet ran; it only drops athletes who never
    /// contested it. Ranking by total stays inside a single event, because
    /// adding four lifts and adding one are not the same measurement.
    fn push_eligible(query: &mut QueryBuilder<Postgres>, filter: &RankingFilter) {
        let sort_column = filter.movement.as_column();

        query.push(" eligible AS ( SELECT * FROM movement_weights WHERE ");
        query.push(sort_column);
        query.push(" IS NOT NULL ");

        if filter.movement == RankingMovement::Total {
            query.push(" AND event_code = ");
            query.push_bind(filter.event.clone());
        }

        query.push(" ) ");
    }

    async fn count_participants(&self, filter: &RankingFilter) -> Result<i64> {
        let mut query = Self::movement_weights(filter);
        Self::push_eligible(&mut query, filter);
        query.push(" SELECT COUNT(*) FROM eligible ");

        let count = query
            .build_query_scalar::<i64>()
            .fetch_one(self.pool)
            .await?;

        Ok(count)
    }

    async fn fetch_ranked_entries(&self, filter: &RankingFilter) -> Result<Vec<RankingRow>> {
        let sort_column = filter.movement.as_column();

        let mut query = Self::movement_weights(filter);
        Self::push_eligible(&mut query, filter);

        query.push(" , ranked_movements AS ( SELECT *, ROW_NUMBER() OVER (ORDER BY ");
        query.push(sort_column);
        query.push(" DESC) as rank FROM eligible ) ");
        query.push(" SELECT * FROM ranked_movements ORDER BY rank LIMIT ");
        query.push_bind(filter.limit);
        query.push(" OFFSET ");
        query.push_bind(filter.offset);

        let rows: Vec<RankingRow> = query.build_query_as().fetch_all(self.pool).await?;

        Ok(rows)
    }
}
