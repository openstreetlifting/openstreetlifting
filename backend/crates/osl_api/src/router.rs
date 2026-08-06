use axum::Router;

use crate::AppState;

/// Everything mounted under `/api`.
///
/// Each slice owns its own routes; this is the only place that knows the
/// full set.
pub fn api_router(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(crate::ranking::routes::router())
        .merge(crate::ris::routes::router(state.clone()))
        .merge(crate::competition::routes::router(state.clone()))
        .merge(crate::athlete::routes::router(state))
}
