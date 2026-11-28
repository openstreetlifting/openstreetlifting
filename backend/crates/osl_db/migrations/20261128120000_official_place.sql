ALTER TABLE competition_participants
    ADD COLUMN place SMALLINT,
    ADD CONSTRAINT place_is_for_competed_results
        CHECK (place IS NULL OR (place > 0 AND status = 'competed'));

CREATE OR REPLACE VIEW participant_standings AS
WITH ranked AS (
    SELECT cp.participant_id, cp.competition_id, cp.weight_class_id, cp.division_id, cp.place,
           COALESCE(cp.category_gender, wc.gender, a.gender) AS category_gender,
           COALESCE(cp.rank_by, c.rank_by, CASE WHEN cp.weight_class_id IS NULL THEN 'ris' ELSE 'total' END) AS rank_by,
           cp.total, COALESCE(cp.reported_ris_score, cp.ris_score) AS placing_ris, cp.bodyweight
    FROM competition_participants cp
    JOIN competitions c USING (competition_id)
    JOIN athletes a USING (athlete_id)
    LEFT JOIN weight_classes wc USING (weight_class_id)
    WHERE cp.status = 'competed'
)
SELECT participant_id, competition_id, place::int AS rank
FROM ranked
WHERE place IS NOT NULL
UNION ALL
SELECT participant_id, competition_id,
       ROW_NUMBER() OVER (
           PARTITION BY competition_id, weight_class_id, division_id, category_gender
           ORDER BY CASE WHEN rank_by = 'ris' THEN placing_ris ELSE total END DESC,
                    total DESC NULLS LAST, bodyweight ASC NULLS LAST, participant_id
       )::int AS rank
FROM ranked
WHERE place IS NULL
  AND ((rank_by = 'ris' AND placing_ris IS NOT NULL)
   OR (rank_by = 'total' AND total IS NOT NULL));
