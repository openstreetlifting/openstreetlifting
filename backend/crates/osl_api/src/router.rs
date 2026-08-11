use axum::Router;

use crate::AppState;

/// Each slice owns its routes; this is the only place that knows the full set.
pub fn api_router() -> Router<AppState> {
    Router::new()
        .merge(crate::ranking::routes::router())
        .merge(crate::ris::routes::router())
        .merge(crate::competition::routes::router())
        .merge(crate::athlete::routes::router())
}
