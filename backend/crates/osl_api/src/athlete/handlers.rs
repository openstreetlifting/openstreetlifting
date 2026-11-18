use crate::AppState;
use crate::error::{WebError, WebResult};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use osl_db::repository::athlete::AthleteRepository;
use osl_db::repository::ranking::RankingRepository;
use serde::Deserialize;

use super::dto::{
    AthleteInclude, AthleteNeighborhood, AthleteResponse, AthleteStanding, LiftBalance,
};
use crate::shared::dto::{PaginatedResponse, PaginationParams};
use crate::shared::query::Include;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct AthleteQuery {
    #[serde(default)]
    #[param(value_type = Vec<AthleteInclude>, style = Form, explode = false)]
    pub include: Include,
}

#[utoipa::path(
    get,
    path = "/api/v1/athletes",
    params(PaginationParams),
    responses(
        (status = 200, description = "A page of athletes", body = PaginatedResponse<AthleteResponse>),
        (status = 400, description = "Invalid pagination parameters")
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

    let data: Vec<AthleteResponse> = athletes.into_iter().map(AthleteResponse::from).collect();

    Ok(Json(PaginatedResponse::new(
        data,
        pagination.page,
        pagination.page_size,
        total_items,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/athletes/{slug}",
    params(
        ("slug" = String, Path, description = "Athlete slug"),
        AthleteQuery,
    ),
    responses(
        (status = 200, description = "Athlete found", body = AthleteResponse),
        (status = 400, description = "Unknown include value"),
        (status = 404, description = "Athlete not found")
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

    if !include.is_empty() {
        let athlete = repo.find_by_slug(&slug).await?;
        let athlete_id = athlete.athlete_id;
        let rankings = RankingRepository::new(state.db.pool());
        let (detail, metric_standings, lift_balance, neighbors) = tokio::try_join!(
            repo.get_detailed_athlete(athlete),
            async {
                if wants(AthleteInclude::Standing) {
                    rankings
                        .get_athlete_metric_standings_stored(athlete_id)
                        .await
                } else {
                    Ok(Vec::new())
                }
            },
            async {
                if wants(AthleteInclude::LiftBalance) {
                    repo.lift_balance(athlete_id).await
                } else {
                    Ok(Vec::new())
                }
            },
            async {
                if wants(AthleteInclude::Neighbors) {
                    rankings
                        .get_athlete_total_neighbors_stored(athlete_id)
                        .await
                } else {
                    Ok(Vec::new())
                }
            }
        )?;

        let mut response = AthleteResponse::from_detail(detail, &include);
        response.standing =
            wants(AthleteInclude::Standing).then(|| AthleteStanding::from_rows(metric_standings));
        response.lift_balance =
            wants(AthleteInclude::LiftBalance).then(|| LiftBalance::from_rows(lift_balance));
        response.neighbors = wants(AthleteInclude::Neighbors)
            .then(|| AthleteNeighborhood::from_rows(neighbors, response.gender));

        return Ok(Json(response));
    }

    let athlete = repo.find_by_slug(&slug).await?;
    Ok(Json(AthleteResponse::from(athlete)))
}
