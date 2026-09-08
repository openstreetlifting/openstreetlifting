use crate::AppState;
use crate::error::{WebError, WebResult};
use axum::{
    Json,
    body::Bytes,
    extract::{Json as JsonBody, Path, State},
    http::header,
    response::{IntoResponse, Response},
};
use osl_domain::Edition;

use osl_db::repository::ris::RisRepository;

use super::dto::{
    ComputeRisRequest, ComputeRisResponse, RisDistributionResponse, RisFormulaResponse,
    RisPerformanceResponse,
};

#[utoipa::path(
    get,
    path = "/api/v1/ris/formulas",
    responses(
        (status = 200, description = "List every RIS edition", body = Vec<RisFormulaResponse>)
    ),
    tag = "ris"
)]
pub async fn list_ris_formulas() -> Json<Vec<RisFormulaResponse>> {
    Json(Edition::ALL.into_iter().map(Into::into).collect())
}

#[utoipa::path(
    get,
    path = "/api/v1/ris/formulas/current",
    responses(
        (status = 200, description = "The edition every ranking is scored with", body = RisFormulaResponse)
    ),
    tag = "ris"
)]
pub async fn get_current_formula() -> Json<RisFormulaResponse> {
    Json(Edition::CURRENT.into())
}

#[utoipa::path(
    get,
    path = "/api/v1/ris/formulas/{year}",
    params(("year" = i32, Path, description = "Edition year")),
    responses(
        (status = 200, description = "RIS edition for this year", body = RisFormulaResponse),
        (status = 404, description = "No edition was published for this year")
    ),
    tag = "ris"
)]
pub async fn get_formula_by_year(Path(year): Path<i32>) -> WebResult<Json<RisFormulaResponse>> {
    let edition = Edition::from_year(year).ok_or(WebError::NotFound)?;

    Ok(Json(edition.into()))
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
    JsonBody(payload): JsonBody<ComputeRisRequest>,
) -> WebResult<Json<ComputeRisResponse>> {
    let edition = match payload.formula_year {
        Some(year) => Edition::from_year(year).ok_or(WebError::NotFound)?,
        None => Edition::CURRENT,
    };

    let ris_score =
        osl_domain::ris::compute(payload.bodyweight, payload.total, payload.gender, edition);

    Ok(Json(ComputeRisResponse {
        ris_score,
        formula_year: edition.year(),
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/ris/distribution",
    responses(
        (status = 200, description = "Every scored performance with its athlete and competition", body = RisDistributionResponse)
    ),
    tag = "ris"
)]
pub async fn get_ris_distribution(State(state): State<AppState>) -> WebResult<Response> {
    let body = state
        .caches
        .ris_distribution
        .get_or_try_init(|| load_ris_distribution(&state.db))
        .await?;

    Ok(([(header::CONTENT_TYPE, "application/json")], body).into_response())
}

async fn load_ris_distribution(db: &osl_db::Database) -> WebResult<Bytes> {
    let repo = RisRepository::new(db.pool());
    let performances = repo.scored_performances().await?;

    let mut response = RisDistributionResponse {
        men: Vec::new(),
        women: Vec::new(),
    };

    for performance in performances {
        let point = RisPerformanceResponse {
            participant_id: performance.participant_id,
            athlete_name: performance.athlete_name,
            athlete_slug: performance.athlete_slug,
            competition_name: performance.competition_name,
            competition_slug: performance.competition_slug,
            competition_date: performance.competition_date,
            bodyweight: performance.bodyweight,
            total: performance.total,
        };

        match performance.gender.as_str() {
            "F" => response.women.push(point),
            _ => response.men.push(point),
        }
    }

    // Cache the wire payload as cheap-to-clone bytes, not thousands of owned strings.
    serde_json::to_vec(&response)
        .map(Bytes::from)
        .map_err(|error| WebError::InternalServerError(error.to_string()))
}
