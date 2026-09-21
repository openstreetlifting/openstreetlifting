-- Competitions ranked on RIS alone define no weight classes.

ALTER TABLE competition_participants ALTER COLUMN "weight_class_id" DROP NOT NULL;
