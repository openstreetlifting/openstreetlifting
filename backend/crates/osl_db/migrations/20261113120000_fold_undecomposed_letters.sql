CREATE FUNCTION pg_temp.fold_letters(value TEXT) RETURNS TEXT
LANGUAGE SQL IMMUTABLE AS $$
    SELECT replace(replace(replace(replace(
        translate(value, 'łøđðı', 'loddi'),
        'ß', 'ss'), 'æ', 'ae'), 'œ', 'oe'), 'þ', 'th')
$$;

DO $$
DECLARE
    identity RECORD;
    retained_id UUID;
BEGIN
    FOR identity IN
        SELECT pg_temp.fold_letters(match_key) AS folded, gender, disambiguation,
               array_agg(athlete_id ORDER BY
                   (match_key = pg_temp.fold_letters(match_key)) DESC,
                   created_at, athlete_id) AS profile_ids
        FROM athletes
        GROUP BY 1, 2, 3
        HAVING COUNT(*) > 1
    LOOP
        retained_id := identity.profile_ids[1];

        UPDATE athletes retained
        SET slug_history = COALESCE((
                SELECT jsonb_agg(DISTINCT old_slug ORDER BY old_slug)
                FROM athletes a
                CROSS JOIN LATERAL jsonb_array_elements_text(
                    COALESCE(a.slug_history, '[]'::jsonb) || jsonb_build_array(a.slug)
                ) AS history(old_slug)
                WHERE a.athlete_id = ANY(identity.profile_ids) AND old_slug <> retained.slug
            ), '[]'::jsonb),
            native_name = COALESCE(retained.native_name, d.native_name),
            native_script = CASE WHEN retained.native_name IS NULL THEN d.native_script
                                 ELSE retained.native_script END,
            profile_picture_url = COALESCE(retained.profile_picture_url, d.profile_picture_url),
            created_at = LEAST(retained.created_at, d.created_at)
        FROM (
            SELECT * FROM athletes
            WHERE athlete_id = ANY(identity.profile_ids) AND athlete_id <> retained_id
            ORDER BY created_at
            LIMIT 1
        ) d
        WHERE retained.athlete_id = retained_id;

        UPDATE competition_participants SET athlete_id = retained_id
        WHERE athlete_id = ANY(identity.profile_ids);
        UPDATE records SET athlete_id = retained_id
        WHERE athlete_id = ANY(identity.profile_ids);

        DELETE FROM athlete_socials duplicate
        WHERE duplicate.athlete_id = ANY(identity.profile_ids)
          AND duplicate.athlete_id <> retained_id
          AND EXISTS (
              SELECT 1 FROM athlete_socials kept
              WHERE kept.athlete_id = retained_id AND kept.social_id = duplicate.social_id
          );
        UPDATE athlete_socials SET athlete_id = retained_id
        WHERE athlete_id = ANY(identity.profile_ids);

        DELETE FROM athlete_metric_standings
        WHERE athlete_id = ANY(identity.profile_ids) AND athlete_id <> retained_id;
        DELETE FROM athletes
        WHERE athlete_id = ANY(identity.profile_ids) AND athlete_id <> retained_id;
    END LOOP;
END $$;

UPDATE athletes
SET match_key = pg_temp.fold_letters(match_key)
WHERE match_key <> pg_temp.fold_letters(match_key);

WITH renamed AS (
    SELECT athlete_id, slug AS old_slug,
           pg_temp.fold_letters(slug) AS new_slug
    FROM athletes
    WHERE slug <> pg_temp.fold_letters(slug)
)
UPDATE athletes a
SET slug = renamed.new_slug,
    slug_history = COALESCE(a.slug_history, '[]'::jsonb) || jsonb_build_array(renamed.old_slug)
FROM renamed
WHERE a.athlete_id = renamed.athlete_id
  AND NOT EXISTS (
      SELECT 1 FROM athletes taken
      WHERE taken.slug = renamed.new_slug
         OR COALESCE(taken.slug_history, '[]'::jsonb) @> to_jsonb(renamed.new_slug)
  );
