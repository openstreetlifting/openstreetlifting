-- A boolean can only say disqualified or not, so a lifter who never took an
-- attempt was stored as though they had lifted and been ruled out. The status
-- separates the two, and only 'competed' counts for rankings and records.

ALTER TABLE "competition_participants"
    ADD COLUMN "status" TEXT NOT NULL DEFAULT 'competed',
    ADD COLUMN "status_reason" TEXT;

UPDATE "competition_participants"
SET "status" = CASE WHEN "is_disqualified" THEN 'disqualified' ELSE 'competed' END,
    "status_reason" = "disqualified_reason";

ALTER TABLE "competition_participants"
    ADD CONSTRAINT "competition_participants_status_check"
    CHECK ("status" IN ('competed', 'disqualified', 'no_show'));

-- A reason explains an outcome, so it has nothing to say about a lifter who
-- simply competed.
ALTER TABLE "competition_participants"
    ADD CONSTRAINT "only_an_outcome_has_a_reason"
    CHECK ("status" <> 'competed' OR "status_reason" IS NULL);

DROP INDEX "competition_participants_index_disqualified";

ALTER TABLE "competition_participants"
    DROP COLUMN "is_disqualified",
    DROP COLUMN "disqualified_reason";

-- Every ranking and record query filters on this.
CREATE INDEX "competition_participants_index_status"
    ON "competition_participants" ("status")
    WHERE "status" <> 'competed';
