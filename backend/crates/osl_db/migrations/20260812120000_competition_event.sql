-- Not every meet contests all four movements. A muscle-up-only competition is
-- a real thing, and its total is not comparable with a four-lift total, so the
-- two must never be ranked against each other.
--
-- Which movements a meet contested is its event, the way OpenPowerlifting calls
-- a full-power meet SBD. The letters live here rather than being derived from
-- movement names, so adding a movement forces someone to choose a letter
-- instead of silently colliding with an existing one.

ALTER TABLE movements ADD COLUMN code VARCHAR(2);

UPDATE movements SET code = CASE name
    WHEN 'Muscle-up' THEN 'M'
    WHEN 'Pull-up'   THEN 'P'
    WHEN 'Dips'      THEN 'D'
    WHEN 'Squat'     THEN 'S'
END;

ALTER TABLE movements ALTER COLUMN code SET NOT NULL;
CREATE UNIQUE INDEX movements_code_unique ON movements (code);

-- Codes in display_order, so the four-movement event always reads MPDS rather
-- than depending on the order a file happened to list its movements.
ALTER TABLE competitions ADD COLUMN event_code TEXT;

UPDATE competitions c
SET event_code = (
    SELECT string_agg(m.code, '' ORDER BY m.display_order)
    FROM competition_movements cm
    JOIN movements m ON m.name = cm.movement_name
    WHERE cm.competition_id = c.competition_id
);

CREATE INDEX competitions_event_code_idx ON competitions (event_code);

-- Left nullable on purpose: a competition with no movements recorded has no
-- event, and ranking by total simply will not match it.
