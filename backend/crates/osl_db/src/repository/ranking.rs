use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::error::Result;
use crate::params::{RankingFilter, RankingMovement};
use crate::projections::ranking::RankingRow;

pub struct RankingRepository<'a> {
    pool: &'a PgPool,
}

fn escape_like(pattern: &str) -> String {
    pattern
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

impl<'a> RankingRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Returns one page of the global ranking plus the unpaginated total.
    pub async fn get_global_ranking(
        &self,
        filter: &RankingFilter,
    ) -> Result<(Vec<RankingRow>, i64)> {
        let total_items = self.count_participants(filter).await?;
        let entries = self.fetch_ranked_entries(filter).await?;

        Ok((entries, total_items))
    }

    fn movement_weights(filter: &RankingFilter) -> QueryBuilder<Postgres> {
        let mut query = QueryBuilder::new(
            r#"
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
                    cat.name as category_name,
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
                    SUM(l.max_weight) as total,
                    cp.ris_score,
                    cp.ris_source
                FROM competition_participants cp
                INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
                INNER JOIN competitions c ON cp.competition_id = c.competition_id
                INNER JOIN lifts l ON cp.participant_id = l.participant_id
                INNER JOIN categories cat ON cp.category_id = cat.category_id
                INNER JOIN federations f ON c.federation_id = f.federation_id
                LEFT JOIN athlete_socials ats
                    ON ats.athlete_id = a.athlete_id
                   AND ats.social_id = (SELECT social_id FROM socials WHERE name = 'instagram')
                WHERE NOT cp.is_disqualified
            "#,
        );

        if let Some(ref gender) = filter.gender {
            query.push(" AND a.gender = ");
            query.push_bind(gender);
        }

        if let Some(ref country) = filter.country {
            query.push(" AND a.country = ");
            query.push_bind(country);
        }

        if let Some(ref name) = filter.name {
            query.push(" AND (a.first_name || ' ' || a.last_name) ILIKE '%' || ");
            query.push_bind(escape_like(name));
            query.push(" || '%' ");
        }

        if let Some(ref category) = filter.category {
            query.push(" AND cat.name LIKE '%' || ");
            query.push_bind(category);
        }

        if let Some(year) = filter.year {
            query.push(" AND EXTRACT(YEAR FROM c.start_date)::int = ");
            query.push_bind(year);
        }

        if let Some(competition_id) = filter.competition_id {
            query.push(" AND c.competition_id = ");
            query.push_bind(competition_id);
        }

        query.push(
            r#"
                GROUP BY cp.participant_id, a.athlete_id, a.first_name, a.last_name,
                         a.slug, a.country, a.gender, cp.bodyweight, cat.name, cp.ris_score, cp.ris_source,
                         c.competition_id, c.name, c.slug, c.start_date, c.event_code, f.name, f.abbreviation,
                         ats.handle
            ),
            "#,
        );

        query
    }

    /// Who belongs in this ranking at all.
    ///
    /// Ranking by one movement spans every event, because a muscle-up is a
    /// muscle-up whatever else the meet ran; it only drops athletes who never
    /// contested it. Ranking by total stays inside a single event, because
    /// adding four lifts and adding one are not the same measurement.
    fn push_eligible(query: &mut QueryBuilder<Postgres>, filter: &RankingFilter) {
        let sort_column = filter.movement.as_column();

        query.push(" eligible AS ( SELECT * FROM movement_weights WHERE ");
        query.push(sort_column);
        query.push(" IS NOT NULL ");

        if filter.movement == RankingMovement::Total {
            query.push(" AND event_code = ");
            query.push_bind(filter.event.clone());
        }

        query.push(" ) ");
    }

    async fn count_participants(&self, filter: &RankingFilter) -> Result<i64> {
        let mut query = Self::movement_weights(filter);
        Self::push_eligible(&mut query, filter);
        query.push(" SELECT COUNT(*) FROM eligible ");

        let count = query
            .build_query_scalar::<i64>()
            .fetch_one(self.pool)
            .await?;

        Ok(count)
    }

    async fn fetch_ranked_entries(&self, filter: &RankingFilter) -> Result<Vec<RankingRow>> {
        let sort_column = filter.movement.as_column();

        let mut query = Self::movement_weights(filter);
        Self::push_eligible(&mut query, filter);

        query.push(" , ranked_movements AS ( SELECT *, ROW_NUMBER() OVER (ORDER BY ");
        query.push(sort_column);
        query.push(" ");
        query.push(filter.direction.as_sql());
        query.push(") as rank FROM eligible ) ");
        query.push(" SELECT * FROM ranked_movements ORDER BY rank LIMIT ");
        query.push_bind(filter.limit);
        query.push(" OFFSET ");
        query.push_bind(filter.offset);

        let rows: Vec<RankingRow> = query.build_query_as().fetch_all(self.pool).await?;

        Ok(rows)
    }

    /// Distinct weight classes, gender-stripped and sorted by weight so the
    /// filter dropdown reads smallest to largest. Men and women don't share
    /// the same classes, so an optional gender narrows the list to what that
    /// gender actually has. An optional competition further narrows it to
    /// the classes actually contested at that meet.
    pub async fn list_distinct_classes(
        &self,
        gender: Option<&str>,
        competition_id: Option<Uuid>,
    ) -> Result<Vec<String>> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT class FROM (
                SELECT DISTINCT split_part(cat.name, ' ', 2) as class
                FROM categories cat
            "#,
        );

        if competition_id.is_some() {
            query.push(
                " INNER JOIN competition_participants cp ON cp.category_id = cat.category_id ",
            );
        }

        let mut has_where = false;

        if let Some(gender) = gender {
            query.push(" WHERE cat.gender = ");
            query.push_bind(gender);
            has_where = true;
        }

        if let Some(competition_id) = competition_id {
            query.push(if has_where { " AND " } else { " WHERE " });
            query.push(" cp.competition_id = ");
            query.push_bind(competition_id);
        }

        query.push(" ) t ORDER BY regexp_replace(class, '[^0-9]', '', 'g')::int ");

        let classes: Vec<String> = query.build_query_scalar().fetch_all(self.pool).await?;

        Ok(classes)
    }

    /// Distinct competition years, most recent first.
    pub async fn list_distinct_years(&self) -> Result<Vec<i32>> {
        let years: Vec<i32> = sqlx::query_scalar(
            r#"
            SELECT DISTINCT EXTRACT(YEAR FROM start_date)::int as year
            FROM competitions
            ORDER BY year DESC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(years)
    }

    /// Distinct countries athletes have actually competed under, alphabetical.
    /// An optional competition narrows it to who competed at that meet.
    pub async fn list_distinct_countries(
        &self,
        competition_id: Option<Uuid>,
    ) -> Result<Vec<String>> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT DISTINCT a.country
            FROM athletes a
            INNER JOIN competition_participants cp ON cp.athlete_id = a.athlete_id
            "#,
        );

        if let Some(competition_id) = competition_id {
            query.push(" WHERE cp.competition_id = ");
            query.push_bind(competition_id);
        }

        query.push(" ORDER BY a.country ");

        let countries: Vec<String> = query.build_query_scalar().fetch_all(self.pool).await?;

        Ok(countries)
    }
}
