use chrono::NaiveDate;
use osl_domain::RisFormula;
use osl_domain::ris::compute_ris;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;
use crate::params::RisScoreUpsert;
use crate::repository::ris::RisRepository;
use crate::rows::ris_score::RisScoreHistoryRow;

pub async fn get_formula_for_date(
    pool: &PgPool,
    competition_date: NaiveDate,
) -> Result<RisFormula> {
    let repo = RisRepository::new(pool);
    Ok(repo.get_formula_for_date(competition_date).await?.into())
}

pub async fn get_current_formula(pool: &PgPool) -> Result<RisFormula> {
    let repo = RisRepository::new(pool);
    Ok(repo.get_current_formula().await?.into())
}

pub async fn compute_and_store_ris(
    pool: &PgPool,
    participant_id: Uuid,
    bodyweight: Decimal,
    total: Decimal,
    gender: &str,
) -> Result<Decimal> {
    let repo = RisRepository::new(pool);
    let formula: RisFormula = repo.get_current_formula().await?.into();

    let ris_score = compute_ris(bodyweight, total, gender, &formula)?;

    repo.upsert_ris_score(&RisScoreUpsert {
        participant_id,
        formula_id: formula.formula_id,
        ris_score,
        bodyweight,
        total_weight: total,
    })
    .await?;
    repo.update_participant_current_ris(participant_id, ris_score)
        .await?;

    Ok(ris_score)
}

pub async fn compute_historical_ris(
    pool: &PgPool,
    participant_id: Uuid,
    bodyweight: Decimal,
    total: Decimal,
    gender: &str,
) -> Result<Vec<RisScoreHistoryRow>> {
    let repo = RisRepository::new(pool);
    let formulas = repo.list_all_formulas().await?;

    let mut results = Vec::new();

    for formula in formulas {
        let formula: RisFormula = formula.into();
        let ris_score = compute_ris(bodyweight, total, gender, &formula)?;
        let score_history = repo
            .upsert_ris_score(&RisScoreUpsert {
                participant_id,
                formula_id: formula.formula_id,
                ris_score,
                bodyweight,
                total_weight: total,
            })
            .await?;
        results.push(score_history);
    }

    Ok(results)
}

pub async fn recompute_all_ris(pool: &PgPool, formula_id: Option<Uuid>) -> Result<u64> {
    let repo = RisRepository::new(pool);

    let formula: RisFormula = if let Some(fid) = formula_id {
        repo.get_formula_by_id(fid).await?.into()
    } else {
        repo.get_current_formula().await?.into()
    };

    let participants = sqlx::query!(
        r#"
        SELECT
            cp.participant_id,
            cp.bodyweight,
            a.gender,
            COALESCE(SUM(l.max_weight), 0) as "total!: rust_decimal::Decimal"
        FROM competition_participants cp
        INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
        LEFT JOIN lifts l ON l.participant_id = cp.participant_id
        WHERE cp.bodyweight IS NOT NULL
        GROUP BY cp.participant_id, cp.bodyweight, a.gender
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut count = 0u64;

    for participant in participants {
        if let Some(bodyweight) = participant.bodyweight {
            let ris_score =
                compute_ris(bodyweight, participant.total, &participant.gender, &formula)?;

            repo.upsert_ris_score(&RisScoreUpsert {
                participant_id: participant.participant_id,
                formula_id: formula.formula_id,
                ris_score,
                bodyweight,
                total_weight: participant.total,
            })
            .await?;

            repo.update_participant_current_ris(participant.participant_id, ris_score)
                .await?;
            count += 1;
        }
    }

    Ok(count)
}
