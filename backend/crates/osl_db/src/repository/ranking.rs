use osl_domain::{Gender, WeightClass};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use super::ranking_query::{push_eligible, push_latest_total_class, push_metric_candidates, push_name_match, push_ranking_pool, same_class};
use crate::error::Result;
use crate::params::RankingFilter;
use crate::projections::ranking::{AthleteMetricStandingRow, AthleteNeighborRow, RankingRow};

pub struct RankingRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> RankingRepository<'a> {
    #[must_use]
    pub const fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "repository.ranking.get_global_ranking", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn get_global_ranking(
        &self,
        filter: &RankingFilter,
    ) -> Result<(Vec<RankingRow>, i64)> {
        let total_items = self.count_participants(filter).await?;
        let entries = self.fetch_ranked_entries(filter).await?;

        Ok((entries, total_items))
    }

    async fn count_participants(&self, filter: &RankingFilter) -> Result<i64> {
        let mut query = Self::movement_weights(Some(filter));
        push_eligible(&mut query, filter);
        push_ranking_pool(&mut query, filter);
        query.push(" SELECT COUNT(*) FROM ranking_pool ");
        push_name_match(&mut query, filter);

        let count = query
            .build_query_scalar::<i64>()
            .fetch_one(self.pool)
            .await?;

        Ok(count)
    }

    async fn fetch_ranked_entries(&self, filter: &RankingFilter) -> Result<Vec<RankingRow>> {
        let sort_column = filter.movement.as_column();

        let mut query = Self::movement_weights(Some(filter));
        push_eligible(&mut query, filter);
        push_ranking_pool(&mut query, filter);

        query.push(" , ranked_movements AS ( SELECT *, ROW_NUMBER() OVER (ORDER BY ");
        query.push(sort_column);
        query.push(" ");
        query.push(filter.direction.as_sql());
        query.push(", athlete_id, participant_id) as rank FROM ranking_pool ) ");
        query.push(" SELECT * FROM ranked_movements ");
        push_name_match(&mut query, filter);
        query.push(" ORDER BY rank LIMIT ");
        query.push_bind(filter.limit);
        query.push(" OFFSET ");
        query.push_bind(filter.offset);

        let rows: Vec<RankingRow> = query.build_query_as().fetch_all(self.pool).await?;

        Ok(rows)
    }

    #[tracing::instrument(name = "repository.ranking.get_athlete_metric_standings", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn get_athlete_metric_standings(
        &self,
        athlete_id: Uuid,
    ) -> Result<Vec<AthleteMetricStandingRow>> {
        let mut query = Self::movement_weights(None);
        push_latest_total_class(&mut query, athlete_id);
        push_metric_candidates(&mut query);
        query.push(
            r"
            mine AS (
                SELECT DISTINCT ON (metric)
                    metric, weight_class_id, gender, weight_class_min, weight_class_max
                FROM metric_candidates
                WHERE (metric = 'ris' OR weight_class_id IS NOT NULL)
                  AND (metric <> 'total' OR EXISTS (
                      SELECT 1 FROM latest_total_class latest WHERE ",
        );
        query.push(same_class("metric_candidates", "latest"));
        query.push(
            r"
                  ))
                  AND athlete_id =
            ",
        );
        query.push_bind(athlete_id);
        query.push(
            r"
                ORDER BY metric, value DESC, start_date DESC, participant_id
            ),
            comparable AS (
                SELECT candidate.*
                FROM metric_candidates candidate
                INNER JOIN mine
                    ON mine.metric = candidate.metric
                   AND (candidate.metric <> 'total' OR candidate.gender = mine.gender)
                   AND (
                       candidate.metric = 'ris'
                       OR (candidate.metric = 'total' AND ",
        );
        query.push(same_class("candidate", "mine"));
        query.push(
            r")
                       OR (candidate.metric <> 'total' AND mine.weight_class_id = candidate.weight_class_id)
                   )
            ),
            ranking_pool AS (
                SELECT DISTINCT ON (metric, athlete_id)
                    metric, athlete_id, country, value, weight_class_min, weight_class_max
                FROM comparable
                ORDER BY metric, athlete_id, value DESC, start_date DESC, participant_id
            ),
            placed AS (
                SELECT
                    metric,
                    athlete_id,
                    country,
                    value,
                    weight_class_min,
                    weight_class_max,
                    ROW_NUMBER() OVER (PARTITION BY metric ORDER BY value DESC, athlete_id) AS global_place,
                    COUNT(*) OVER (PARTITION BY metric) AS global_field,
                    ROW_NUMBER() OVER (PARTITION BY metric, country ORDER BY value DESC, athlete_id) AS country_place,
                    COUNT(*) OVER (PARTITION BY metric, country) AS country_field
                FROM ranking_pool
            )
            SELECT metric, value, weight_class_min, weight_class_max,
                   global_place, global_field, country, country_place, country_field
            FROM placed
            WHERE athlete_id =
            ",
        );
        query.push_bind(athlete_id);
        query.push(" ORDER BY metric ");

        let standings: Vec<AthleteMetricStandingRow> =
            query.build_query_as().fetch_all(self.pool).await?;

        Ok(standings)
    }

    /// Closest total on each side among athletes whose latest All4 category matches, including the athlete.
    pub async fn get_athlete_total_neighbors(
        &self,
        athlete_id: Uuid,
    ) -> Result<Vec<AthleteNeighborRow>> {
        let mut query = Self::movement_weights(None);
        push_latest_total_class(&mut query, athlete_id);
        super::ranking_query::push_latest_total_classes(&mut query);
        query.push(
            r"
            eligible AS (
                SELECT mw.* FROM movement_weights mw
                JOIN latest_total_class latest ON latest.gender = mw.gender AND ",
        );
        query.push(same_class("latest", "mw"));
        query.push(
            r"
                JOIN latest_total_classes current ON current.athlete_id = mw.athlete_id AND ",
        );
        query.push(same_class("current", "latest"));
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
                    ROW_NUMBER() OVER (ORDER BY total DESC, athlete_id) AS global_place,
                    COUNT(*) OVER () AS global_field,
                    ROW_NUMBER() OVER (PARTITION BY country ORDER BY total DESC, athlete_id) AS country_place,
                    COUNT(*) OVER (PARTITION BY country) AS country_field
                FROM ranking_pool
            ),
            target AS (SELECT * FROM placed WHERE athlete_id = ");
        query.push_bind(athlete_id);
        query.push(
            r"),
            neighbors AS (
                SELECT p.*, 'global' AS scope, p.global_place AS place,
                       p.global_field AS field, p.total - t.total AS gap
                FROM placed p CROSS JOIN target t
                WHERE p.global_place BETWEEN t.global_place - 1 AND t.global_place + 1
                UNION ALL
                SELECT p.*, 'national' AS scope, p.country_place AS place,
                       p.country_field AS field, p.total - t.total AS gap
                FROM placed p CROSS JOIN target t
                WHERE t.country IS NOT NULL AND p.country = t.country
                  AND p.country_place BETWEEN t.country_place - 1 AND t.country_place + 1
            )
            SELECT scope, athlete_id, first_name, last_name, slug, country,
                   weight_class_min, weight_class_max, total AS value, gap, place, field,
                   competition_name, competition_slug, start_date AS competition_date
            FROM neighbors ORDER BY scope, place
        ",
        );
        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }

    pub async fn get_athlete_metric_standings_stored(
        &self,
        athlete_id: Uuid,
    ) -> Result<Vec<AthleteMetricStandingRow>> {
        let standings: Vec<AthleteMetricStandingRow> = sqlx::query_as(
            r#"
            SELECT metric, value, weight_class_min, weight_class_max,
                   global_place, global_field, country, country_place, country_field
            FROM athlete_metric_standings
            WHERE athlete_id = $1
            ORDER BY metric
            "#,
        )
        .bind(athlete_id)
        .fetch_all(self.pool)
        .await?;
        Ok(standings)
    }

    pub async fn get_athlete_total_neighbors_stored(
        &self,
        athlete_id: Uuid,
    ) -> Result<Vec<AthleteNeighborRow>> {
        let neighbors: Vec<AthleteNeighborRow> = sqlx::query_as(
            r#"
            SELECT scope, neighbor_athlete_id AS athlete_id, first_name, last_name, slug,
                   country, weight_class_min, weight_class_max, value, gap, place, field,
                   competition_name, competition_slug, competition_date
            FROM athlete_rivals
            WHERE profile_athlete_id = $1
            ORDER BY scope, place
            "#,
        )
        .bind(athlete_id)
        .fetch_all(self.pool)
        .await?;
        Ok(neighbors)
    }

    fn movement_weights(filter: Option<&RankingFilter>) -> QueryBuilder<Postgres> {
        super::ranking_query::movement_weights(filter)
    }

    #[tracing::instrument(name = "repository.ranking.list_distinct_classes", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn list_distinct_classes(
        &self,
        gender: Option<Gender>,
        competition_id: Option<Uuid>,
    ) -> Result<Vec<String>> {
        let mut query = QueryBuilder::new(
            r"
            SELECT min_kg, max_kg FROM (
                SELECT DISTINCT wc.min_kg, wc.max_kg
                FROM competition_participants cp
                INNER JOIN weight_classes wc ON wc.weight_class_id = cp.weight_class_id
                INNER JOIN athletes a ON a.athlete_id = cp.athlete_id
            ",
        );

        if let Some(gender) = gender {
            query.push(if competition_id.is_some() {
                " WHERE COALESCE(cp.category_gender, wc.gender, a.gender) = "
            } else {
                " WHERE a.gender = "
            });
            query.push_bind(gender.as_str());
        }

        if let Some(competition_id) = competition_id {
            query.push(if gender.is_some() { " AND " } else { " WHERE " });
            query.push(" cp.competition_id = ");
            query.push_bind(competition_id);
        }

        query.push(" ) t ORDER BY COALESCE(max_kg, min_kg), max_kg NULLS LAST ");

        let bounds: Vec<(Option<Decimal>, Option<Decimal>)> =
            query.build_query_as().fetch_all(self.pool).await?;

        let mut classes: Vec<String> = bounds
            .into_iter()
            .filter_map(|(min, max)| WeightClass::of(min, max))
            .map(|class| class.to_string())
            .collect();

        classes.dedup();

        Ok(classes)
    }

    #[tracing::instrument(name = "repository.ranking.list_distinct_federations", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn list_distinct_federations(&self) -> Result<Vec<String>> {
        let federations: Vec<String> = sqlx::query_scalar(
            r"
            SELECT DISTINCT f.name
            FROM federations f
            INNER JOIN competitions c ON c.federation_id = f.federation_id
            INNER JOIN competition_participants cp ON cp.competition_id = c.competition_id
            ORDER BY f.name
            ",
        )
        .fetch_all(self.pool)
        .await?;

        Ok(federations)
    }

    #[tracing::instrument(name = "repository.ranking.list_distinct_years", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn list_distinct_years(&self) -> Result<Vec<i32>> {
        let years: Vec<i32> = sqlx::query_scalar(
            r"
            SELECT DISTINCT EXTRACT(YEAR FROM start_date)::int as year
            FROM competitions
            ORDER BY year DESC
            ",
        )
        .fetch_all(self.pool)
        .await?;

        Ok(years)
    }

    #[tracing::instrument(name = "repository.ranking.list_distinct_countries", skip_all, fields(db.system.name = "postgresql"))]
    pub async fn list_distinct_countries(
        &self,
        competition_id: Option<Uuid>,
    ) -> Result<Vec<String>> {
        let mut query = QueryBuilder::new(
            r"
            SELECT DISTINCT a.country
            FROM athletes a
            INNER JOIN competition_participants cp ON cp.athlete_id = a.athlete_id
            WHERE a.country IS NOT NULL
            ",
        );

        if let Some(competition_id) = competition_id {
            query.push(" AND cp.competition_id = ");
            query.push_bind(competition_id);
        }

        query.push(" ORDER BY a.country ");

        let countries: Vec<String> = query.build_query_scalar().fetch_all(self.pool).await?;

        Ok(countries)
    }
}
