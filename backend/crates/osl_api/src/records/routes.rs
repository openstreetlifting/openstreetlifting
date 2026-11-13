use axum::{Router, routing::get};

use super::handler::{list_record_years, list_records};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/records", get(list_records))
        .route("/records/years", get(list_record_years))
}
