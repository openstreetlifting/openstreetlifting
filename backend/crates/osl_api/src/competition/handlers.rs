use crate::AppState;
use crate::error::{WebError, WebResult};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use osl_db::repository::competition::CompetitionRepository;
use osl_domain::CompetitionStatus;
use serde::Deserialize;
use std::str::FromStr;

use super::dto::CompetitionResponse;
use crate::shared::dto::{PaginatedResponse, PaginationParams};
use crate::shared::query::Include;

const LIST_INCLUDES: &[&str] = &["federation", "movements"];
const DETAIL_INCLUDES: &[&str] = &["federation", "results"];

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct CompetitionListQuery {
    #[serde(default)]
    pub include: Include,
    pub status: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct CompetitionQuery {
    #[serde(default)]
    pub include: Include,
}

#[utoipa::path(
    get,
    path = "/api/v1/competitions",
    params(CompetitionListQuery),
    responses(
        (status = 200, description = "A page of competitions", body = PaginatedResponse<CompetitionResponse>),
        (status = 400, description = "Invalid pagination or include value")
    ),
    tag = "competitions"
)]
pub async fn list_competitions(
    State(state): State<AppState>,
    Query(query): Query<CompetitionListQuery>,
) -> WebResult<Json<PaginatedResponse<CompetitionResponse>>> {
    query.pagination.validate().map_err(WebError::BadRequest)?;
    query
        .include
        .validate(LIST_INCLUDES)
        .map_err(WebError::BadRequest)?;

    let status = query
        .status
        .as_deref()
        .map(CompetitionStatus::from_str)
        .transpose()
        .map_err(WebError::BadRequest)?;

    let repo = CompetitionRepository::new(state.db.pool());
    let page = query.pagination.to_page();
    let status = status.map(|status| status.as_str());

    // The per-row federation and movement lookups are only worth it when asked for.
    let (data, total_items) = if LIST_INCLUDES.iter().any(|name| query.include.has(name)) {
        let (items, total) = repo.list_with_details(&page, status).await?;
        let data = items
            .into_iter()
            .map(|item| CompetitionResponse::from_list_item(item, &query.include))
            .collect();
        (data, total)
    } else {
        let (rows, total) = repo.list(&page, status).await?;
        let data = rows.into_iter().map(CompetitionResponse::from).collect();
        (data, total)
    };

    Ok(Json(PaginatedResponse::new(
        data,
        query.pagination.page,
        query.pagination.page_size,
        total_items,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/competitions/{slug}",
    params(
        ("slug" = String, Path, description = "Competition slug"),
        CompetitionQuery,
    ),
    responses(
        (status = 200, description = "Competition found", body = CompetitionResponse),
        (status = 400, description = "Unknown include value"),
        (status = 404, description = "Competition not found")
    ),
    tag = "competitions"
)]
pub async fn get_competition(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<CompetitionQuery>,
) -> WebResult<Json<CompetitionResponse>> {
    query
        .include
        .validate(DETAIL_INCLUDES)
        .map_err(WebError::BadRequest)?;

    let repo = CompetitionRepository::new(state.db.pool());

    if DETAIL_INCLUDES.iter().any(|name| query.include.has(name)) {
        let detail = repo.find_by_slug_detailed(&slug).await?;
        return Ok(Json(CompetitionResponse::from_detail(
            detail,
            &query.include,
        )));
    }

    let competition = repo.find_by_slug(&slug).await?;
    Ok(Json(CompetitionResponse::from(competition)))
}
