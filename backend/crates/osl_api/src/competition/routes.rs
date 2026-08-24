use axum::{Router, routing::get};

use super::handlers::{
    get_competition, list_competition_countries, list_competition_federations,
    list_competition_years, list_competitions,
};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/competitions", get(list_competitions))
        .route(
            "/competitions/federations",
            get(list_competition_federations),
        )
        .route("/competitions/years", get(list_competition_years))
        .route("/competitions/countries", get(list_competition_countries))
        .route("/competitions/{slug}", get(get_competition))
}
