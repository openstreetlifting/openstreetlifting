-- The column names the metric results are ordered by, not how points are computed.
ALTER TABLE competitions RENAME COLUMN scoring TO rank_by;
ALTER TABLE competitions RENAME CONSTRAINT competitions_scoring_check TO competitions_rank_by_check;

-- One meet can run divisions under different rules, such as an Open ranked by
-- total beside an Elite ranked by RIS. Divisions belong to a federation, so the
-- rule a division used at one meet is kept on its entries.
ALTER TABLE competition_participants ADD COLUMN rank_by TEXT CHECK (rank_by IN ('total', 'ris'));

CREATE OR REPLACE VIEW participant_standings AS
WITH ranked AS (
    SELECT cp.participant_id, cp.competition_id, cp.weight_class_id, cp.division_id,
           COALESCE(cp.category_gender, wc.gender, a.gender) AS category_gender,
           COALESCE(cp.rank_by, c.rank_by, CASE WHEN cp.weight_class_id IS NULL THEN 'ris' ELSE 'total' END) AS rank_by,
           cp.total, cp.ris_score, cp.bodyweight
    FROM competition_participants cp
    JOIN competitions c USING (competition_id)
    JOIN athletes a USING (athlete_id)
    LEFT JOIN weight_classes wc USING (weight_class_id)
    WHERE cp.status = 'competed'
)
SELECT participant_id, competition_id,
       ROW_NUMBER() OVER (
           PARTITION BY competition_id, weight_class_id, division_id, category_gender
           ORDER BY CASE WHEN rank_by = 'ris' THEN ris_score ELSE total END DESC,
                    total DESC NULLS LAST, bodyweight ASC NULLS LAST, participant_id
       )::int AS rank
FROM ranked
WHERE (rank_by = 'ris' AND ris_score IS NOT NULL)
   OR (rank_by = 'total' AND total IS NOT NULL);
