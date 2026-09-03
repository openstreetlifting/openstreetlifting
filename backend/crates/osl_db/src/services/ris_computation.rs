use osl_domain::RisFormula;
use osl_domain::event::FULL_EVENT;
use osl_domain::ris::compute_ris;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;
use crate::params::RisScoreUpsert;
use crate::projections::ris::ScorableParticipant;
use crate::repository::ris::RisRepository;

pub async fn get_current_formula(pool: &PgPool) -> Result<RisFormula> {
    let repo = RisRepository::new(pool);
    Ok(repo.get_current_formula().await?.into())
}

/// The formula divides by a benchmark fitted to four-lift totals, so it means
/// nothing on a shorter event. A reported score came from a source that never
/// gave us the bodyweight behind it, so it cannot be reproduced and is never
/// overwritten.
pub async fn scorable_participants<'e, E>(
    executor: E,
    competition_id: Option<Uuid>,
) -> Result<Vec<ScorableParticipant>>
where
    E: sqlx::PgExecutor<'e>,
{
    let participants = sqlx::query_as!(
        ScorableParticipant,
        r#"
        SELECT
            cp.participant_id,
            cp.bodyweight as "bodyweight!",
            a.gender,
            COALESCE(SUM(l.max_weight), 0) as "total!"
        FROM competition_participants cp
        INNER JOIN athletes a ON cp.athlete_id = a.athlete_id
        INNER JOIN competitions c ON c.competition_id = cp.competition_id
        LEFT JOIN lifts l ON l.participant_id = cp.participant_id
        WHERE c.event_code = $1
          AND cp.status = 'competed'
          AND cp.ris_source IS DISTINCT FROM 'reported'
          AND cp.bodyweight IS NOT NULL
          AND ($2::uuid IS NULL OR cp.competition_id = $2)
        GROUP BY cp.participant_id, cp.bodyweight, a.gender
        "#,
        FULL_EVENT,
        competition_id
    )
    .fetch_all(executor)
    .await?;

    Ok(participants)
}

/// Publishing a new RIS edition means re-scoring the archive, so one ranking
/// never mixes two scales.
pub async fn recompute_all_ris(pool: &PgPool) -> Result<u64> {
    let repo = RisRepository::new(pool);
    let formula: RisFormula = repo.get_current_formula().await?.into();

    let participants = scorable_participants(pool, None).await?;
    let mut count = 0u64;

    for participant in participants {
        let ris_score = compute_ris(
            participant.bodyweight,
            participant.total,
            &participant.gender,
            &formula,
        )?;

        repo.upsert_ris_score(&RisScoreUpsert {
            participant_id: participant.participant_id,
            formula_id: formula.formula_id,
            ris_score,
            bodyweight: participant.bodyweight,
            total_weight: participant.total,
        })
        .await?;

        repo.update_participant_current_ris(participant.participant_id, ris_score)
            .await?;
        count += 1;
    }

    Ok(count)
}
