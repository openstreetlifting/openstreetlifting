use sqlx::{Postgres, Transaction};

use crate::error::{Result, StorageError};

/// Resolve profile countries from source entries after the complete import batch.
pub async fn refresh(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
    let conflict = sqlx::query!(
        r#"SELECT a.first_name, a.last_name,
                  string_agg(DISTINCT cp.country, ', ' ORDER BY cp.country) AS "countries!"
           FROM athletes a JOIN competition_participants cp USING (athlete_id)
           GROUP BY a.athlete_id
           HAVING COUNT(DISTINCT cp.country) > 1
           ORDER BY a.athlete_id LIMIT 1"#
    )
    .fetch_optional(&mut **tx)
    .await?;
    if let Some(conflict) = conflict {
        return Err(StorageError::ConflictingCountries(format!(
            "{}: {}; correct the source entries together, or use Disambiguation for different people",
            osl_domain::display_name(&conflict.first_name, &conflict.last_name),
            conflict.countries
        )));
    }
    sqlx::query!(
        "UPDATE athletes a SET country = evidence.country
         FROM (SELECT athlete_id, MIN(country) AS country
               FROM competition_participants GROUP BY athlete_id) evidence
         WHERE evidence.athlete_id = a.athlete_id AND a.country IS DISTINCT FROM evidence.country"
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}
