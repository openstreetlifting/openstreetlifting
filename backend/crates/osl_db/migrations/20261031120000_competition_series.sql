ALTER TABLE competitions ADD COLUMN series TEXT
    CHECK (series IS NULL OR series ~ '^[a-z0-9]+(-[a-z0-9]+)*$');

CREATE INDEX competitions_series_start_date_idx ON competitions (series, start_date DESC)
    WHERE series IS NOT NULL;
