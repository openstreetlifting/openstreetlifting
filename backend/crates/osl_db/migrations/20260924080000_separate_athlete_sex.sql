-- Athlete sex selects identity and the RIS formula; MX describes a contest.
ALTER TABLE athletes ADD CONSTRAINT athlete_sex_valid CHECK (gender IN ('M', 'F'));

ALTER TABLE competitions ADD COLUMN scoring TEXT CHECK (scoring IN ('total', 'ris'));
ALTER TABLE competition_participants ADD COLUMN category_gender TEXT
    CHECK (category_gender IS NULL OR category_gender = 'MX');

-- A lifter may enter both a mixed and a single-sex contest without weight classes.
DROP INDEX competition_participants_unique_idx;
CREATE UNIQUE INDEX competition_participants_unique_idx
    ON competition_participants
       (competition_id, weight_class_id, division_id, athlete_id, category_gender)
    NULLS NOT DISTINCT;

-- Use the same contest membership and scoring rule on results and athlete pages.
CREATE VIEW participant_standings AS
WITH scored AS (
    SELECT cp.participant_id, cp.competition_id, cp.weight_class_id, cp.division_id,
           COALESCE(cp.category_gender, wc.gender, a.gender) AS category_gender,
           COALESCE(c.scoring, CASE WHEN cp.weight_class_id IS NULL THEN 'ris' ELSE 'total' END) AS scoring,
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
