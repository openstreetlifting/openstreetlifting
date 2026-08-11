use axum::{Router, routing::get};

use super::handlers::{get_competition, list_competitions};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/competitions", get(list_competitions))
        .route("/competitions/{slug}", get(get_competition))
}
