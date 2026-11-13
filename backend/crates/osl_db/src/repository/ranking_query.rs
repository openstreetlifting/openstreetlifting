//! Shared SQL fragments for ranking computations.
//!
//! One source of truth for the `movement_weights` pipeline: the live global
//! ranking page, the athlete standings read path, and the import-time
//! standings recompute all build on these fragments. Keeping them here stops
//! the three callers from forking into divergent ranking semantics.

use sqlx::{Postgres, QueryBuilder};

use osl_domain::WeightClass;
use uuid::Uuid;

use crate::params::{RankingFilter, RankingMovement};

fn escape_like(pattern: &str) -> String {
    pattern
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

pub(crate) fn same_class(left: &str, right: &str) -> String {
    format!(
        "({left}.weight_class_max = {right}.weight_class_max \
         OR ({left}.weight_class_max IS NULL AND {right}.weight_class_max IS NULL \
             AND {left}.weight_class_min = {right}.weight_class_min))"
    )
}

/// Aggregates each participant's best lift per movement. With no filter this
/// scans every competed participant in the database; callers that only need a
/// subset should always pass a filter.
pub(crate) fn movement_weights(filter: Option<&RankingFilter>) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        r"
            WITH movement_weights AS (
                SELECT
                    cp.participant_id,
                    a.athlete_id,
                    a.first_name,
                    a.last_name,
                    a.slug,
                    a.country,
                    a.gender,
                    cp.bodyweight,
                    d.name as division,
                    cp.weight_class_id,
                    wc.min_kg as weight_class_min,
                    wc.max_kg as weight_class_max,
                    c.competition_id,
                    c.name as competition_name,
                    c.slug as competition_slug,
                    c.start_date,
                    c.event_code,
                    f.name as federation_name,
                    f.abbreviation as federation_abbreviation,
                    ats.handle as instagram_handle,
                    MAX(CASE WHEN l.movement_name = 'Muscle-up' THEN l.max_weight END) as muscleup,
                    MAX(CASE WHEN l.movement_name = 'Pull-up' THEN l.max_weight END) as pullup,
                    MAX(CASE WHEN l.movement_name = 'Dips' THEN l.max_weight END) as dips,
                    MAX(CASE WHEN l.movement_name = 'Squat' THEN l.max_weight END) as squat,
                    cp.total,
                    cp.ris_score,
                    cp.ris_source
                FROM competition_participants cp
                INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
                INNER JOIN competitions c ON cp.competition_id = c.competition_id
                LEFT JOIN lifts l ON cp.participant_id = l.participant_id
                LEFT JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
                LEFT JOIN divisions d ON d.division_id = cp.division_id
                INNER JOIN federations f ON c.federation_id = f.federation_id
                LEFT JOIN athlete_socials ats
                    ON ats.athlete_id = a.athlete_id
                   AND ats.social_id = (SELECT social_id FROM socials WHERE name = 'instagram')
                WHERE cp.status = 'competed'
                  AND (l.lift_id IS NOT NULL OR cp.total IS NOT NULL OR cp.ris_score IS NOT NULL)
            ",
    );

    if let Some(filter) = filter {
        if let Some(gender) = filter.gender {
            query.push(if filter.competition_id.is_some() {
                " AND COALESCE(cp.category_gender, wc.gender, a.gender) = "
            } else {
                " AND a.gender = "
            });
            query.push_bind(gender.as_str());
        }

        if let Some(ref country) = filter.country {
            query.push(" AND a.country = ");
            query.push_bind(country);
        }

        if let Some(ref federation) = filter.federation {
            query.push(" AND f.name = ");
            query.push_bind(federation);
        }

        match filter.category {
            Some(WeightClass::UpTo(max)) => {
                query.push(" AND wc.max_kg = ");
                query.push_bind(max);
            }
            Some(WeightClass::Above(min)) => {
                query.push(" AND wc.max_kg IS NULL AND wc.min_kg = ");
                query.push_bind(min);
            }
            None => {}
        }

        if let Some(year) = filter.year {
            query.push(" AND EXTRACT(YEAR FROM c.start_date)::int = ");
            query.push_bind(year);
        }

        if let Some(competition_id) = filter.competition_id {
            query.push(" AND c.competition_id = ");
            query.push_bind(competition_id);
        }
    }

    query.push(
        r"
                GROUP BY cp.participant_id, a.athlete_id, a.first_name, a.last_name,
                         a.slug, a.country, a.gender, cp.bodyweight, d.name, cp.weight_class_id,
                         wc.min_kg, wc.max_kg,
                         cp.ris_score, cp.ris_source,
                         c.competition_id, c.name, c.slug, c.start_date, c.event_code, f.name, f.abbreviation,
                         ats.handle
            ),
            ",
    );

    query
}

