use axum::{Router, routing::get};

use super::handler::list_records;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/records", get(list_records))
}
