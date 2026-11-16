-- The athlete's age at the competition, as published by the source. Stored
-- per entry rather than as a birth date: it is what results sheets print and
-- identifies the athlete less. Not exposed by the API yet.
ALTER TABLE "competition_participants"
    ADD COLUMN "age" SMALLINT CHECK ("age" > 0);
