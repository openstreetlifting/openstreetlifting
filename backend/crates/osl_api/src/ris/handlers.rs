use crate::AppState;
use crate::error::WebResult;
use axum::{
    Json,
    extract::{Path, State},
};
use osl_db::repository::ris::RisRepository;
use osl_domain::RisFormula;

use super::dto::{ComputeRisRequest, ComputeRisResponse, RisFormulaResponse, RisScoreResponse};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/v1/ris/formulas",
    responses(
        (status = 200, description = "List all RIS formula versions", body = Vec<RisFormulaResponse>)
    ),
    tag = "ris"
)]
pub async fn list_ris_formulas(
    State(state): State<AppState>,
) -> WebResult<Json<Vec<RisFormulaResponse>>> {
    let repo = RisRepository::new(state.db.pool());
    let formulas = repo.list_all_formulas().await?;

    let response: Vec<RisFormulaResponse> = formulas
        .into_iter()
        .map(|row| RisFormulaResponse::from(RisFormula::from(row)))
        .collect();

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/ris/formulas/current",
    responses(
        (status = 200, description = "Get the current active RIS formula", body = RisFormulaResponse),
        (status = 404, description = "No current formula found")
    ),
    tag = "ris"
)]
pub async fn get_current_formula(
    State(state): State<AppState>,
) -> WebResult<Json<RisFormulaResponse>> {
    let repo = RisRepository::new(state.db.pool());
    let formula = RisFormula::from(repo.get_current_formula().await?);

    Ok(Json(RisFormulaResponse::from(formula)))
}

#[utoipa::path(
    get,
    path = "/api/v1/ris/formulas/{year}",
    params(
        ("year" = i32, Path, description = "Formula year")
    ),
    responses(
        (status = 200, description = "RIS formula for specified year", body = RisFormulaResponse),
        (status = 404, description = "Formula not found for this year")
    ),
    tag = "ris"
)]
pub async fn get_formula_by_year(
    State(state): State<AppState>,
    Path(year): Path<i32>,
) -> WebResult<Json<RisFormulaResponse>> {
    let repo = RisRepository::new(state.db.pool());
    let formula = RisFormula::from(repo.get_formula_by_year(year).await?);

    Ok(Json(RisFormulaResponse::from(formula)))
}

#[utoipa::path(
    get,
    path = "/api/v1/participants/{participant_id}/ris-scores",
    params(
        ("participant_id" = Uuid, Path, description = "Participant ID")
    ),
    responses(
        (status = 200, description = "RIS scores for participant", body = Vec<RisScoreResponse>)
    ),
    tag = "ris"
)]
pub async fn get_participant_ris_scores(
    State(state): State<AppState>,
    Path(participant_id): Path<Uuid>,
) -> WebResult<Json<Vec<RisScoreResponse>>> {
    let repo = RisRepository::new(state.db.pool());
    let history = repo.get_participant_ris_history(participant_id).await?;

    let formulas = repo.list_all_formulas().await?;
    let formula_map: std::collections::HashMap<Uuid, i32> = formulas
        .into_iter()
        .map(|f| (f.formula_id, f.year))
        .collect();

    let response: Vec<RisScoreResponse> = history
        .into_iter()
        .map(|h| RisScoreResponse {
            formula_year: *formula_map.get(&h.formula_id).unwrap_or(&2025),
            ris_score: h.ris_score,
            bodyweight: h.bodyweight,
            total_weight: h.total_weight,
            computed_at: h.computed_at,
        })
        .collect();

    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/ris/calculations",
    request_body = ComputeRisRequest,
    responses(
        (status = 200, description = "RIS computed successfully", body = ComputeRisResponse),
        (status = 400, description = "Invalid request")
    ),
    tag = "ris"
)]
pub async fn calculate_ris(
    State(state): State<AppState>,
    Json(payload): Json<ComputeRisRequest>,
) -> WebResult<Json<ComputeRisResponse>> {
    let repo = RisRepository::new(state.db.pool());
    let formula: RisFormula = if let Some(year) = payload.formula_year {
        repo.get_formula_by_year(year).await?.into()
    } else {
        repo.get_current_formula().await?.into()
    };

    let ris_score =
        osl_domain::ris::compute_ris(payload.bodyweight, payload.total, &payload.gender, &formula)?;

    let response = ComputeRisResponse {
        ris_score,
        formula_year: formula.year,
    };

    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/ris/recomputations",
    responses(
        (status = 200, description = "RIS scores recomputed successfully"),
        (status = 500, description = "Recomputation failed")
    ),
    tag = "ris"
)]
pub async fn recompute_ris_scores(
    State(state): State<AppState>,
) -> WebResult<Json<serde_json::Value>> {
    let count = osl_db::services::ris_computation::recompute_all_ris(state.db.pool(), None).await?;

    Ok(Json(serde_json::json!({
        "recomputed_count": count,
        "message": format!("Successfully recomputed RIS for {} participants", count)
    })))
}
