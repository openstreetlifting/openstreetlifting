-- Categories stop owning their bounds and point at a weight class instead.
-- Two meets labelling the same class differently keep their own labels but
-- resolve to one weight_classes row, which is what makes them comparable.

ALTER TABLE categories
    ADD COLUMN "weight_class_id" UUID REFERENCES weight_classes(weight_class_id)
        ON UPDATE CASCADE ON DELETE RESTRICT;

-- Any bounds outside the seeded ladder become their own row.
INSERT INTO weight_classes (gender, min_kg, max_kg)
SELECT DISTINCT gender, weight_class_min, weight_class_max
FROM categories
WHERE weight_class_min IS NOT NULL OR weight_class_max IS NOT NULL
ON CONFLICT ON CONSTRAINT weight_class_bounds_unique DO NOTHING;

UPDATE categories c
SET weight_class_id = wc.weight_class_id
FROM weight_classes wc
WHERE wc.gender = c.gender
  AND wc.min_kg IS NOT DISTINCT FROM c.weight_class_min
  AND wc.max_kg IS NOT DISTINCT FROM c.weight_class_max;

-- Fails loudly if a category carried no bounds at all, rather than leaving a
-- row whose class nothing can determine.
ALTER TABLE categories ALTER COLUMN "weight_class_id" SET NOT NULL;

ALTER TABLE categories DROP COLUMN "weight_class_min";
ALTER TABLE categories DROP COLUMN "weight_class_max";

CREATE INDEX "categories_index_2" ON "categories" ("weight_class_id");
