-- Preserve a published All4 total when the source omits individual lift results.
ALTER TABLE competition_participants
    ADD COLUMN reported_total NUMERIC,
    ADD CONSTRAINT reported_total_valid CHECK (
        reported_total IS NULL OR (reported_total > 0 AND status = 'competed')
    );
