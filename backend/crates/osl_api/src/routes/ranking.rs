use axum::{Router, routing::get};

use crate::AppState;
use crate::handlers::ranking::get_global_ranking;

pub fn router() -> Router<AppState> {
    Router::new().route("/rankings/global", get(get_global_ranking))
}
