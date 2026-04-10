use axum::{Router, middleware, routing::{get, post}};

use crate::AppState;
use crate::handlers::ris::{
    compute_ris, get_current_formula, get_formula_by_year, get_participant_ris_history,
    list_ris_formulas, recompute_all_ris,
};
use crate::middleware::auth::require_auth;

pub fn router(state: AppState) -> Router<AppState> {
    let ris_routes = Router::new()
        .route("/ris/formulas", get(list_ris_formulas))
        .route("/ris/formulas/current", get(get_current_formula))
        .route("/ris/formulas/{year}", get(get_formula_by_year))
        .route("/ris/compute", post(compute_ris));

    let participant_routes = Router::new().route(
        "/participants/{participant_id}/ris-history",
        get(get_participant_ris_history),
    );

    let admin_routes = Router::new()
        .route("/admin/ris/recompute-all", post(recompute_all_ris))
        .route_layer(middleware::from_fn_with_state(state, require_auth));

    ris_routes.merge(participant_routes).merge(admin_routes)
}
