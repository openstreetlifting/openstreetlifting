DROP TABLE records;

CREATE TABLE records (
    "participant_id" UUID NOT NULL REFERENCES competition_participants (participant_id) ON DELETE CASCADE,
    "movement_name" VARCHAR(20) NOT NULL CHECK (movement_name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat')),
    "gender" VARCHAR(1) NOT NULL CHECK (gender IN ('M', 'F')),
    "weight_class_id" UUID REFERENCES weight_classes (weight_class_id) ON DELETE CASCADE,
    "athlete_id" UUID NOT NULL REFERENCES athletes (athlete_id) ON DELETE CASCADE,
    "competition_id" UUID NOT NULL REFERENCES competitions (competition_id) ON DELETE CASCADE,
    "weight" DECIMAL NOT NULL CHECK (weight >= 0),
    "date" DATE NOT NULL,
    PRIMARY KEY (participant_id, movement_name)
);

CREATE INDEX records_by_weight ON records (movement_name, gender, weight DESC, date);
CREATE INDEX records_athlete ON records (athlete_id);
CREATE INDEX records_competition ON records (competition_id);
