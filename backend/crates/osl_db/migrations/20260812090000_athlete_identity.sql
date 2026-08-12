-- Athlete identity was an exact match on first name, last name, gender and
-- country, so "Mérandon" and "MERANDON" were two people and two people sharing
-- a name were one. Nothing enforced uniqueness either, so duplicates could
-- already exist unnoticed.
--
-- match_key holds the name folded down to what identity ignores: accents, case
-- and punctuation. disambiguation separates two real people who genuinely share
-- a name, the way OpenPowerlifting writes "John Doe #1" and "John Doe #2".

CREATE EXTENSION IF NOT EXISTS unaccent;

ALTER TABLE athletes ADD COLUMN disambiguation SMALLINT
    CHECK (disambiguation IS NULL OR disambiguation > 0);

ALTER TABLE athletes ADD COLUMN match_key TEXT;

-- Approximates the importer's fold, which is the authority. A re-import
-- rewrites every match_key from Rust, so any disagreement here is corrected
-- rather than baked in, and a disagreement that would merge two people trips
-- the unique index below instead of passing silently.
UPDATE athletes
SET match_key = trim(
    regexp_replace(
        regexp_replace(
            lower(unaccent(first_name || ' ' || last_name)),
            '[''’]', '', 'g'
        ),
        '[^[:alnum:]]+', ' ', 'g'
    )
);

ALTER TABLE athletes ALTER COLUMN match_key SET NOT NULL;

-- NULLS NOT DISTINCT so two athletes with no disambiguator collide rather than
-- both being allowed through, which is the whole point of the constraint.
--
-- If this fails, duplicates already exist. List them with:
--   SELECT match_key, gender, country, COUNT(*), array_agg(slug)
--   FROM athletes GROUP BY 1, 2, 3 HAVING COUNT(*) > 1;
-- Each group is either one person imported twice, which needs the rows merged,
-- or two people who need a disambiguation number.
CREATE UNIQUE INDEX athletes_identity_unique
    ON athletes (match_key, gender, country, disambiguation) NULLS NOT DISTINCT;
