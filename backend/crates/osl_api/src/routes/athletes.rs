use axum::{Router, middleware, routing::{get, post}};

use crate::AppState;
use crate::handlers::athletes::{
    create_athlete, delete_athlete, get_athlete, get_athlete_detailed, list_athletes,
    update_athlete,
};
use crate::middleware::auth::require_auth;

pub fn router(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/athletes", get(list_athletes))
        .route("/athletes/{slug}", get(get_athlete))
        .route("/athletes/{slug}/detailed", get(get_athlete_detailed));

    let protected = Router::new()
        .route("/athletes", post(create_athlete))
        .route(
            "/athletes/{slug}",
            axum::routing::put(update_athlete).delete(delete_athlete),
        )
        .route_layer(middleware::from_fn_with_state(state, require_auth));

    public.merge(protected)
}
