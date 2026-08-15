ALTER TABLE competitions DROP COLUMN venue;
ALTER TABLE competitions DROP COLUMN number_of_judge;
ALTER TABLE competitions ADD COLUMN region VARCHAR(255);

CREATE INDEX competitions_region_idx ON competitions (country, region);
