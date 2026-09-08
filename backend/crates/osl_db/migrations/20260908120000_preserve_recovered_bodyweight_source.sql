ALTER TABLE competition_participants
    ADD COLUMN bodyweight_source TEXT CHECK (bodyweight_source IN ('reported', 'recovered')),
    ADD COLUMN reported_ris_score NUMERIC CHECK (reported_ris_score >= 0),
    ADD COLUMN reported_ris_edition INTEGER;

UPDATE competition_participants SET bodyweight_source = 'reported' WHERE bodyweight IS NOT NULL;
UPDATE competition_participants SET reported_ris_score = ris_score WHERE ris_source = 'reported';

ALTER TABLE competition_participants
    ADD CONSTRAINT bodyweight_has_source CHECK ((bodyweight IS NULL) = (bodyweight_source IS NULL)),
    ADD CONSTRAINT reported_ris_edition_has_score CHECK (reported_ris_edition IS NULL OR reported_ris_score IS NOT NULL),
    ADD CONSTRAINT recovered_bodyweight_has_evidence CHECK (
        bodyweight_source IS DISTINCT FROM 'recovered'
        OR (bodyweight IS NOT NULL AND reported_ris_score IS NOT NULL
            AND reported_ris_score > 0 AND reported_ris_edition IS NOT NULL)
    );
