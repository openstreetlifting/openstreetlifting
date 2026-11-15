use axum::{Router, routing::get};

use super::handler::{get_global_ranking, get_ranking_facets, list_ranking_countries};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/rankings", get(get_global_ranking))
        .route("/rankings/countries", get(list_ranking_countries))
        .route("/rankings/facets", get(get_ranking_facets))
}
