use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
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

pub fn router() -> Router<AppState> {
    Router::new().route("/rankings/global", get(get_global_ranking))
}

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
    Ok(Json(PaginatedResponse::new(
        entries,
        filter.pagination.page,
        filter.pagination.page_size,
        total_items,
    )))
}
