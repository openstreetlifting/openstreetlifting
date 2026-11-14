use axum::{
    Json,
    extract::{Query, State},
};
use std::collections::HashMap;

use osl_db::repository::ranking::RankingRepository;
use osl_db::repository::records::RecordsRepository;

use super::dto::{GlobalRankingEntry, GlobalRankingFilter, LiftRecord, RankingFacets};
use crate::AppState;
use crate::error::{WebError, WebResult};
use crate::shared::dto::{PaginatedResponse, PaginationParams};

#[utoipa::path(
    get,
    path = "/api/v1/rankings/facets",
    params(GlobalRankingFilter),
    responses(
        (status = 200, description = "Values each ranking filter can take given the other filters. Search, sort and paging are ignored.", body = RankingFacets),
    ),
    tag = "rankings"
)]
pub async fn get_ranking_facets(
    State(state): State<AppState>,
    Query(filter): Query<GlobalRankingFilter>,
) -> WebResult<Json<RankingFacets>> {
    let facets = RankingRepository::new(state.db.pool())
        .facets(&filter.filters())
        .await?;

    Ok(Json(facets.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/rankings/countries",
    responses(
        (status = 200, description = "Distinct countries athletes have competed under", body = Vec<String>),
    ),
    tag = "rankings"
)]
pub async fn list_ranking_countries(State(state): State<AppState>) -> WebResult<Json<Vec<String>>> {
    let repo = RankingRepository::new(state.db.pool());
    let countries = repo.list_distinct_countries().await?;

    Ok(Json(countries))
}

#[utoipa::path(
    get,
    path = "/api/v1/rankings",
    params(PaginationParams, GlobalRankingFilter),
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
    let (rows, total_items) = repo.get_global_ranking(&filter.to_db_filter()).await?;

    let participant_ids: Vec<_> = rows.iter().map(|row| row.participant_id).collect();
    let marks = RecordsRepository::new(state.db.pool())
        .marks(&participant_ids)
        .await?;
    let mut records: HashMap<_, Vec<LiftRecord>> = HashMap::new();
    for (participant_id, movement, mark) in marks {
        records
            .entry(participant_id)
            .or_default()
            .push(LiftRecord { movement, mark });
    }
    let entries: Vec<GlobalRankingEntry> = rows
        .into_iter()
        .map(|row| {
            let records = records.remove(&row.participant_id).unwrap_or_default();
            GlobalRankingEntry::new(row, records)
        })
        .collect();

    let response = PaginatedResponse::new(
        entries,
        filter.pagination.page,
        filter.pagination.page_size,
        total_items,
    );

    Ok(Json(response))
}
