use axum::{
    Json,
    extract::{Query, State},
};
use osl_db::repository::ranking::RankingRepository;

use super::dto::{ClassesFilter, CountriesFilter, GlobalRankingEntry, GlobalRankingFilter};
use crate::AppState;
use crate::error::{WebError, WebResult};
use crate::shared::dto::PaginatedResponse;

#[utoipa::path(
    get,
    path = "/api/v1/rankings/classes",
    params(ClassesFilter),
    responses(
        (status = 200, description = "Distinct weight classes, optionally narrowed to one gender", body = Vec<String>),
    ),
    tag = "rankings"
)]
pub async fn list_ranking_classes(
    State(state): State<AppState>,
    Query(filter): Query<ClassesFilter>,
) -> WebResult<Json<Vec<String>>> {
    let repo = RankingRepository::new(state.db.pool());
    let classes = repo
        .list_distinct_classes(filter.gender.as_deref(), filter.competition_id)
        .await?;

    Ok(Json(classes))
}

#[utoipa::path(
    get,
    path = "/api/v1/rankings/countries",
    params(CountriesFilter),
    responses(
        (status = 200, description = "Distinct countries athletes have competed under, optionally narrowed to one competition", body = Vec<String>),
    ),
    tag = "rankings"
)]
pub async fn list_ranking_countries(
    State(state): State<AppState>,
    Query(filter): Query<CountriesFilter>,
) -> WebResult<Json<Vec<String>>> {
    let repo = RankingRepository::new(state.db.pool());
    let countries = repo.list_distinct_countries(filter.competition_id).await?;

    Ok(Json(countries))
}

#[utoipa::path(
    get,
    path = "/api/v1/rankings/federations",
    responses(
        (status = 200, description = "Federations athletes have competed under, alphabetical", body = Vec<String>),
    ),
    tag = "rankings"
)]
pub async fn list_ranking_federations(
    State(state): State<AppState>,
) -> WebResult<Json<Vec<String>>> {
    let repo = RankingRepository::new(state.db.pool());
    let federations = repo.list_distinct_federations().await?;

    Ok(Json(federations))
}

#[utoipa::path(
    get,
    path = "/api/v1/rankings/years",
    responses(
        (status = 200, description = "Distinct competition years, most recent first", body = Vec<i32>),
    ),
    tag = "rankings"
)]
pub async fn list_ranking_years(State(state): State<AppState>) -> WebResult<Json<Vec<i32>>> {
    let repo = RankingRepository::new(state.db.pool());
    let years = repo.list_distinct_years().await?;

    Ok(Json(years))
}

#[utoipa::path(
    get,
    path = "/api/v1/rankings",
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
    let (rows, total_items) = repo.get_global_ranking(&filter.to_db_filter()).await?;

    let entries: Vec<GlobalRankingEntry> = rows.into_iter().map(GlobalRankingEntry::from).collect();

    let response = PaginatedResponse::new(
        entries,
        filter.pagination.page,
        filter.pagination.page_size,
        total_items,
    );

    Ok(Json(response))
}
