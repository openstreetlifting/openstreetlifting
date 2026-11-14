use axum::{
    Json,
    extract::{Query, State},
};
use std::collections::HashMap;

use osl_db::repository::ranking::RankingRepository;
use osl_db::repository::records::RecordsRepository;

use super::dto::{
    ClassesFilter, CountriesFilter, GlobalRankingEntry, GlobalRankingFilter, LiftRecord,
};
use crate::AppState;
use crate::error::{WebError, WebResult};
use crate::shared::dto::{PaginatedResponse, PaginationParams};

#[utoipa::path(
    get,
    path = "/api/v1/rankings/series",
    responses((status = 200, description = "Competition series with results", body = Vec<super::dto::RankingSeries>)),
    tag = "rankings"
)]
pub async fn list_ranking_series(
    State(state): State<AppState>,
) -> WebResult<Json<Vec<super::dto::RankingSeries>>> {
    let series = RankingRepository::new(state.db.pool())
        .list_distinct_series()
        .await?
        .into_iter()
        .map(|(series, name, federation)| super::dto::RankingSeries {
            series,
            name,
            federation,
        })
        .collect();
    Ok(Json(series))
}

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
        .list_distinct_classes(filter.gender, filter.competition_id)
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
