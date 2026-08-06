use axum::{
    Router, middleware,
    routing::{get, post},
};

use super::handlers::{
    calculate_ris, get_current_formula, get_formula_by_year, get_participant_ris_scores,
    list_ris_formulas, recompute_ris_scores,
};
use crate::AppState;
use crate::middleware::auth::require_auth;

pub fn router(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/ris/formulas", get(list_ris_formulas))
        .route("/ris/formulas/current", get(get_current_formula))
        .route("/ris/formulas/{year}", get(get_formula_by_year))
        .route("/ris/calculations", post(calculate_ris))
        .route(
            "/participants/{participant_id}/ris-scores",
            get(get_participant_ris_scores),
        );

    let protected = Router::new()
        .route("/ris/recomputations", post(recompute_ris_scores))
        .route_layer(middleware::from_fn_with_state(state, require_auth));

    public.merge(protected)
}
