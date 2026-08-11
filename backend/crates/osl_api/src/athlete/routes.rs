use axum::{Router, routing::get};

use super::handlers::{get_athlete, list_athletes};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/athletes", get(list_athletes))
        .route("/athletes/{slug}", get(get_athlete))
}
