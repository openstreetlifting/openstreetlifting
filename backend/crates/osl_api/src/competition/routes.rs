use axum::{
    Router, middleware,
    routing::{get, post},
};

use super::handlers::{
    create_competition, delete_competition, get_competition, list_competitions, update_competition,
};
use crate::AppState;
use crate::middleware::auth::require_auth;

pub fn router(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/competitions", get(list_competitions))
        .route("/competitions/{slug}", get(get_competition));

    let protected = Router::new()
        .route("/competitions", post(create_competition))
        .route(
            "/competitions/{slug}",
            axum::routing::patch(update_competition).delete(delete_competition),
        )
        .route_layer(middleware::from_fn_with_state(state, require_auth));

    public.merge(protected)
}
