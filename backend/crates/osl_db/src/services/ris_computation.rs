use osl_domain::Edition;
use osl_domain::ris::compute;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;
use crate::projections::ris::ScorableParticipant;
use crate::repository::parse_gender;

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
        osl_domain::FULL_EVENT,
        competition_id
    )
    .fetch_all(executor)
    .await?;

    Ok(participants)
}

pub async fn score_participant<'e, E>(
    executor: E,
    participant: &ScorableParticipant,
    edition: Edition,
) -> Result<()>
where
    E: sqlx::PgExecutor<'e>,
{
    let gender = parse_gender(&participant.gender)?;
    let ris_score = compute(participant.bodyweight, participant.total, gender, edition);

    sqlx::query!(
        r#"
        UPDATE competition_participants
        SET ris_score = $1, ris_source = 'computed', ris_edition = $2
        WHERE participant_id = $3
        "#,
        ris_score,
        edition.year(),
        participant.participant_id
    )
    .execute(executor)
    .await?;

    Ok(())
}

/// Publishing a new RIS edition means re-scoring the archive, so one ranking
/// never mixes two scales.
pub async fn recompute_all_ris(pool: &PgPool) -> Result<u64> {
    let participants = scorable_participants(pool, None).await?;
    let mut count = 0u64;

    for participant in &participants {
        score_participant(pool, participant, Edition::CURRENT).await?;
        count += 1;
    }

    Ok(count)
}
