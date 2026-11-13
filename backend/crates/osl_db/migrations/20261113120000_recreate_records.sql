DROP TABLE records;

CREATE TABLE records (
    "movement_name" VARCHAR(20) NOT NULL CHECK (movement_name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat')),
    "gender" VARCHAR(1) NOT NULL CHECK (gender IN ('M', 'F')),
    "weight_class_id" UUID REFERENCES weight_classes (weight_class_id) ON DELETE CASCADE,
    "athlete_id" UUID NOT NULL REFERENCES athletes (athlete_id) ON DELETE CASCADE,
    "competition_id" UUID NOT NULL REFERENCES competitions (competition_id) ON DELETE CASCADE,
    "weight" DECIMAL NOT NULL CHECK (weight >= 0),
    "date" DATE NOT NULL
);

CREATE UNIQUE INDEX records_one_best
    ON records (movement_name, weight_class_id, athlete_id) NULLS NOT DISTINCT;
CREATE INDEX records_by_weight ON records (movement_name, gender, weight DESC, date);
CREATE INDEX records_athlete ON records (athlete_id);

-- A NULL weight class is the open record for the gender.
CREATE TABLE record_progressions (
    "movement_name" VARCHAR(20) NOT NULL CHECK (movement_name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat')),
    "gender" VARCHAR(1) NOT NULL CHECK (gender IN ('M', 'F')),
    "weight_class_id" UUID REFERENCES weight_classes (weight_class_id) ON DELETE CASCADE,
    "athlete_id" UUID NOT NULL REFERENCES athletes (athlete_id) ON DELETE CASCADE,
    "competition_id" UUID NOT NULL REFERENCES competitions (competition_id) ON DELETE CASCADE,
    "weight" DECIMAL NOT NULL CHECK (weight >= 0),
    "previous_weight" DECIMAL,
    "date" DATE NOT NULL
);

CREATE INDEX record_progressions_scope
    ON record_progressions (movement_name, gender, weight_class_id, date);
CREATE INDEX record_progressions_athlete ON record_progressions (athlete_id);
