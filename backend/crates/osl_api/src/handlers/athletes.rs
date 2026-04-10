use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use osl_db::{
    dto::athlete::{
        AthleteDetailResponse, AthleteResponse, CreateAthleteRequest, UpdateAthleteRequest,
    },
    repository::athlete::AthleteRepository,
};
use crate::AppState;
use crate::error::WebResult;

#[utoipa::path(
    get,
    path = "/api/athletes",
    responses(
        (status = 200, description = "List all athletes successfully", body = Vec<AthleteResponse>)
    ),
    tag = "athletes"
)]
pub async fn list_athletes(
    State(state): State<AppState>,
) -> WebResult<Json<Vec<AthleteResponse>>> {
    let repo = AthleteRepository::new(state.db.pool());
    let athletes = repo.list().await?;

    let response: Vec<AthleteResponse> = athletes.into_iter().map(AthleteResponse::from).collect();

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/athletes/{slug}",
    params(
        ("slug" = String, Path, description = "Athlete slug")
    ),
    responses(
        (status = 200, description = "Athlete found", body = AthleteResponse),
        (status = 404, description = "Athlete not found")
    ),
    tag = "athletes"
)]
pub async fn get_athlete(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> WebResult<Json<AthleteResponse>> {
    let repo = AthleteRepository::new(state.db.pool());
    let athlete = repo.find_by_slug(&slug).await?;

    Ok(Json(AthleteResponse::from(athlete)))
}

#[utoipa::path(
    get,
    path = "/api/athletes/{slug}/detailed",
    params(
        ("slug" = String, Path, description = "Athlete slug")
    ),
    responses(
        (status = 200, description = "Athlete with full details including competition history", body = AthleteDetailResponse),
        (status = 404, description = "Athlete not found")
    ),
    tag = "athletes"
)]
pub async fn get_athlete_detailed(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> WebResult<Json<AthleteDetailResponse>> {
    let repo = AthleteRepository::new(state.db.pool());
    let athlete = repo.find_by_slug_detailed(&slug).await?;

    Ok(Json(athlete))
}

#[utoipa::path(
    post,
    path = "/api/athletes",
    request_body = CreateAthleteRequest,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Athlete created successfully", body = AthleteResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "athletes"
)]
pub async fn create_athlete(
    State(state): State<AppState>,
    Json(req): Json<CreateAthleteRequest>,
) -> WebResult<impl IntoResponse> {
    let repo = AthleteRepository::new(state.db.pool());
    let athlete = repo.create(&req).await?;

    Ok((StatusCode::CREATED, Json(AthleteResponse::from(athlete))))
}

#[utoipa::path(
    put,
    path = "/api/athletes/{slug}",
    params(
        ("slug" = String, Path, description = "Athlete slug")
    ),
    request_body = UpdateAthleteRequest,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Athlete updated successfully", body = AthleteResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Athlete not found")
    ),
    tag = "athletes"
)]
pub async fn update_athlete(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(update_req): Json<UpdateAthleteRequest>,
) -> WebResult<Json<AthleteResponse>> {
    let repo = AthleteRepository::new(state.db.pool());
    let existing = repo.find_by_slug(&slug).await?;
    let updated = repo
        .update(existing.athlete_id, &existing, &update_req)
        .await?;

    Ok(Json(AthleteResponse::from(updated)))
}

#[utoipa::path(
    delete,
    path = "/api/athletes/{slug}",
    params(
        ("slug" = String, Path, description = "Athlete slug")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 204, description = "Athlete deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Athlete not found")
    ),
    tag = "athletes"
)]
pub async fn delete_athlete(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> WebResult<StatusCode> {
    let repo = AthleteRepository::new(state.db.pool());
    let athlete = repo.find_by_slug(&slug).await?;
    repo.delete(athlete.athlete_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
