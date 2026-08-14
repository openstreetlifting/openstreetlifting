use axum::{Router, routing::get};

use super::handler::{
    get_global_ranking, list_ranking_classes, list_ranking_countries, list_ranking_years,
};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/rankings", get(get_global_ranking))
        .route("/rankings/classes", get(list_ranking_classes))
        .route("/rankings/countries", get(list_ranking_countries))
        .route("/rankings/years", get(list_ranking_years))
}
