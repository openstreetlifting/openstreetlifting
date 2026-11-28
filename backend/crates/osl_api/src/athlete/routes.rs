use axum::{Router, routing::get};

use super::handlers::{get_athlete, list_athletes, search_athletes};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/athletes", get(list_athletes))
        .route("/athletes/search", get(search_athletes))
        .route("/athletes/{slug}", get(get_athlete))
}
