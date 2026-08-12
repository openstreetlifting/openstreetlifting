use super::models::*;
use crate::{ImporterError, Result};
use osl_domain::NormalizedAthleteName;
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

pub struct CanonicalTransformer<'a> {
    pool: &'a PgPool,
}

/// Every row the file just wrote, so anything else belonging to the
/// competition can be recognised as absent and removed.
#[derive(Default)]
struct ImportedFacts {
    participants: Vec<Uuid>,
    lifts: Vec<Uuid>,
    attempts: Vec<Uuid>,
}

impl<'a> CanonicalTransformer<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn import_to_database(&self, canonical: CanonicalFormat) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let competition_id = self
            .upsert_competition(&canonical.competition, &mut tx)
            .await?;

        self.upsert_competition_movements(competition_id, &canonical.movements, &mut tx)
            .await?;

        let mut imported = ImportedFacts::default();

        for category in &canonical.categories {
            let category_id = self.upsert_category(category, &mut tx).await?;

            for athlete in &category.athletes {
                self.import_athlete_performance(
                    athlete,
                    category,
                    competition_id,
                    category_id,
                    &mut imported,
                    &mut tx,
                )
                .await?;
            }
        }

        self.prune_absent_facts(competition_id, &canonical, &imported, &mut tx)
            .await?;

        // After the prune, so the event reflects the movements that survived.
        let event_code = self.refresh_event_code(competition_id, &mut tx).await?;

        if osl_domain::is_full_event(event_code.as_deref()) {
            info!("Computing RIS scores for all participants...");
            self.compute_ris_for_competition(
                competition_id,
                canonical.competition.start_date,
                &mut tx,
            )
            .await?;
        } else {
            info!(
                "Event {} is not the full four movements, so no RIS is computed",
                event_code.as_deref().unwrap_or("(none)")
            );
        }

        tx.commit().await?;
        Ok(())
    }

    /// Rewrites the competition's event from the movements it actually has, so
    /// a file that drops a movement changes the event rather than leaving a
    /// stale one behind.
    async fn refresh_event_code(
        &self,
        competition_id: Uuid,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Option<String>> {
        let event_code = sqlx::query_scalar!(
            r#"
            UPDATE competitions
            SET event_code = (
                SELECT string_agg(m.code, '' ORDER BY m.display_order)
                FROM competition_movements cm
                JOIN movements m ON m.name = cm.movement_name
                WHERE cm.competition_id = $1
            )
            WHERE competition_id = $1
            RETURNING event_code
            "#,
            competition_id
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(event_code)
    }

    /// A canonical file is the whole truth about its competition, so a row it
    /// no longer lists has been corrected away and must go. Only facts the
    /// competition owns are pruned: athletes, categories, weight classes,
    /// movements and federations are shared between competitions and stay.
    ///
    /// Order matters. Participants go first and take their lifts, attempts and
    /// RIS history with them through the foreign keys, which leaves the later
    /// statements looking only at the participants and lifts that survived.
    async fn prune_absent_facts(
        &self,
        competition_id: Uuid,
        canonical: &CanonicalFormat,
        imported: &ImportedFacts,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        let movement_names: Vec<String> = canonical
            .movements
            .iter()
            .map(|movement| movement.name.clone())
            .collect();

        let movements = sqlx::query!(
            r#"
            DELETE FROM competition_movements
            WHERE competition_id = $1 AND movement_name <> ALL($2::text[])
            "#,
            competition_id,
            &movement_names
        )
        .execute(&mut **tx)
        .await?
        .rows_affected();

        let participants = sqlx::query!(
            r#"
            DELETE FROM competition_participants
            WHERE competition_id = $1 AND participant_id <> ALL($2::uuid[])
            "#,
            competition_id,
            &imported.participants
        )
        .execute(&mut **tx)
        .await?
        .rows_affected();

        let lifts = sqlx::query!(
            r#"
            DELETE FROM lifts
            WHERE participant_id = ANY($1::uuid[]) AND lift_id <> ALL($2::uuid[])
            "#,
            &imported.participants,
            &imported.lifts
        )
        .execute(&mut **tx)
        .await?
        .rows_affected();

        let attempts = sqlx::query!(
            r#"
            DELETE FROM attempts
            WHERE lift_id = ANY($1::uuid[]) AND attempt_id <> ALL($2::uuid[])
            "#,
            &imported.lifts,
            &imported.attempts
        )
        .execute(&mut **tx)
        .await?
        .rows_affected();

        if movements + participants + lifts + attempts > 0 {
            info!(
                "Pruned rows the file no longer lists: {} movement(s), {} participant(s), {} lift(s), {} attempt(s)",
                movements, participants, lifts, attempts
            );
        }

        Ok(())
    }

    async fn upsert_competition(
        &self,
        competition: &CompetitionData,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Uuid> {
        let federation_id = self
            .get_or_create_federation(&competition.federation, tx)
            .await?;

        let competition_id = sqlx::query_scalar!(
            r#"
            INSERT INTO competitions (name, slug, status, federation_id, start_date, end_date, venue, city, country, number_of_judge)
            VALUES ($1, $2, COALESCE($3, 'completed'), $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (slug)
            DO UPDATE SET
                name = EXCLUDED.name,
                status = COALESCE($3, competitions.status),
                venue = EXCLUDED.venue,
                city = EXCLUDED.city,
                country = EXCLUDED.country,
                number_of_judge = EXCLUDED.number_of_judge
            RETURNING competition_id as "competition_id: Uuid"
            "#,
            competition.name,
            competition.slug,
            // A canonical file describes a meet that has happened. A status the
            // file does not state is unknown, so it must not overwrite one set
            // deliberately on a re-import.
            competition.status.map(|status| status.as_str()),
            federation_id,
            competition.start_date,
            competition.end_date,
            competition.venue,
            competition.city,
            competition.country.as_str(),
            competition.number_of_judges
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(competition_id)
    }

    async fn get_or_create_federation(
        &self,
        federation: &FederationData,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Uuid> {
        let existing = sqlx::query_scalar!(
            r#"SELECT federation_id as "federation_id: Uuid" FROM federations WHERE name = $1"#,
            federation.name
        )
        .fetch_optional(&mut **tx)
        .await?;

        if let Some(id) = existing {
            sqlx::query!(
                r#"
                UPDATE federations
                SET abbreviation = COALESCE($1, abbreviation),
                    country = COALESCE($2, country)
                WHERE federation_id = $3
                "#,
                federation.abbreviation,
                federation.country.map(|c| c.as_str().to_owned()),
                id
            )
            .execute(&mut **tx)
            .await?;

            return Ok(id);
        }

        let federation_id = sqlx::query_scalar!(
            r#"
            INSERT INTO federations (name, abbreviation, country)
            VALUES ($1, $2, $3)
            RETURNING federation_id as "federation_id: Uuid"
            "#,
            federation.name,
            federation.abbreviation,
            federation.country.map(|c| c.as_str().to_owned())
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| {
            ImporterError::TransformationError(format!("Failed to create federation: {}", e))
        })?;

        Ok(federation_id)
    }

    async fn upsert_competition_movements(
        &self,
        competition_id: Uuid,
        movements: &[MovementData],
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        for movement in movements {
            sqlx::query!(
                r#"
                INSERT INTO competition_movements (competition_id, movement_name, is_required, display_order)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (competition_id, movement_name)
                DO UPDATE SET
                    is_required = EXCLUDED.is_required,
                    display_order = EXCLUDED.display_order
                "#,
                competition_id,
                movement.name,
                movement.is_required.unwrap_or(true),
                movement.order as i32
            )
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    /// Finds the weight class the category's bounds describe, creating it if
    /// the meet runs one outside the standard ladder.
    async fn resolve_weight_class(
        &self,
        category: &CategoryData,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Uuid> {
        let (min, max) = match category.weight_class_slug.as_ref() {
            Some(slug) => slug.bounds(),
            None => (category.weight_class_min, category.weight_class_max),
        };

        if min.is_none() && max.is_none() {
            return Err(ImporterError::TransformationError(format!(
                "Category '{}' has no weight class. Set weight_class_slug, or \
                 weight_class_min and weight_class_max",
                category.name
            )));
        }

        // The no-op SET is what makes RETURNING fire on an existing row.
        let weight_class_id = sqlx::query_scalar!(
            r#"
            INSERT INTO weight_classes (gender, min_kg, max_kg)
            VALUES ($1, $2, $3)
            ON CONFLICT ON CONSTRAINT weight_class_bounds_unique
            DO UPDATE SET gender = EXCLUDED.gender
            RETURNING weight_class_id as "weight_class_id: Uuid"
            "#,
            category.gender.as_str(),
            min,
            max
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(weight_class_id)
    }

    async fn upsert_category(
        &self,
        category: &CategoryData,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Uuid> {
        let gender = category.gender.as_str();

        let existing = sqlx::query_scalar!(
            r#"SELECT category_id as "category_id: Uuid" FROM categories WHERE name = $1 AND gender = $2"#,
            category.name,
            gender
        )
        .fetch_optional(&mut **tx)
        .await?;

        let weight_class_id = self.resolve_weight_class(category, tx).await?;

        if let Some(id) = existing {
            sqlx::query!(
                r#"UPDATE categories SET weight_class_id = $1 WHERE category_id = $2"#,
                weight_class_id,
                id
            )
            .execute(&mut **tx)
            .await?;

            return Ok(id);
        }

        let category_id = sqlx::query_scalar!(
            r#"
            INSERT INTO categories (name, gender, weight_class_id)
            VALUES ($1, $2, $3)
            RETURNING category_id as "category_id: Uuid"
            "#,
            category.name,
            gender,
            weight_class_id
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(category_id)
    }

    async fn import_athlete_performance(
        &self,
        athlete: &AthleteData,
        category: &CategoryData,
        competition_id: Uuid,
        category_id: Uuid,
        imported: &mut ImportedFacts,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        let athlete_id = self.upsert_athlete(athlete, category, tx).await?;

        // The schema stores the outcome as a flag plus free text. Richer
        // statuses need a status column, which 1.1.0 does not add.
        let is_disqualified = athlete.status.is_disqualified();

        let participant_id = sqlx::query_scalar!(
            r#"
            INSERT INTO competition_participants
                (competition_id, category_id, athlete_id, bodyweight, rank, is_disqualified, disqualified_reason, ris_score, ris_source)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CASE WHEN $8::numeric IS NULL THEN NULL ELSE 'reported' END)
            ON CONFLICT (competition_id, category_id, athlete_id)
            DO UPDATE SET
                bodyweight = EXCLUDED.bodyweight,
                rank = EXCLUDED.rank,
                is_disqualified = EXCLUDED.is_disqualified,
                disqualified_reason = EXCLUDED.disqualified_reason,
                ris_score = EXCLUDED.ris_score,
                ris_source = EXCLUDED.ris_source
            RETURNING participant_id as "participant_id: Uuid"
            "#,
            competition_id,
            category_id,
            athlete_id,
            athlete.bodyweight,
            None as Option<i32>,
            is_disqualified,
            athlete.status_reason,
            athlete.ris
        )
        .fetch_one(&mut **tx)
        .await?;

        imported.participants.push(participant_id);

        for lift in &athlete.lifts {
            self.import_lift(participant_id, lift, imported, &mut *tx)
                .await?;
        }

        Ok(())
    }

    async fn upsert_athlete(
        &self,
        athlete: &AthleteData,
        category: &CategoryData,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Uuid> {
        let gender = athlete.gender.unwrap_or(category.gender).as_str();

        let normalized_name = NormalizedAthleteName::new(&athlete.first_name, &athlete.last_name);
        let (db_first_name, db_last_name) = normalized_name.as_database_tuple();

        // The folded name is what decides who this is. The stored name keeps
        // whatever spelling the file used, so the page shows the accents.
        let match_key = normalized_name.match_name();

        let existing = sqlx::query_scalar!(
            r#"
            SELECT athlete_id as "athlete_id: Uuid" FROM athletes
            WHERE match_key = $1
              AND gender = $2
              AND country = $3
              AND disambiguation IS NOT DISTINCT FROM $4
            "#,
            match_key,
            gender,
            athlete.country.as_str(),
            athlete.disambiguation
        )
        .fetch_optional(&mut **tx)
        .await?;

        if let Some(id) = existing {
            // COALESCE because a missing nationality means unknown, not cleared.
            // match_key is rewritten so a row backfilled by the migration takes
            // the importer's fold, which is the authority.
            sqlx::query!(
                r#"
                UPDATE athletes
                SET nationality = COALESCE($1, nationality),
                    match_key = $2
                WHERE athlete_id = $3
                "#,
                athlete.nationality.map(|c| c.as_str().to_owned()),
                match_key,
                id
            )
            .execute(&mut **tx)
            .await?;

            return Ok(id);
        }

        // Slugs come from the folded name, so a URL never carries an accent.
        // Only new athletes get one, so existing links do not move.
        let slug = self.generate_unique_slug(&match_key, &mut *tx).await?;

        let athlete_id = sqlx::query_scalar!(
            r#"
            INSERT INTO athletes (first_name, last_name, gender, country, nationality, slug, match_key, disambiguation)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING athlete_id as "athlete_id: Uuid"
            "#,
            db_first_name,
            db_last_name,
            gender,
            athlete.country.as_str(),
            athlete.nationality.map(|c| c.as_str().to_owned()),
            slug,
            match_key,
            athlete.disambiguation
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(athlete_id)
    }

    async fn generate_unique_slug(
        &self,
        match_key: &str,
        tx: &mut sqlx::PgConnection,
    ) -> Result<String> {
        let base_slug = match_key
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-");

        let base_slug = if base_slug.is_empty() {
            "athlete".to_string()
        } else {
            base_slug
        };

        let mut final_slug = base_slug.clone();
        let mut counter = 2;

        while sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM athletes WHERE slug = $1)",
            final_slug
        )
        .fetch_one(&mut *tx)
        .await?
        .unwrap_or(false)
        {
            final_slug = format!("{}-{}", base_slug, counter);
            counter += 1;
        }

        Ok(final_slug)
    }

    async fn import_lift(
        &self,
        participant_id: Uuid,
        lift: &LiftData,
        imported: &mut ImportedFacts,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        let max_weight = lift
            .attempts
            .iter()
            .filter(|a| a.is_successful)
            .map(|a| a.weight)
            .max();

        // Equipment settings came from a liftcontrol-only field. Nothing in
        // the agnostic format carries them, so the column stays empty until
        // some source can express it.
        let settings: Option<String> = None;

        let lift_id = sqlx::query_scalar!(
            r#"
            INSERT INTO lifts (participant_id, movement_name, max_weight, equipment_setting)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (participant_id, movement_name)
            DO UPDATE SET
                max_weight = EXCLUDED.max_weight,
                equipment_setting = EXCLUDED.equipment_setting,
                updated_at = CURRENT_TIMESTAMP
            RETURNING lift_id as "lift_id: Uuid"
            "#,
            participant_id,
            lift.movement,
            max_weight,
            settings
        )
        .fetch_one(&mut **tx)
        .await?;

        imported.lifts.push(lift_id);

        for attempt in &lift.attempts {
            self.import_attempt(lift_id, attempt, imported, tx).await?;
        }

        Ok(())
    }

    async fn import_attempt(
        &self,
        lift_id: Uuid,
        attempt: &AttemptData,
        imported: &mut ImportedFacts,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        let attempt_id = sqlx::query_scalar!(
            r#"
            INSERT INTO attempts (lift_id, attempt_number, weight, is_successful, passing_judges, no_rep_reason, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (lift_id, attempt_number)
            DO UPDATE SET
                weight = EXCLUDED.weight,
                is_successful = EXCLUDED.is_successful,
                passing_judges = EXCLUDED.passing_judges,
                no_rep_reason = EXCLUDED.no_rep_reason,
                created_by = EXCLUDED.created_by
            RETURNING attempt_id as "attempt_id: Uuid"
            "#,
            lift_id,
            attempt.attempt_number,
            attempt.weight,
            attempt.is_successful,
            None as Option<i16>,
            attempt.judge_note,
            "Canonical Importer"
        )
        .fetch_one(&mut **tx)
        .await?;

        imported.attempts.push(attempt_id);

        Ok(())
    }

    async fn compute_ris_for_competition(
        &self,
        competition_id: Uuid,
        competition_date: chrono::NaiveDate,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<()> {
        let formula =
            osl_db::services::ris_computation::get_formula_for_date(self.pool, competition_date)
                .await
                .map_err(|e| {
                    ImporterError::TransformationError(format!(
                        "No RIS formula available for date {}: {}",
                        competition_date, e
                    ))
                })?;

        let participants = sqlx::query!(
            r#"
            SELECT
                cp.participant_id,
                cp.athlete_id,
                cp.bodyweight,
                a.gender,
                COALESCE(SUM(l.max_weight), 0) as "total!: rust_decimal::Decimal"
            FROM competition_participants cp
            INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
            LEFT JOIN lifts l ON l.participant_id = cp.participant_id
            WHERE cp.competition_id = $1
              AND cp.ris_source IS DISTINCT FROM 'reported'
            GROUP BY cp.participant_id, cp.athlete_id, cp.bodyweight, a.gender
            "#,
            competition_id
        )
        .fetch_all(&mut **tx)
        .await?;

        let participant_count = participants.len();

        for participant in participants {
            if let Some(bodyweight) = participant.bodyweight {
                let ris_score = osl_domain::ris::compute_ris(
                    bodyweight,
                    participant.total,
                    &participant.gender,
                    &formula,
                )
                .map_err(|e| {
                    ImporterError::TransformationError(format!(
                        "Failed to compute RIS for participant {}: {}",
                        participant.participant_id, e
                    ))
                })?;

                sqlx::query!(
                    r#"
                    UPDATE competition_participants
                    SET ris_score = $1, ris_source = 'computed'
                    WHERE participant_id = $2
                    "#,
                    ris_score,
                    participant.participant_id
                )
                .execute(&mut **tx)
                .await?;

                sqlx::query!(
                    r#"
                    INSERT INTO ris_scores_history (participant_id, formula_id, ris_score, bodyweight, total_weight)
                    VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (participant_id, formula_id)
                    DO UPDATE SET
                        ris_score = EXCLUDED.ris_score,
                        bodyweight = EXCLUDED.bodyweight,
                        total_weight = EXCLUDED.total_weight,
                        computed_at = CURRENT_TIMESTAMP
                    "#,
                    participant.participant_id,
                    formula.formula_id,
                    ris_score,
                    bodyweight,
                    participant.total
                )
                .execute(&mut **tx)
                .await?;
            }
        }

        info!("Computed RIS for {} participants", participant_count);
        Ok(())
    }
}
