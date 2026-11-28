use crate::AppState;
use crate::error::{WebError, WebResult};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use osl_db::projections::athlete::AthleteRecords;
use osl_db::repository::athlete::AthleteRepository;
use osl_db::repository::ranking::RankingRepository;
use serde::Deserialize;
use std::collections::HashMap;

use super::dto::{
    AthleteInclude, AthleteNeighborhood, AthleteResponse, AthleteSearchHit, AthleteSearchResponse,
    AthleteStanding, LiftBalance,
};
use crate::shared::dto::{PaginatedResponse, PaginationParams};
use crate::shared::query::{Include, load_if};
use osl_db::repository::name_search::has_name_terms;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct AthleteQuery {
    /// Sections to include, comma-separated.
    #[serde(default)]
    #[param(value_type = Vec<AthleteInclude>, style = Form, explode = false)]
    pub include: Include,
}

/// List all athletes
///
/// Returns athletes sorted by last name.
#[utoipa::path(
    get,
    path = "/api/v1/athletes",
    params(PaginationParams),
    responses(
        (status = 200, description = "A page of athletes", body = PaginatedResponse<AthleteResponse>),
        (status = 400, description = "Invalid pagination parameters", body = crate::error::ErrorResponse)
    ),
    tag = "athletes"
)]
pub async fn list_athletes(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> WebResult<Json<PaginatedResponse<AthleteResponse>>> {
    pagination.validate().map_err(WebError::BadRequest)?;

    let repo = AthleteRepository::new(state.db.pool());
    let (athletes, total_items) = repo.list(&pagination.to_page()).await?;

    let athlete_ids: Vec<_> = athletes.iter().map(|athlete| athlete.athlete_id).collect();
    let mut handles: HashMap<_, _> = repo
        .instagram_handles(&athlete_ids)
        .await?
        .into_iter()
        .collect();
    let data: Vec<AthleteResponse> = athletes
        .into_iter()
        .map(|athlete| {
            let instagram_handle = handles.remove(&athlete.athlete_id);
            let mut response = AthleteResponse::from(athlete);
            response.instagram_handle = instagram_handle;
            response
        })
        .collect();

    Ok(Json(PaginatedResponse::new(
        data,
        pagination.page,
        pagination.page_size,
        total_items,
    )))
}

const DEFAULT_SEARCH_LIMIT: u32 = 6;
const MAX_SEARCH_LIMIT: u32 = 20;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AthleteSearchQuery {
    /// Name to search, in any order, ignoring case and accents.
    pub q: String,
    /// Maximum number of athletes, from 1 to 20. Defaults to 6.
    pub limit: Option<u32>,
    /// Keep only athletes without an MPDS result. Defaults to `false`.
    #[serde(default)]
    pub without_full_result: bool,
}

/// Search athletes by name
///
/// Matches every athlete, ranked or not. Each word of `q` must appear in the name.
#[utoipa::path(
    get,
    path = "/api/v1/athletes/search",
    params(AthleteSearchQuery),
    responses(
        (status = 200, description = "Matching athletes", body = AthleteSearchResponse),
        (status = 400, description = "Invalid limit", body = crate::error::ErrorResponse)
    ),
    tag = "athletes"
)]
pub async fn search_athletes(
    State(state): State<AppState>,
    Query(query): Query<AthleteSearchQuery>,
) -> WebResult<Json<AthleteSearchResponse>> {
    let limit = query.limit.unwrap_or(DEFAULT_SEARCH_LIMIT);
    if !(1..=MAX_SEARCH_LIMIT).contains(&limit) {
        return Err(WebError::BadRequest(format!(
            "limit must be between 1 and {MAX_SEARCH_LIMIT}"
        )));
    }
    if !has_name_terms(&query.q) {
        return Ok(Json(AthleteSearchResponse {
            data: Vec::new(),
            total_items: 0,
        }));
    }

    let (rows, total_items) = AthleteRepository::new(state.db.pool())
        .search(&query.q, query.without_full_result, i64::from(limit))
        .await?;

    Ok(Json(AthleteSearchResponse {
        data: rows.into_iter().map(AthleteSearchHit::from).collect(),
        total_items,
    }))
}

/// Retrieve an athlete
///
/// Returns an athlete. Use `include` to add sections.
#[utoipa::path(
    get,
    path = "/api/v1/athletes/{slug}",
    params(
        ("slug" = String, Path, description = "Athlete slug"),
        AthleteQuery,
    ),
    responses(
        (status = 200, description = "Athlete found", body = AthleteResponse),
        (status = 400, description = "Unknown include value", body = crate::error::ErrorResponse),
        (status = 404, description = "Athlete not found", body = crate::error::ErrorResponse)
    ),
    tag = "athletes"
)]
pub async fn get_athlete(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<AthleteQuery>,
) -> WebResult<Json<AthleteResponse>> {
    let include: Vec<AthleteInclude> = query.include.parse_as().map_err(WebError::BadRequest)?;
    let wants = |section| include.contains(&section);

    let repo = AthleteRepository::new(state.db.pool());
    let athlete = repo.find_by_slug(&slug).await?;
    let athlete_id = athlete.athlete_id;
    let rankings = RankingRepository::new(state.db.pool());
    let wants_competitions = wants(AthleteInclude::Competitions);
    let wants_records = wants(AthleteInclude::Records);
    let (history, instagram_handle, standing, lift_balance, neighbors) = tokio::try_join!(
        load_if(
            wants_competitions || wants_records,
            repo.competition_history(athlete_id)
        ),
        repo.instagram_handle(athlete_id),
        load_if(
            wants(AthleteInclude::Standing),
            rankings.get_athlete_metric_standings_stored(athlete_id)
        ),
        load_if(
            wants(AthleteInclude::LiftBalance),
            repo.lift_balance(athlete_id)
        ),
        load_if(
            wants(AthleteInclude::Neighbors),
            rankings.get_athlete_total_neighbors_stored(athlete_id)
        ),
    )?;

    let mut response = AthleteResponse::from(athlete);
    response.instagram_handle = instagram_handle;
    if let Some(history) = history {
        if wants_records {
            response.set_records(AthleteRecords::from_history(&history).into());
        }
        if wants_competitions {
            response.set_history(history.into());
        }
    }
    response.standing = standing.map(AthleteStanding::from_rows);
    response.lift_balance = lift_balance.map(LiftBalance::from_rows);
    response.neighbors =
        neighbors.map(|rows| AthleteNeighborhood::from_rows(rows, response.gender));

    Ok(Json(response))
}
