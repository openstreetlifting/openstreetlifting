ALTER TABLE competitions ADD COLUMN venue TEXT
    CHECK (venue IS NULL OR length(trim(venue)) > 0);
