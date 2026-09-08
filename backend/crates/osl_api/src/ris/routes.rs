use axum::{
    Router,
    routing::{get, post},
};

use super::handlers::{
    calculate_ris, get_current_formula, get_formula_by_year, get_ris_distribution,
    list_ris_formulas,
};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ris/formulas", get(list_ris_formulas))
        .route("/ris/formulas/current", get(get_current_formula))
        .route("/ris/formulas/{year}", get(get_formula_by_year))
        // The archive's own performances, for plotting against the curve.
        .route("/ris/distribution", get(get_ris_distribution))
        // Computes a score from a bodyweight and a total. Writes nothing.
        .route("/ris/calculations", post(calculate_ris))
}
