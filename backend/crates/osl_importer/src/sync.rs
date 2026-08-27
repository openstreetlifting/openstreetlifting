//! Bringing the database back to what the canonical files say.
//!
//! The files are the data, so a competition no file claims should not exist,
//! and neither should an athlete or a federation left without a single result.

use osl_domain::display_name;
use sqlx::PgPool;

use crate::error::{ImporterError, Result};

pub struct CompetitionSync<'a> {
    pool: &'a PgPool,
}

#[derive(Debug, Default)]
pub struct SyncPlan {
    pub competitions: Vec<DeletedCompetition>,
    pub athletes: Vec<String>,
    pub federations: Vec<String>,
}

#[derive(Debug)]
pub struct DeletedCompetition {
    pub slug: String,
    pub name: String,
}

impl SyncPlan {
    pub fn is_empty(&self) -> bool {
        self.competitions.is_empty() && self.athletes.is_empty() && self.federations.is_empty()
    }
}

impl<'a> CompetitionSync<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// What `apply` would delete, worked out by doing it and rolling back, so
    /// the report can never disagree with what follows it.
    pub async fn dry_run(&self, claimed_slugs: &[String]) -> Result<SyncPlan> {
        let mut tx = self.pool.begin().await?;
        let plan = Self::purge(claimed_slugs, &mut tx).await?;
        tx.rollback().await?;

        Ok(plan)
    }

    pub async fn apply(&self, claimed_slugs: &[String]) -> Result<SyncPlan> {
        let mut tx = self.pool.begin().await?;
        let plan = Self::purge(claimed_slugs, &mut tx).await?;
        tx.commit().await?;

        Ok(plan)
    }

    /// Claiming nothing would mean deleting every competition, which is what a
    /// mistyped directory looks like, so it is refused rather than obeyed.
    async fn purge(
        claimed_slugs: &[String],
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<SyncPlan> {
        if claimed_slugs.is_empty() {
            return Err(ImporterError::ImportError(
                "no canonical file claims a competition, so every stored competition would go. \
                 Point the import at the whole imports tree"
                    .to_string(),
            ));
        }

        let competitions = sqlx::query!(
            r#"
            DELETE FROM competitions
            WHERE slug <> ALL($1::text[])
            RETURNING slug, name
            "#,
            claimed_slugs
        )
        .fetch_all(&mut **tx)
        .await?;

        let athletes = sqlx::query!(
            r#"
            DELETE FROM athletes
            WHERE NOT EXISTS (
                SELECT 1
                FROM competition_participants cp
                WHERE cp.athlete_id = athletes.athlete_id
            )
            RETURNING first_name, last_name
            "#
        )
        .fetch_all(&mut **tx)
        .await?;

        // Renaming a federation in a file leaves the old row behind with nothing
        // pointing at it, so it goes the same way an athlete without a result does.
        let federations = sqlx::query!(
            r#"
            DELETE FROM federations
            WHERE NOT EXISTS (
                SELECT 1
                FROM competitions c
                WHERE c.federation_id = federations.federation_id
            )
            RETURNING name
            "#
        )
        .fetch_all(&mut **tx)
        .await?;

        Ok(SyncPlan {
            competitions: competitions
                .into_iter()
                .map(|row| DeletedCompetition {
                    slug: row.slug,
                    name: row.name,
                })
                .collect(),
            athletes: athletes
                .into_iter()
                .map(|row| display_name(&row.first_name, &row.last_name))
                .collect(),
            federations: federations.into_iter().map(|row| row.name).collect(),
        })
    }
}
