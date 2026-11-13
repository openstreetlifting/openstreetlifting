use axum::{
    Json,
    extract::{Query, State},
};
use osl_db::repository::records::RecordsRepository;

use super::dto::{RecordGroup, RecordsQuery, group};
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
