use sqlx::{Postgres, QueryBuilder};

use osl_domain::{Movement, WeightClass};
use uuid::Uuid;

use crate::filter::Predicate;
use crate::params::{RankingBy, RankingFilter, RankingMovement};

fn escape_like(pattern: &str) -> String {
    pattern
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

impl Predicate for RankingBy {
    fn push(&self, query: &mut QueryBuilder<Postgres>) {
        match self {
            Self::Sex(gender) => query.push("a.gender = ").push_bind(gender.as_str()),
            Self::CategoryGender(gender) => query
                .push("COALESCE(cp.category_gender, wc.gender, a.gender) = ")
                .push_bind(gender.as_str()),
            Self::Country(country) => query.push("a.country = ").push_bind(country.clone()),
            Self::Federation(federation) => query.push("f.name = ").push_bind(federation.clone()),
            Self::Series(series) => query.push("c.series = ").push_bind(series.clone()),
            Self::Class(WeightClass::UpTo(max)) => query.push("wc.max_kg = ").push_bind(*max),
            Self::Class(WeightClass::Above(min)) => query
                .push("wc.max_kg IS NULL AND wc.min_kg = ")
                .push_bind(*min),
            Self::Year(year) => query
                .push("EXTRACT(YEAR FROM c.start_date)::int = ")
                .push_bind(*year),
            Self::Competition(id) => query.push("c.competition_id = ").push_bind(*id),
            Self::Division(name) => query
                .push("cp.division_id IN (SELECT division_id FROM divisions WHERE name = ")
                .push_bind(name.clone())
                .push(")"),
        };
    }
}

fn scored_on(movement: RankingMovement) -> String {
    let lifted = |movement: Movement| {
        format!(
            "EXISTS (SELECT 1 FROM lifts scored \
             WHERE scored.participant_id = cp.participant_id \
               AND scored.movement_name = '{}')",
            movement.as_str()
        )
    };

    match movement {
        RankingMovement::Ris => "cp.ris_score IS NOT NULL".to_owned(),
        RankingMovement::Total => "cp.total IS NOT NULL".to_owned(),
        RankingMovement::Muscleup => lifted(Movement::MuscleUp),
        RankingMovement::Pullup => lifted(Movement::PullUp),
        RankingMovement::Dips => lifted(Movement::Dips),
        RankingMovement::Squat => lifted(Movement::Squat),
    }
}

fn push_ranked_on_metric(query: &mut QueryBuilder<Postgres>, filter: &RankingFilter) {
    query.push(" AND ");
    query.push(scored_on(filter.movement));

    if matches!(
        filter.movement,
        RankingMovement::Total | RankingMovement::Ris
    ) && !filter.in_competition()
    {
        query.push(" AND c.event_code = ");
        query.push_bind(filter.event.clone());
    }
}

pub(crate) fn ranked_results(
    select: &str,
    filter: &RankingFilter,
    own: impl Fn(&RankingBy) -> bool,
) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(select);
    query.push(
        r"
            FROM competition_participants cp
            INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
            INNER JOIN competitions c ON cp.competition_id = c.competition_id
            INNER JOIN federations f ON c.federation_id = f.federation_id
            LEFT JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
            WHERE TRUE
        ",
    );
    if !filter.in_competition() {
        query.push(" AND cp.status = 'competed' ");
        push_ranked_on_metric(&mut query, filter);
    }
    filter.by.without(own).push_and(&mut query);
    query
}

pub(crate) fn same_class(left: &str, right: &str) -> String {
    format!(
        "({left}.weight_class_max = {right}.weight_class_max \
         OR ({left}.weight_class_max IS NULL AND {right}.weight_class_max IS NULL \
             AND {left}.weight_class_min = {right}.weight_class_min))"
    )
}

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
        filter.by.push_and(&mut query);
        push_ranked_on_metric(&mut query, filter);
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

pub(crate) fn push_ranking_pool(query: &mut QueryBuilder<Postgres>, filter: &RankingFilter) {
    if filter.in_competition() {
        query.push(" ranking_pool AS ( SELECT * FROM movement_weights ) ");
        return;
    }

    query.push(" ranking_pool AS ( SELECT DISTINCT ON (athlete_id) * FROM movement_weights ORDER BY athlete_id, ");
    query.push(filter.movement.as_column());
    query.push(" DESC NULLS LAST");
    if filter.movement == RankingMovement::Total {
        query.push(", bodyweight ASC NULLS LAST");
    }
    query.push(", start_date DESC, participant_id ) ");
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

        // Global total and RIS standings compare the MPDS event only.
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

pub(crate) fn push_latest_total_class(query: &mut QueryBuilder<Postgres>, athlete_id: Uuid) {
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
