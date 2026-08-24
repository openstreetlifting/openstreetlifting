-- A meet can run one weight class twice, once per division, and those are two
-- contests with two winners. Categories could not say so: the row was a name,
-- a gender and a weight class, and the name only spelled out the other two.
--
-- With the name gone the table is a surrogate key over (weight_class_id,
-- division_id) whose gender repeats weight_classes.gender, so it goes and the
-- participant carries both keys itself.

-- Division names are not standardised between federations: one body's "Elite"
-- has nothing to do with another's, so the name is only unique within one.
CREATE TABLE IF NOT EXISTS "divisions" (
    "division_id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "federation_id" UUID NOT NULL REFERENCES federations(federation_id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    "name" VARCHAR(100) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("division_id"),
    CONSTRAINT "division_name_unique_per_federation" UNIQUE ("federation_id", "name")
);

CREATE INDEX "divisions_index_0" ON "divisions" ("federation_id");

ALTER TABLE competition_participants
    ADD COLUMN "weight_class_id" UUID REFERENCES weight_classes(weight_class_id)
        ON UPDATE CASCADE ON DELETE RESTRICT,
    ADD COLUMN "division_id" UUID REFERENCES divisions(division_id)
        ON UPDATE CASCADE ON DELETE RESTRICT;

UPDATE competition_participants cp
SET weight_class_id = c.weight_class_id
FROM categories c
WHERE cp.category_id = c.category_id;

ALTER TABLE competition_participants ALTER COLUMN "weight_class_id" SET NOT NULL;

DROP INDEX IF EXISTS competition_participants_unique_idx;
DROP INDEX IF EXISTS competition_participants_category_rank_idx;

ALTER TABLE competition_participants DROP COLUMN "category_id";

-- Nothing ever wrote this. A placing depends on who else turned up, so it is
-- computed from the lifts at read time rather than stored and left to rot.
ALTER TABLE competition_participants DROP COLUMN "rank";

ALTER TABLE records
    ADD COLUMN "weight_class_id" UUID REFERENCES weight_classes(weight_class_id)
        ON UPDATE CASCADE ON DELETE RESTRICT,
    ADD COLUMN "division_id" UUID REFERENCES divisions(division_id)
        ON UPDATE CASCADE ON DELETE RESTRICT;

UPDATE records r
SET weight_class_id = c.weight_class_id
FROM categories c
WHERE r.category_id = c.category_id;

ALTER TABLE records ALTER COLUMN "weight_class_id" SET NOT NULL;

ALTER TABLE records DROP COLUMN "category_id";

DROP TABLE categories;

-- Postgres treats NULLs as distinct by default, so a plain UNIQUE would let a
-- divisionless meet enter one athlete in one class twice.
CREATE UNIQUE INDEX "competition_participants_unique_idx"
    ON "competition_participants" ("competition_id", "weight_class_id", "division_id", "athlete_id")
    NULLS NOT DISTINCT;

CREATE INDEX "competition_participants_contest_idx"
    ON "competition_participants" ("competition_id", "weight_class_id", "division_id");

CREATE INDEX "records_index_2" ON "records" ("weight_class_id", "movement_name");

CREATE UNIQUE INDEX "records_index_3"
    ON "records" ("record_type", "weight_class_id", "division_id", "movement_name", "gender")
    NULLS NOT DISTINCT;
