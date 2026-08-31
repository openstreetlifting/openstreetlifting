-- Display only. match_key is untouched, so adding a native name to an existing
-- athlete never moves them.
ALTER TABLE athletes ADD COLUMN native_name TEXT;
ALTER TABLE athletes ADD COLUMN native_script TEXT;

ALTER TABLE athletes ADD CONSTRAINT athletes_native_name_has_a_script CHECK (
    (native_name IS NULL AND native_script IS NULL)
    OR (native_name IS NOT NULL AND native_script IS NOT NULL)
);

ALTER TABLE athletes ADD CONSTRAINT athletes_native_script_is_known CHECK (
    native_script IS NULL
    OR native_script IN ('cyrillic', 'greek', 'han', 'japanese', 'korean')
);
