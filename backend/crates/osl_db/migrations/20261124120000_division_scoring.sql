-- One meet can run divisions under different rules, such as an Open ranked by
-- total beside an Elite ranked by RIS. Divisions belong to a federation, so the
-- rule a division used at one meet is kept on its entries.
ALTER TABLE competition_participants ADD COLUMN scoring TEXT CHECK (scoring IN ('total', 'ris'));

CREATE OR REPLACE VIEW participant_standings AS
WITH scored AS (
    SELECT cp.participant_id, cp.competition_id, cp.weight_class_id, cp.division_id,
           COALESCE(cp.category_gender, wc.gender, a.gender) AS category_gender,
           COALESCE(cp.scoring, c.scoring, CASE WHEN cp.weight_class_id IS NULL THEN 'ris' ELSE 'total' END) AS scoring,
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
           ORDER BY CASE WHEN scoring = 'ris' THEN ris_score ELSE total END DESC,
                    total DESC NULLS LAST, bodyweight ASC NULLS LAST, participant_id
       )::int AS rank
FROM scored
WHERE (scoring = 'ris' AND ris_score IS NOT NULL)
   OR (scoring = 'total' AND total IS NOT NULL);
