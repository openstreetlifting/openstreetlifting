-- Store one total for imports, rankings, pages and RIS computation.
ALTER TABLE competition_participants RENAME COLUMN reported_total TO total;
ALTER TABLE competition_participants
    DROP CONSTRAINT reported_total_valid,
    ADD CONSTRAINT total_valid CHECK (
        total IS NULL OR (total >= 0 AND status = 'competed')
    );

-- Backfill only complete event breakdowns. Missing results are not zeroes.
WITH totals AS (
    SELECT cp.participant_id, SUM(l.max_weight) AS total
    FROM competition_participants cp
    JOIN competition_movements cm ON cm.competition_id = cp.competition_id
    LEFT JOIN lifts l ON l.participant_id = cp.participant_id
                     AND l.movement_name = cm.movement_name
    WHERE cp.status = 'competed' AND cp.total IS NULL
    GROUP BY cp.participant_id
    HAVING COUNT(l.max_weight) = COUNT(cm.movement_name)
)
UPDATE competition_participants cp
SET total = totals.total
FROM totals
WHERE totals.participant_id = cp.participant_id;

-- Remove calculated scores based on subtotals; preserve published evidence.
UPDATE competition_participants cp
SET ris_score = NULL, ris_source = NULL, ris_edition = NULL
FROM competitions c
WHERE c.competition_id = cp.competition_id
  AND cp.ris_source = 'computed'
  AND (cp.total IS NULL OR cp.bodyweight IS NULL
       OR cp.status != 'competed' OR c.event_code IS DISTINCT FROM 'MPDS');
