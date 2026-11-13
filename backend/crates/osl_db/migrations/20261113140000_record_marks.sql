CREATE TABLE record_marks (
    "participant_id" UUID NOT NULL REFERENCES competition_participants (participant_id) ON DELETE CASCADE,
    "movement_name" VARCHAR(20) NOT NULL CHECK (movement_name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat')),
    "mark" VARCHAR(2) NOT NULL CHECK (mark IN ('WR', 'NR')),
    PRIMARY KEY (participant_id, movement_name)
);
