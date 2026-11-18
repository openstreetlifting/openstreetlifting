use sqlx::{Postgres, QueryBuilder, Transaction};

use crate::error::Result;

const RESULTS: &str = r"
    WITH entrants AS (
        SELECT a.gender, cp.weight_class_id, wc.slug AS weight_class_slug,
               cp.athlete_id, cp.competition_id, cp.participant_id, cp.total,
               c.start_date AS date, c.event_code,
               a.last_name, a.first_name
        FROM competition_participants cp
        JOIN athletes a USING (athlete_id)
        JOIN competitions c ON c.competition_id = cp.competition_id
        LEFT JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
        WHERE cp.status = 'competed'
          AND cp.category_gender IS NULL
          AND (wc.gender IS NULL OR wc.gender = a.gender)
    ),
    results AS (
        SELECT l.movement_name, e.gender, e.weight_class_id, e.weight_class_slug,
               e.athlete_id, e.competition_id, e.participant_id,
               l.max_weight AS weight, e.date, e.last_name, e.first_name
        FROM lifts l
        JOIN entrants e USING (participant_id)
        WHERE l.max_weight IS NOT NULL
        UNION ALL
        SELECT 'Total', e.gender, e.weight_class_id, e.weight_class_slug,
               e.athlete_id, e.competition_id, e.participant_id,
               e.total, e.date, e.last_name, e.first_name
        FROM entrants e
        WHERE e.total IS NOT NULL
          AND e.event_code = 'MPDS'
    )
";

const RESULTS_INSERT: &str = r"
    INSERT INTO records
        (participant_id, movement_name, gender, weight_class_id, athlete_id, competition_id, weight, date)
    SELECT participant_id, movement_name, gender, weight_class_id, athlete_id, competition_id, weight, date
    FROM results
";

const PROGRESSIONS: &str = r"
    , scoped AS (
        SELECT *, weight_class_id AS scope FROM results WHERE weight_class_slug IS NOT NULL
        UNION ALL
        SELECT *, NULL AS scope FROM results
    ),
    daily AS (
        SELECT DISTINCT ON (movement_name, gender, scope, date) *
        FROM scoped
        ORDER BY movement_name, gender, scope, date, weight DESC, last_name, first_name, athlete_id
    ),
    running AS (
        SELECT *, MAX(weight) OVER (
            PARTITION BY movement_name, gender, scope
            ORDER BY date
            ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING
        ) AS previous_best
        FROM daily
    )
    INSERT INTO record_progressions
        (movement_name, gender, weight_class_id, athlete_id, competition_id, weight, previous_weight, date)
    SELECT movement_name, gender, scope, athlete_id, competition_id, weight, previous_best, date
    FROM running
    WHERE previous_best IS NULL OR weight > previous_best
";

const MARKS: &str = r"
    WITH ranked AS (
        SELECT r.participant_id, r.movement_name, r.weight_class_id, a.country,
               ROW_NUMBER() OVER (
                   PARTITION BY r.movement_name, r.weight_class_id
                   ORDER BY r.weight DESC, r.date, a.last_name, a.first_name, a.athlete_id
               ) AS world_rank,
               ROW_NUMBER() OVER (
                   PARTITION BY r.movement_name, r.weight_class_id, a.country
                   ORDER BY r.weight DESC, r.date, a.last_name, a.first_name, a.athlete_id
               ) AS national_rank
        FROM records r
        JOIN athletes a ON a.athlete_id = r.athlete_id
        JOIN weight_classes wc ON wc.weight_class_id = r.weight_class_id
        WHERE wc.slug IS NOT NULL
    )
    INSERT INTO record_marks (participant_id, movement_name, mark)
    SELECT participant_id, movement_name, CASE WHEN world_rank = 1 THEN 'WR' ELSE 'NR' END
    FROM ranked
    WHERE world_rank = 1 OR (national_rank = 1 AND country IS NOT NULL)
";

pub async fn refresh(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
    sqlx::query("TRUNCATE records, record_progressions, record_marks")
        .execute(&mut **tx)
        .await?;
    for statement in [RESULTS_INSERT, PROGRESSIONS] {
        let mut query = QueryBuilder::<Postgres>::new(RESULTS);
        query.push(statement);
        query.build().execute(&mut **tx).await?;
    }
    sqlx::query(MARKS).execute(&mut **tx).await?;

    Ok(())
}
