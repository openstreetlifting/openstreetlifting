use crate::error::{WebError, WebResult};
use axum::{
    Json,
    extract::{Json as JsonBody, Path},
};
use osl_domain::{Edition, Gender};
use std::str::FromStr;

use super::dto::{ComputeRisRequest, ComputeRisResponse, RisFormulaResponse};

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
    let gender = Gender::from_str(&payload.gender).map_err(WebError::BadRequest)?;

    let edition = match payload.formula_year {
        Some(year) => Edition::from_year(year).ok_or(WebError::NotFound)?,
        None => Edition::CURRENT,
    };

    let ris_score = osl_domain::ris::compute(payload.bodyweight, payload.total, gender, edition);

    Ok(Json(ComputeRisResponse {
        ris_score,
        formula_year: edition.year(),
    }))
}
