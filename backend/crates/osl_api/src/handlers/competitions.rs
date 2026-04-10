use crate::AppState;
use crate::error::{WebError, WebResult};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use osl_db::{
    dto::competition::{
        CompetitionDetailResponse, CompetitionListResponse, CompetitionResponse,
        CreateCompetitionRequest, UpdateCompetitionRequest,
    },
    repository::competition::CompetitionRepository,
};

#[utoipa::path(
    get,
    path = "/api/competitions",
    responses(
        (status = 200, description = "List all competitions successfully", body = Vec<CompetitionResponse>)
    ),
    tag = "competitions"
)]
pub async fn list_competitions(
    State(state): State<AppState>,
) -> WebResult<Json<Vec<CompetitionResponse>>> {
    let repo = CompetitionRepository::new(state.db.pool());
    let competitions = repo.list().await?;

    let response: Vec<CompetitionResponse> = competitions
        .into_iter()
        .map(CompetitionResponse::from)
        .collect();

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/competitions/detailed",
    responses(
        (status = 200, description = "List all competitions with detailed information (federation and movements)", body = Vec<CompetitionListResponse>)
    ),
    tag = "competitions"
)]
pub async fn list_competitions_detailed(
    State(state): State<AppState>,
) -> WebResult<Json<Vec<CompetitionListResponse>>> {
    let repo = CompetitionRepository::new(state.db.pool());
    let competitions = repo.list_with_details().await?;

    Ok(Json(competitions))
}

#[utoipa::path(
    get,
    path = "/api/competitions/{slug}",
    params(
        ("slug" = String, Path, description = "Competition slug")
    ),
    responses(
        (status = 200, description = "Competition found", body = CompetitionResponse),
        (status = 404, description = "Competition not found")
    ),
    tag = "competitions"
)]
pub async fn get_competition(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> WebResult<Json<CompetitionResponse>> {
    let repo = CompetitionRepository::new(state.db.pool());
    let competition = repo.find_by_slug(&slug).await?;

    Ok(Json(CompetitionResponse::from(competition)))
}

#[utoipa::path(
    get,
    path = "/api/competitions/{slug}/detailed",
    params(
        ("slug" = String, Path, description = "Competition slug")
    ),
    responses(
        (status = 200, description = "Competition with full details including category-merged participants and computed rankings", body = CompetitionDetailResponse),
        (status = 404, description = "Competition not found")
    ),
    tag = "competitions"
)]
pub async fn get_competition_detailed(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> WebResult<Json<CompetitionDetailResponse>> {
    let repo = CompetitionRepository::new(state.db.pool());
    let competition = repo.find_by_slug_detailed(&slug).await?;

    Ok(Json(competition))
}

#[utoipa::path(
    post,
    path = "/api/competitions",
    request_body = CreateCompetitionRequest,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Competition created successfully", body = CompetitionResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Slug already exists")
    ),
    tag = "competitions"
)]
pub async fn create_competition(
    State(state): State<AppState>,
    Json(req): Json<CreateCompetitionRequest>,
) -> WebResult<impl IntoResponse> {
    req.validate_dates()
        .map_err(|e| WebError::BadRequest(e.to_string()))?;

    let repo = CompetitionRepository::new(state.db.pool());
    let competition = repo.create(&req).await?;

    Ok((
        StatusCode::CREATED,
        Json(CompetitionResponse::from(competition)),
    ))
}

#[utoipa::path(
    put,
    path = "/api/competitions/{slug}",
    params(
        ("slug" = String, Path, description = "Competition slug")
    ),
    request_body = UpdateCompetitionRequest,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Competition updated successfully", body = CompetitionResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Competition not found"),
        (status = 409, description = "Slug already exists")
    ),
    tag = "competitions"
)]
pub async fn update_competition(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(update_req): Json<UpdateCompetitionRequest>,
) -> WebResult<Json<CompetitionResponse>> {
    let repo = CompetitionRepository::new(state.db.pool());
    let existing = repo.find_by_slug(&slug).await?;
    let updated = repo
        .update(existing.competition_id, &existing, &update_req)
        .await?;

    Ok(Json(CompetitionResponse::from(updated)))
}

#[utoipa::path(
    delete,
    path = "/api/competitions/{slug}",
    params(
        ("slug" = String, Path, description = "Competition slug")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 204, description = "Competition deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Competition not found")
    ),
    tag = "competitions"
)]
pub async fn delete_competition(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> WebResult<StatusCode> {
    let repo = CompetitionRepository::new(state.db.pool());
    let competition = repo.find_by_slug(&slug).await?;
    repo.delete(competition.competition_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