pub(crate) fn push_eligible(query: &mut QueryBuilder<Postgres>, filter: &RankingFilter) {
    let sort_column = filter.movement.as_column();

    query.push(" eligible AS ( SELECT * FROM movement_weights WHERE ");
    query.push(sort_column);
    query.push(" IS NOT NULL ");

    if matches!(
        filter.movement,
        RankingMovement::Total | RankingMovement::Ris
    ) && filter.competition_id.is_none()
    {
        query.push(" AND event_code = ");
        query.push_bind(filter.event.clone());
    }

    query.push(" ) ");
}

pub(crate) fn push_ranking_pool(query: &mut QueryBuilder<Postgres>, filter: &RankingFilter) {
    if filter.competition_id.is_some() {
        query.push(" , ranking_pool AS ( SELECT * FROM eligible ) ");
        return;
    }

    query.push(" , ranking_pool AS ( SELECT DISTINCT ON (athlete_id) * FROM eligible ORDER BY athlete_id, ");
    query.push(filter.movement.as_column());
    query.push(" DESC NULLS LAST, start_date DESC, participant_id ) ");
}

pub(crate) fn push_name_match(query: &mut QueryBuilder<Postgres>, filter: &RankingFilter) {
    let Some(name) = filter.name.as_ref() else {
        return;
    };

    query.push(" WHERE (first_name || ' ' || last_name) ILIKE '%' || ");
    query.push_bind(escape_like(name));
    query.push(" || '%' ");
}

pub(crate) fn push_metric_candidates(query: &mut QueryBuilder<Postgres>) {
    query.push(
        r"
            metric_candidates AS (
                SELECT movement_weights.athlete_id,
                       movement_weights.participant_id,
                       movement_weights.start_date,
                       movement_weights.country,
                       movement_weights.gender,
                       movement_weights.weight_class_id,
                       movement_weights.weight_class_min,
                       movement_weights.weight_class_max,
                       metric.name AS metric,
                       metric.value
                FROM movement_weights
                CROSS JOIN LATERAL ( VALUES
            ",
    );

    for (index, movement) in RankingMovement::ALL.into_iter().enumerate() {
        if index > 0 {
            query.push(", ");
        }

        query.push(format!("('{}', ", movement.as_str()));

        // Global total and RIS standings compare the All4 event only.
        if matches!(movement, RankingMovement::Total | RankingMovement::Ris) {
            query.push("CASE WHEN event_code = ");
            query.push_bind(osl_domain::FULL_EVENT);
            query.push(format!(" THEN {} END)", movement.as_column()));
        } else {
            query.push(format!("{})", movement.as_column()));
        }
    }

    query.push(
        r"
                ) AS metric(name, value)
                WHERE metric.value IS NOT NULL
            ),
        ",
    );
}

fn push_latest_total_class_from(query: &mut QueryBuilder<Postgres>) {
    query.push(
        r"
                FROM competition_participants cp
                JOIN competitions c USING (competition_id)
                JOIN athletes a USING (athlete_id)
                JOIN weight_classes wc USING (weight_class_id)
                WHERE cp.status = 'competed' AND c.event_code = ",
    );
    query.push_bind(osl_domain::FULL_EVENT);
}

pub(crate) fn push_latest_total_class(
    query: &mut QueryBuilder<Postgres>,
    athlete_id: Uuid,
) {
    query.push(
        r"
            latest_total_class AS (
                SELECT a.gender, wc.min_kg AS weight_class_min, wc.max_kg AS weight_class_max",
    );
    push_latest_total_class_from(query);
    query.push(" AND cp.athlete_id = ");
    query.push_bind(athlete_id);
    query.push(
        r"
                ORDER BY c.start_date DESC NULLS LAST, cp.participant_id
                LIMIT 1
            ),
        ",
    );
}

pub(crate) fn push_latest_total_classes(query: &mut QueryBuilder<Postgres>) {
    query.push(
        r"
            latest_total_classes AS (
                SELECT DISTINCT ON (cp.athlete_id)
                    cp.athlete_id, a.gender, wc.min_kg AS weight_class_min, wc.max_kg AS weight_class_max",
    );
    push_latest_total_class_from(query);
    query.push(
        r"
                ORDER BY cp.athlete_id, c.start_date DESC NULLS LAST, cp.participant_id
            ),
        ",
    );
}
