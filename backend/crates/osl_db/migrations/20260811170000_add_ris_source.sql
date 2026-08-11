-- A RIS score we computed and one a source reported are not the same claim.
-- Ours is reproducible from the bodyweight and the formula version; a reported
-- one cannot be recomputed, verified, or migrated to a later formula.

ALTER TABLE competition_participants
    ADD COLUMN "ris_source" VARCHAR(20) CHECK (ris_source IN ('computed', 'reported'));

-- Everything scored so far came from our own formula run. This has to happen
-- before the constraint below, which validates existing rows as it is added.
UPDATE competition_participants SET ris_source = 'computed' WHERE ris_score IS NOT NULL;

-- A score and its provenance travel together, always.
ALTER TABLE competition_participants
    ADD CONSTRAINT "ris_score_has_a_source"
    CHECK ((ris_score IS NULL) = (ris_source IS NULL));
