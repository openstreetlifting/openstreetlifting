use axum::{
    Json,
    extract::{Query, State},
};
use osl_db::{
    dto::{
        common::PaginatedResponse,
        ranking::{GlobalRankingEntry, GlobalRankingFilter},
    },
    repository::ranking::RankingRepository,
};

use crate::AppState;
use crate::error::{WebError, WebResult};

#[utoipa::path(
    get,
    path = "/api/rankings/global",
    params(GlobalRankingFilter),
    responses(
        (status = 200, description = "Global ranking retrieved successfully", body = PaginatedResponse<GlobalRankingEntry>),
        (status = 400, description = "Invalid query parameters")
    ),
    tag = "rankings"
)]
pub async fn get_global_ranking(
    State(state): State<AppState>,
    Query(filter): Query<GlobalRankingFilter>,
) -> WebResult<Json<PaginatedResponse<GlobalRankingEntry>>> {
    filter.validate().map_err(WebError::BadRequest)?;

    let repo = RankingRepository::new(state.db.pool());
    let (entries, total_items) = repo.get_global_ranking(&filter).await?;

    let response = PaginatedResponse::new(
        entries,
        filter.pagination.page,
        filter.pagination.page_size,
        total_items,
    );

    Ok(Json(response))
}
