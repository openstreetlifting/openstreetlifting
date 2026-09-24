-- Merge duplicate profiles for these seven athletes.
-- Prefer the profile with the corrected country and preserve old URLs.
DO $$
DECLARE
    confirmed RECORD;
    duplicate athletes%ROWTYPE;
    profile_ids UUID[];
    retained_id UUID;
BEGIN
    FOR confirmed IN
        SELECT * FROM (VALUES
            ('harry twister', 'M', 'BA', 'SE', 'BA'),
            ('denilson monteiro', 'M', 'DE', 'FR', 'FR'),
            ('giuseppe cicero', 'M', 'IT', 'SM', 'IT'),
            ('lorenzo giorgetti', 'M', 'IT', 'SM', 'IT'),
            ('jacopo bartoli', 'M', 'IT', 'SM', 'IT'),
            ('ilaria valentini', 'F', 'IT', 'SM', 'IT'),
            ('tony nguyen', 'M', 'FR', 'US', 'US')
        ) AS reviewed(match_key, gender, first_country, second_country, country)
    LOOP
        SELECT array_agg(athlete_id ORDER BY
            (disambiguation IS NULL) DESC,
            (country = confirmed.country) DESC, created_at, athlete_id)
        INTO profile_ids
        FROM athletes
        WHERE match_key = confirmed.match_key AND gender = confirmed.gender
          AND country IN (confirmed.first_country, confirmed.second_country)
          AND (disambiguation IS NULL
               OR (country = confirmed.first_country AND disambiguation = 1)
               OR (country = confirmed.second_country AND disambiguation = 2));

        IF profile_ids IS NULL THEN
            CONTINUE;
        END IF;
        retained_id := profile_ids[1];

        IF EXISTS (
            SELECT social_id FROM athlete_socials
            WHERE athlete_id = ANY(profile_ids)
            GROUP BY social_id HAVING COUNT(DISTINCT handle) > 1
        ) THEN
            RAISE EXCEPTION 'Conflicting social accounts for %; resolve before merging',
                confirmed.match_key;
        END IF;

        UPDATE athletes retained
        SET slug_history = COALESCE((
            SELECT jsonb_agg(DISTINCT old_slug ORDER BY old_slug)
            FROM athletes a
            CROSS JOIN LATERAL jsonb_array_elements_text(
                COALESCE(a.slug_history, '[]'::jsonb) || jsonb_build_array(a.slug)
            ) AS history(old_slug)
            WHERE a.athlete_id = ANY(profile_ids) AND old_slug <> retained.slug
        ), '[]'::jsonb)
        WHERE retained.athlete_id = retained_id;

        FOR duplicate IN
            SELECT * FROM athletes
            WHERE athlete_id = ANY(profile_ids) AND athlete_id <> retained_id
        LOOP
            UPDATE athletes SET
                native_script = CASE WHEN native_name IS NULL THEN duplicate.native_script
                                     ELSE native_script END,
                native_name = COALESCE(native_name, duplicate.native_name),
                profile_picture_url = COALESCE(profile_picture_url, duplicate.profile_picture_url),
                created_at = LEAST(created_at, duplicate.created_at)
            WHERE athlete_id = retained_id;
        END LOOP;

        -- A duplicate result violates the existing unique constraint and aborts.
        UPDATE competition_participants
        SET athlete_id = retained_id, country = confirmed.country
        WHERE athlete_id = ANY(profile_ids);
        UPDATE records SET athlete_id = retained_id WHERE athlete_id = ANY(profile_ids);
        UPDATE athlete_socials SET athlete_id = retained_id WHERE athlete_id = ANY(profile_ids);

        DELETE FROM athletes WHERE athlete_id = ANY(profile_ids) AND athlete_id <> retained_id;
        UPDATE athletes SET country = confirmed.country, disambiguation = NULL
        WHERE athlete_id = retained_id;
    END LOOP;
END $$;
