use axum::{
    Router, middleware,
    routing::{get, post},
};

use super::handlers::{create_athlete, delete_athlete, get_athlete, list_athletes, update_athlete};
use crate::AppState;
use crate::middleware::auth::require_auth;

pub fn router(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/athletes", get(list_athletes))
        .route("/athletes/{slug}", get(get_athlete));

    let protected = Router::new()
        .route("/athletes", post(create_athlete))
        .route(
            "/athletes/{slug}",
            axum::routing::patch(update_athlete).delete(delete_athlete),
        )
        .route_layer(middleware::from_fn_with_state(state, require_auth));

    public.merge(protected)
}
