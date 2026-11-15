use axum::{Router, routing::get};

use super::handler::{get_record_facets, list_records};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/records", get(list_records))
        .route("/records/facets", get(get_record_facets))
}
