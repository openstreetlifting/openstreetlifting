use sqlx::{Postgres, Transaction};

use crate::error::Result;

pub async fn refresh(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
    sqlx::query("TRUNCATE athlete_metric_standings")
        .execute(&mut **tx)
        .await?;
    sqlx::query("TRUNCATE athlete_rivals")
        .execute(&mut **tx)
        .await?;

    let mut query = crate::repository::ranking_query::movement_weights(None);
    super::athlete_standings_sql::push_metric_pool(&mut query);
    query.build().execute(&mut **tx).await?;

    let mut query = crate::repository::ranking_query::movement_weights(None);
    super::athlete_standings_sql::push_rivals(&mut query);
    query.build().execute(&mut **tx).await?;

    Ok(())
}
