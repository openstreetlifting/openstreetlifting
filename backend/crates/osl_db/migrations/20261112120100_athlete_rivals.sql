-- Precomputed "current rivals" rows for the athlete profile. Each row mirrors
-- one entry of the former on-the-fly neighbors query: the athletes ranked
-- immediately above and below the profile athlete on the All4 total ranking,
-- in the profile athlete's latest weight class, globally and nationally.
-- Names and competition references are denormalized on purpose: the read path
-- is a single indexed lookup and the rows are fully rebuilt on every import.

CREATE TABLE IF NOT EXISTS "athlete_rivals" (
    "profile_athlete_id" UUID NOT NULL REFERENCES "athletes" ("athlete_id") ON DELETE CASCADE,
    "scope" VARCHAR(10) NOT NULL CHECK (scope IN ('global', 'national')),
    "place" BIGINT NOT NULL CHECK (place > 0),
    "field" BIGINT NOT NULL CHECK (field > 0),
    "neighbor_athlete_id" UUID NOT NULL REFERENCES "athletes" ("athlete_id") ON DELETE CASCADE,
    "first_name" TEXT NOT NULL,
    "last_name" TEXT NOT NULL,
    "slug" TEXT NOT NULL,
    "country" VARCHAR(255),
    "weight_class_min" DECIMAL,
    "weight_class_max" DECIMAL,
    "value" DECIMAL NOT NULL,
    "gap" DECIMAL NOT NULL,
    "competition_name" TEXT NOT NULL,
    "competition_slug" TEXT NOT NULL,
    "competition_date" DATE,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("profile_athlete_id", "scope", "place")
);

