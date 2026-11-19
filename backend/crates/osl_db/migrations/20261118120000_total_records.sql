ALTER TABLE records
    DROP CONSTRAINT records_movement_name_check,
    ADD CONSTRAINT records_movement_name_check
        CHECK (movement_name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat', 'Total'));

ALTER TABLE record_progressions
    DROP CONSTRAINT record_progressions_movement_name_check,
    ADD CONSTRAINT record_progressions_movement_name_check
        CHECK (movement_name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat', 'Total'));
