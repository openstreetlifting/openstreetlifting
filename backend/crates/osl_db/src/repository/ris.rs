use sqlx::PgPool;

use crate::error::Result;
use crate::projections::ris::ScoredPerformance;

pub struct RisRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> RisRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Every performance the current edition scored, with its athlete and meet.
    /// Reported scores are left out: they were produced by constants we
    /// did not choose, so plotting them against our curve would compare two
    /// scales.
    ///
    /// One row per performance rather than per athlete, because the curve is
    /// fitted to performances and a repeat competitor is not one point.
    pub async fn scored_performances(&self) -> Result<Vec<ScoredPerformance>> {
        let performances = sqlx::query_as!(
            ScoredPerformance,
            r#"
            SELECT
                cp.participant_id,
                CONCAT_WS(' ', a.first_name, a.last_name) as "athlete_name!",
                a.slug as athlete_slug,
                c.name as competition_name,
                c.slug as competition_slug,
                c.start_date as competition_date,
                a.gender,
                cp.bodyweight as "bodyweight!",
                COALESCE(SUM(l.max_weight), 0) as "total!"
            FROM competition_participants cp
            INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
            INNER JOIN competitions c ON cp.competition_id = c.competition_id
            INNER JOIN lifts l ON l.participant_id = cp.participant_id
            WHERE cp.ris_source = 'computed'
              AND cp.bodyweight IS NOT NULL
            GROUP BY cp.participant_id, a.athlete_id, c.competition_id
            ORDER BY cp.bodyweight, cp.participant_id
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(performances)
    }
}
