-- Two results the schema could not express, both ordinary in streetlifting.
--
-- A muscle-up with no added weight is a real, successful lift, recorded as
-- 0 kg. `CHECK (weight > 0)` rejected it outright.
--
-- A movement where every attempt failed has no weight at all. The importer
-- derives the stored weight from successful attempts, so it produced NULL and
-- hit the NOT NULL, which meant a lifter who bombed one movement could not be
-- imported even though the rest of their meet was fine.
--
-- The two must stay apart. 0 means they lifted their bodyweight and nothing
-- more; NULL means they lifted nothing at all. Collapsing the second into the
-- first would credit a lift that never happened.

ALTER TABLE attempts DROP CONSTRAINT attempts_weight_check;
ALTER TABLE attempts ADD CONSTRAINT attempts_weight_check CHECK (weight >= 0);

ALTER TABLE lifts DROP CONSTRAINT lifts_max_weight_check;
ALTER TABLE lifts ALTER COLUMN max_weight DROP NOT NULL;
ALTER TABLE lifts ADD CONSTRAINT lifts_max_weight_check CHECK (max_weight >= 0);

COMMENT ON COLUMN lifts.max_weight IS
    'Best successful attempt. 0 is a bodyweight-only lift; NULL means the '
    'movement was contested and no attempt succeeded.';
