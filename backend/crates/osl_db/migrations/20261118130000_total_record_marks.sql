ALTER TABLE record_marks
    DROP CONSTRAINT record_marks_movement_name_check,
    ADD CONSTRAINT record_marks_movement_name_check
        CHECK (movement_name IN ('Muscle-up', 'Pull-up', 'Dips', 'Squat', 'Total'));
