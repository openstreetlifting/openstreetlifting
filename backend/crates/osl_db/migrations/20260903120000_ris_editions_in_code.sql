-- The RIS constants are third party reference data: three published sets that
-- change once a year. Keeping them in a table meant a copy of the migration
-- that created them, truncated to DECIMAL(10,5), so they now live in
-- osl_domain::ris instead.

-- A score is a number on a scale, so the scale travels with it. Nothing
-- recorded which edition produced competition_participants.ris_score.
ALTER TABLE competition_participants ADD COLUMN "ris_edition" INTEGER;

-- Everything we have scored so far used the only seeded formula.
UPDATE competition_participants SET ris_edition = 2025 WHERE ris_source = 'computed';

ALTER TABLE competition_participants
    ADD CONSTRAINT "computed_ris_names_its_edition"
    CHECK (
        (ris_source = 'computed' AND ris_edition IS NOT NULL)
        OR (ris_source IS DISTINCT FROM 'computed' AND ris_edition IS NULL)
    );

-- ris_scores_history was built for scoring each participant under every
-- edition, which never shipped. Both write paths only ever inserted one row
-- per participant holding the same value as competition_participants.ris_score,
-- so it is a duplicate of that column rather than a history.
DROP TABLE ris_scores_history;

DROP TABLE ris_formula_versions;
