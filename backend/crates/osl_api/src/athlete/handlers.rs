use crate::AppState;
use crate::error::{WebError, WebResult};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use osl_db::repository::athlete::AthleteRepository;
use osl_db::repository::ranking::RankingRepository;
use serde::Deserialize;

use super::dto::{AthleteResponse, AthleteStanding, StrengthProfile};
use crate::shared::dto::{PaginatedResponse, PaginationParams};
use crate::shared::query::Include;

const ATHLETE_INCLUDES: &[&str] = &["competitions", "records", "standing", "strength_profile"];

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct AthleteQuery {
    #[serde(default)]
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
    query
        .include
        .validate(ATHLETE_INCLUDES)
        .map_err(WebError::BadRequest)?;

    let repo = AthleteRepository::new(state.db.pool());

    if ATHLETE_INCLUDES.iter().any(|name| query.include.has(name)) {
        let athlete = repo.find_by_slug(&slug).await?;
        let athlete_id = athlete.athlete_id;
        let rankings = RankingRepository::new(state.db.pool());
        let (detail, metric_standings, strength_profile) = tokio::try_join!(
            repo.get_detailed_athlete(athlete),
            async {
                if query.include.has("standing") {
                    rankings.get_athlete_metric_standings(athlete_id).await
                } else {
                    Ok(Vec::new())
                }
            },
            async {
                if query.include.has("strength_profile") {
                    repo.strength_profile(athlete_id).await
                } else {
                    Ok(Vec::new())
                }
            }
        )?;

        let mut response = AthleteResponse::from_detail(detail, &query.include);
        response.standing = AthleteStanding::from_rows(metric_standings);
        response.strength_profile = StrengthProfile::from_rows(strength_profile);

        return Ok(Json(response));
    }

    let athlete = repo.find_by_slug(&slug).await?;
    Ok(Json(AthleteResponse::from(athlete)))
}
