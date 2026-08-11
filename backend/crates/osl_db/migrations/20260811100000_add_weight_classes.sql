-- A weight class is identified by its bounds, not by its name. "Catégorie +87",
-- "+87 kg" and "M+87" are one class, so keying on the label forks it into
-- three. The unique constraint below is what stops that.

CREATE TABLE IF NOT EXISTS "weight_classes" (
    "weight_class_id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "gender" VARCHAR(10) NOT NULL CHECK (gender IN ('M', 'F', 'MX')),
    "min_kg" DECIMAL CHECK (min_kg > 0),
    "max_kg" DECIMAL CHECK (max_kg > 0),
    "slug" VARCHAR(20) UNIQUE,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("weight_class_id"),
    CONSTRAINT "weight_class_has_a_bound" CHECK (min_kg IS NOT NULL OR max_kg IS NOT NULL),
    CONSTRAINT "weight_class_bounds_ordered" CHECK (min_kg IS NULL OR max_kg IS NULL OR max_kg > min_kg),
    -- Postgres treats NULLs as distinct by default, so a plain UNIQUE would
    -- accept two (M, 87, NULL) rows and reintroduce the duplicates.
    CONSTRAINT "weight_class_bounds_unique" UNIQUE NULLS NOT DISTINCT ("gender", "min_kg", "max_kg")
);

CREATE INDEX "weight_classes_index_0" ON "weight_classes" ("gender");

-- The standard ladder. A class outside it is an ordinary row with a NULL slug.
INSERT INTO weight_classes (gender, min_kg, max_kg, slug)
VALUES
    ('F', NULL, 52,   'F-52'),
    ('F', 52,   57,   'F-57'),
    ('F', 57,   63,   'F-63'),
    ('F', 63,   70,   'F-70'),
    ('F', 70,   NULL, 'F+70'),
    ('M', NULL, 66,   'M-66'),
    ('M', 66,   73,   'M-73'),
    ('M', 73,   80,   'M-80'),
    ('M', 80,   87,   'M-87'),
    ('M', 87,   94,   'M-94'),
    ('M', 94,   101,  'M-101'),
    ('M', 101,  NULL, 'M+101')
ON CONFLICT (gender, min_kg, max_kg) DO UPDATE
SET slug = EXCLUDED.slug;
