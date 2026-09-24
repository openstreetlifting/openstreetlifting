-- Keep existing IDs and URLs while making country independent of identity.
-- These unresolved historical profiles remain separate pending source review.
WITH identities(match_key, gender, country, number) AS (VALUES
    ('harry twister', 'M', 'BA', 1),
    ('harry twister', 'M', 'SE', 2),
    ('denilson monteiro', 'M', 'DE', 1),
    ('denilson monteiro', 'M', 'FR', 2),
    ('giuseppe cicero', 'M', 'IT', 1),
    ('giuseppe cicero', 'M', 'SM', 2),
    ('lorenzo giorgetti', 'M', 'IT', 1),
    ('lorenzo giorgetti', 'M', 'SM', 2),
    ('jacopo bartoli', 'M', 'IT', 1),
    ('jacopo bartoli', 'M', 'SM', 2),
    ('ilaria valentini', 'F', 'IT', 1),
    ('ilaria valentini', 'F', 'SM', 2),
    ('tony nguyen', 'M', 'FR', 1),
    ('tony nguyen', 'M', 'US', 2)
)
UPDATE athletes a SET disambiguation = i.number
FROM identities i
WHERE a.match_key = i.match_key AND a.gender = i.gender AND a.country = i.country
  AND a.disambiguation IS NULL;

DROP INDEX athletes_identity_unique;
CREATE UNIQUE INDEX athletes_identity_unique
    ON athletes (match_key, gender, disambiguation) NULLS NOT DISTINCT;

ALTER TABLE athletes ALTER COLUMN country DROP NOT NULL;
ALTER TABLE competition_participants ADD COLUMN country VARCHAR(255);
UPDATE competition_participants cp SET country = a.country
FROM athletes a WHERE a.athlete_id = cp.athlete_id;
ALTER TABLE competition_participants ADD CONSTRAINT participant_country_valid
    CHECK (country IS NULL OR country ~ '^[A-Z]{2}$');
