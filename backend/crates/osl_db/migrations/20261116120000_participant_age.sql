-- Stored per entry rather than as a birth date: it is what results sheets
-- print and identifies the athlete less.
ALTER TABLE "competition_participants"
    ADD COLUMN "age" SMALLINT CHECK ("age" > 0);
