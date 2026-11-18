use crate::AppState;
use crate::error::{WebError, WebResult};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use osl_db::filter::Filters;
use osl_db::params::{CompetitionBy, CompetitionFilter};
use osl_db::repository::competition::CompetitionRepository;
use osl_domain::CompetitionStatus;
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

use super::dto::{
    CompetitionFacets, CompetitionInclude, CompetitionListInclude, CompetitionResponse,
    FederationInfo,
};
use crate::shared::dto::{Direction, PaginatedResponse, PaginationParams, optional_from_str};
use crate::shared::query::{Include, load_if};

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CompetitionFilterParams {
    #[serde(default, deserialize_with = "optional_from_str")]
    #[param(nullable)]
    pub status: Option<CompetitionStatus>,
    #[param(nullable)]
    pub event: Option<String>,
    #[param(nullable)]
    pub federation: Option<String>,
    #[param(nullable)]
    pub country: Option<String>,
    #[serde(default, deserialize_with = "optional_from_str")]
    #[param(nullable)]
    pub year: Option<i32>,
}

impl CompetitionFilterParams {
    fn filters(&self) -> Filters<CompetitionBy> {
        [
            self.status.map(CompetitionBy::Status),
            self.event.clone().map(CompetitionBy::Event),
            self.federation.clone().map(CompetitionBy::Federation),
            self.country.clone().map(CompetitionBy::Country),
            self.year.map(CompetitionBy::Year),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn validate_event(&self) -> Result<(), WebError> {
        if let Some(event) = &self.event {
            osl_domain::event::movements(event).map_err(WebError::BadRequest)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CompetitionListQuery {
    #[serde(default)]
    #[param(value_type = Vec<CompetitionListInclude>, style = Form, explode = false)]
    pub include: Include,
    #[param(nullable)]
    pub q: Option<String>,
    #[serde(default)]
    pub direction: Direction,
    #[serde(flatten)]
    #[param(ignore)]
    pub filters: CompetitionFilterParams,
    #[serde(flatten)]
    #[param(ignore)]
    pub pagination: PaginationParams,
}

impl CompetitionListQuery {
    fn to_db_filter(&self) -> CompetitionFilter {
        CompetitionFilter {
            by: self.filters.filters(),
            search: self
                .q
                .as_deref()
                .map(str::trim)
                .filter(|query| !query.is_empty())
                .map(str::to_string),
            direction: self.direction.into(),
        }
    }
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct CompetitionQuery {
    #[serde(default)]
    #[param(value_type = Vec<CompetitionInclude>, style = Form, explode = false)]
    pub include: Include,
}

#[utoipa::path(
    get,
    path = "/api/v1/competitions",
    params(PaginationParams, CompetitionFilterParams, CompetitionListQuery),
    responses(
        (status = 200, description = "A page of competitions", body = PaginatedResponse<CompetitionResponse>),
        (status = 400, description = "Invalid pagination, include value or event code")
    ),
    tag = "competitions"
)]
pub async fn list_competitions(
    State(state): State<AppState>,
    Query(query): Query<CompetitionListQuery>,
) -> WebResult<Json<PaginatedResponse<CompetitionResponse>>> {
    query.pagination.validate().map_err(WebError::BadRequest)?;
    query.filters.validate_event()?;
    let include: Vec<CompetitionListInclude> =
        query.include.parse_as().map_err(WebError::BadRequest)?;
    let wants = |section| include.contains(&section);

    let repo = CompetitionRepository::new(state.db.pool());
    let page = query.pagination.to_page();
    let filter = query.to_db_filter();
    let (rows, total_items) = repo.list(&page, &filter).await?;

    let competition_ids: Vec<Uuid> = rows
        .iter()
        .map(|row| row.competition.competition_id)
        .collect();
    let federation_ids: Vec<Uuid> = rows
        .iter()
        .map(|row| row.competition.federation_id)
        .collect();
    let (federations, movements) = tokio::try_join!(
        load_if(
            wants(CompetitionListInclude::Federation),
            repo.federations(&federation_ids)
        ),
        load_if(
            wants(CompetitionListInclude::Movements),
            repo.competition_movements(&competition_ids)
        ),
    )?;

    let federations: Option<HashMap<Uuid, FederationInfo>> = federations.map(|rows| {
        rows.into_iter()
            .map(|row| (row.federation_id, row.into()))
            .collect()
    });
    let mut movements = movements.map(|rows| {
        let mut by_competition: HashMap<Uuid, Vec<_>> = HashMap::new();
        for row in rows {
            by_competition
                .entry(row.competition_id)
                .or_default()
                .push(row);
        }
        by_competition
    });

    let data = rows
        .into_iter()
        .map(|row| {
            let competition_id = row.competition.competition_id;
            let federation_id = row.competition.federation_id;
            let mut response = CompetitionResponse::from(row.competition);
            response.lifter_count = Some(row.lifter_count);
            if let Some(federations) = &federations {
                response.federation = federations.get(&federation_id).cloned();
            }
            if let Some(movements) = &mut movements {
                response.set_movements(movements.remove(&competition_id).unwrap_or_default());
            }
            response
        })
        .collect();

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
    let include: Vec<CompetitionInclude> =
        query.include.parse_as().map_err(WebError::BadRequest)?;
    let wants = |section| include.contains(&section);

    let repo = CompetitionRepository::new(state.db.pool());
    let competition = repo.find_by_slug(&slug).await?;
    let competition_id = competition.competition_id;
    let federation_id = competition.federation_id;
    let (federation, movements, results, editions) = tokio::try_join!(
        load_if(
            wants(CompetitionInclude::Federation),
            repo.federation(federation_id)
        ),
        load_if(
            wants(CompetitionInclude::Movements),
            repo.movements(competition_id)
        ),
        load_if(
            wants(CompetitionInclude::Results),
            repo.results(competition_id)
        ),
        load_if(wants(CompetitionInclude::Editions), repo.editions(&slug)),
    )?;

    let mut response = CompetitionResponse::from(competition);
    response.federation = federation.map(Into::into);
    if let Some(movements) = movements {
        response.set_movements(movements);
    }
    if let Some(results) = results {
        response.set_results(results);
    }
    response.editions = editions.map(|rows| rows.into_iter().map(Into::into).collect());

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/competitions/facets",
    params(CompetitionFilterParams),
    responses(
        (status = 200, description = "Values each competition filter can take given the other filters.", body = CompetitionFacets),
        (status = 400, description = "Invalid event code")
    ),
    tag = "competitions"
)]
pub async fn get_competition_facets(
    State(state): State<AppState>,
    Query(query): Query<CompetitionFilterParams>,
) -> WebResult<Json<CompetitionFacets>> {
    query.validate_event()?;

    let facets = CompetitionRepository::new(state.db.pool())
        .facets(&query.filters())
        .await?;

    Ok(Json(CompetitionFacets {
        federations: facets.federations,
        countries: facets.countries,
        years: facets.years,
        formats: facets.formats,
    }))
}
