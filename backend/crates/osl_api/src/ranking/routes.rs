use axum::{Router, routing::get};

use super::handler::get_global_ranking;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/rankings", get(get_global_ranking))
}
