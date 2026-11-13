-- Precomputed per-athlete metric standings. One row per (metric, athlete),
-- rebuilt from competition data by the import pipeline. The athlete detail
-- API reads this table instead of re-ranking every participant per request.

CREATE TABLE IF NOT EXISTS "athlete_metric_standings" (
    "metric" VARCHAR(20) NOT NULL CHECK (metric IN ('muscleup', 'pullup', 'dips', 'squat', 'total', 'ris')),
    "athlete_id" UUID NOT NULL,
    "value" DECIMAL NOT NULL,
    "global_place" BIGINT NOT NULL CHECK (global_place > 0),
    "global_field" BIGINT NOT NULL CHECK (global_field > 0),
    "country" VARCHAR(255),
    "country_place" BIGINT NOT NULL CHECK (country_place > 0),
    "country_field" BIGINT NOT NULL CHECK (country_field > 0),
    "weight_class_id" UUID REFERENCES "weight_classes" ("weight_class_id") ON DELETE SET NULL,
    "weight_class_min" DECIMAL,
    "weight_class_max" DECIMAL,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("metric", "athlete_id"),
    CONSTRAINT "athlete_metric_standings_field" CHECK (global_place <= global_field AND country_place <= country_field),
    CONSTRAINT "athlete_metric_standings_class_bounds_ordered" CHECK (weight_class_min IS NULL OR weight_class_max IS NULL OR weight_class_max > weight_class_min)
);

CREATE INDEX "athlete_metric_standings_global_place" ON "athlete_metric_standings" ("metric", "global_place");
CREATE INDEX "athlete_metric_standings_country_place" ON "athlete_metric_standings" ("metric", "country", "country_place");
CREATE INDEX "athlete_metric_standings_athlete" ON "athlete_metric_standings" ("athlete_id");
