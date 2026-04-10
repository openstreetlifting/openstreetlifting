use axum::{Router, middleware, routing::{get, post}};

use crate::AppState;
use crate::handlers::competitions::{
    create_competition, delete_competition, get_competition, get_competition_detailed,
    list_competitions, list_competitions_detailed, update_competition,
};
use crate::middleware::auth::require_auth;

pub fn router(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/competitions", get(list_competitions))
        .route("/competitions/detailed", get(list_competitions_detailed))
        .route("/competitions/{slug}", get(get_competition))
        .route(
            "/competitions/{slug}/detailed",
            get(get_competition_detailed),
        );

    let protected = Router::new()
        .route("/competitions", post(create_competition))
        .route(
            "/competitions/{slug}",
            axum::routing::put(update_competition).delete(delete_competition),
        )
        .route_layer(middleware::from_fn_with_state(state, require_auth));

    public.merge(protected)
}
