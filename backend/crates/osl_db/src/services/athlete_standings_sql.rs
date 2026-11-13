use crate::repository::ranking_query::{
    push_latest_total_classes, push_metric_candidates, same_class,
};
use sqlx::{Postgres, QueryBuilder};

/// Appends the metric-candidate pipeline and an INSERT that fills
/// `athlete_metric_standings` for every athlete. Must be appended right after
/// `ranking_query::movement_weights(None)`.
pub(super) fn push_metric_pool(query: &mut QueryBuilder<Postgres>) {
    push_metric_candidates(query);
    push_latest_total_classes(query);

    query.push(
        r"
            mine AS (
                SELECT DISTINCT ON (metric, athlete_id)
                    metric, athlete_id, country, gender, value,
                    weight_class_id, weight_class_min, weight_class_max,
                    CASE
                        WHEN metric = 'ris' THEN 'ris'
                        WHEN metric = 'total' THEN gender || '|' || COALESCE(weight_class_min::text, '') || '|' || COALESCE(weight_class_max::text, '')
                        ELSE weight_class_id::text
                    END AS class_group
                FROM metric_candidates candidate
                WHERE (candidate.metric = 'ris' OR candidate.weight_class_id IS NOT NULL)
                  AND (candidate.metric <> 'total' OR EXISTS (
                      SELECT 1 FROM latest_total_classes latest
                      WHERE latest.athlete_id = candidate.athlete_id
                        AND latest.gender = candidate.gender
                        AND ",
    );
    query.push(same_class("latest", "candidate"));
    query.push(
        r"
                  ))
                ORDER BY metric, athlete_id, value DESC, start_date DESC, participant_id
            ),
            pool AS (
                SELECT DISTINCT ON (metric, class_group, athlete_id)
                    metric, class_group, athlete_id, country, value,
                    weight_class_min, weight_class_max
                FROM (
                    SELECT candidate.metric, candidate.athlete_id, candidate.country,
                           candidate.value, candidate.start_date, candidate.participant_id,
                           candidate.weight_class_id,
                           candidate.weight_class_min, candidate.weight_class_max,
                           CASE
                               WHEN candidate.metric = 'ris' THEN 'ris'
                               WHEN candidate.metric = 'total' THEN candidate.gender || '|' || COALESCE(candidate.weight_class_min::text, '') || '|' || COALESCE(candidate.weight_class_max::text, '')
                               ELSE candidate.weight_class_id::text
                           END AS class_group
                    FROM metric_candidates candidate
                    WHERE candidate.metric = 'ris' OR candidate.weight_class_id IS NOT NULL
                ) scoped
                ORDER BY metric, class_group, athlete_id, value DESC, start_date DESC, participant_id
            ),
            placed AS (
                SELECT metric, class_group, athlete_id, country, value,
                       ROW_NUMBER() OVER (PARTITION BY metric, class_group ORDER BY value DESC, athlete_id) AS global_place,
                       COUNT(*) OVER (PARTITION BY metric, class_group) AS global_field,
                       ROW_NUMBER() OVER (PARTITION BY metric, class_group, country ORDER BY value DESC, athlete_id) AS country_place,
                       COUNT(*) OVER (PARTITION BY metric, class_group, country) AS country_field
                FROM pool
            )
            INSERT INTO athlete_metric_standings (
                metric, athlete_id, value, global_place, global_field,
                country, country_place, country_field,
                weight_class_id, weight_class_min, weight_class_max
            )
            SELECT mine.metric, mine.athlete_id, mine.value,
                   placed.global_place, placed.global_field,
                   mine.country, placed.country_place, placed.country_field,
                   mine.weight_class_id, mine.weight_class_min, mine.weight_class_max
            FROM mine
            JOIN placed ON placed.metric = mine.metric
                       AND placed.athlete_id = mine.athlete_id
                       AND placed.class_group = mine.class_group
        ",
    );
}

/// Appends the rivals pipeline and an INSERT that fills `athlete_rivals` for
/// every athlete. Must be appended right after
/// `ranking_query::movement_weights(None)`.
pub(super) fn push_rivals(query: &mut QueryBuilder<Postgres>) {
    push_latest_total_classes(query);
    query.push(
        r"
            eligible AS (
                SELECT mw.* FROM movement_weights mw
                JOIN latest_total_classes latest ON latest.athlete_id = mw.athlete_id AND ",
    );
    query.push(same_class("latest", "mw"));
    query.push(" WHERE mw.total IS NOT NULL AND mw.event_code = ");
    query.push_bind(osl_domain::FULL_EVENT);
    query.push(
        r"
            ),
            ranking_pool AS (
                SELECT DISTINCT ON (athlete_id) * FROM eligible
                ORDER BY athlete_id, total DESC, start_date DESC, participant_id
            ),
            placed AS (
                SELECT *,
                    ROW_NUMBER() OVER (PARTITION BY gender, weight_class_min, weight_class_max ORDER BY total DESC, athlete_id) AS global_place,
                    COUNT(*) OVER (PARTITION BY gender, weight_class_min, weight_class_max) AS global_field,
                    ROW_NUMBER() OVER (PARTITION BY gender, weight_class_min, weight_class_max, country ORDER BY total DESC, athlete_id) AS country_place,
                    COUNT(*) OVER (PARTITION BY gender, weight_class_min, weight_class_max, country) AS country_field
                FROM ranking_pool
            )
            INSERT INTO athlete_rivals (
                profile_athlete_id, scope, place, field, neighbor_athlete_id,
                first_name, last_name, slug, country,
                weight_class_min, weight_class_max, value, gap,
                competition_name, competition_slug, competition_date
            )
            SELECT target.athlete_id, 'global', p.global_place, p.global_field,
                   p.athlete_id, p.first_name, p.last_name, p.slug, p.country,
                   p.weight_class_min, p.weight_class_max, p.total, p.total - target.total,
                   p.competition_name, p.competition_slug, p.start_date
            FROM placed target
            JOIN placed p ON p.gender = target.gender
                         AND p.weight_class_min IS NOT DISTINCT FROM target.weight_class_min
                         AND p.weight_class_max IS NOT DISTINCT FROM target.weight_class_max
                         AND p.global_place BETWEEN target.global_place - 1 AND target.global_place + 1
            UNION ALL
            SELECT target.athlete_id, 'national', p.country_place, p.country_field,
                   p.athlete_id, p.first_name, p.last_name, p.slug, p.country,
                   p.weight_class_min, p.weight_class_max, p.total, p.total - target.total,
                   p.competition_name, p.competition_slug, p.start_date
            FROM placed target
            JOIN placed p ON p.gender = target.gender
                         AND p.weight_class_min IS NOT DISTINCT FROM target.weight_class_min
                         AND p.weight_class_max IS NOT DISTINCT FROM target.weight_class_max
                         AND target.country IS NOT NULL AND p.country = target.country
                         AND p.country_place BETWEEN target.country_place - 1 AND target.country_place + 1
        ",
    );
}
