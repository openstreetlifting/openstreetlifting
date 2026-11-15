use axum::{
    Json,
    extract::{Query, State},
};
use osl_db::repository::records::RecordsRepository;

use super::dto::{RecordFacets, RecordGroup, RecordSeries, RecordsQuery, group};
use crate::AppState;
use crate::error::{WebError, WebResult};

#[utoipa::path(
    get,
    path = "/api/v1/records",
    params(RecordsQuery),
    responses(
        (status = 200, description = "The best results in the OpenStreetlifting archive, grouped by movement, gender and weight class. The open group comes first in each gender.", body = Vec<RecordGroup>),
        (status = 400, description = "Invalid query parameters")
    ),
    tag = "records"
)]
pub async fn list_records(
    State(state): State<AppState>,
    Query(query): Query<RecordsQuery>,
) -> WebResult<Json<Vec<RecordGroup>>> {
    let filter = query.to_db_filter().map_err(WebError::BadRequest)?;
    let rows = RecordsRepository::new(state.db.pool())
        .list(&filter)
        .await?;

    Ok(Json(group(rows)))
}

#[utoipa::path(
    get,
    path = "/api/v1/records/facets",
    params(RecordsQuery),
    responses(
        (status = 200, description = "Values each records filter can take given the other filters", body = RecordFacets),
        (status = 400, description = "Invalid query parameters")
    ),
    tag = "records"
)]
pub async fn get_record_facets(
    State(state): State<AppState>,
    Query(query): Query<RecordsQuery>,
) -> WebResult<Json<RecordFacets>> {
    let filter = query.to_db_filter().map_err(WebError::BadRequest)?;
    let facets = RecordsRepository::new(state.db.pool())
        .facets(&filter.by)
        .await?;

    Ok(Json(RecordFacets {
        countries: facets.countries,
        federations: facets.federations,
        series: facets
            .series
            .into_iter()
            .map(|option| RecordSeries {
                series: option.series,
                name: option.name,
                federation: option.federation,
            })
            .collect(),
        years: facets.years,
        custom_weight_classes: facets.custom_weight_classes,
    }))
}
