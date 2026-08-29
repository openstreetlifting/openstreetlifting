use axum::Router;

use crate::AppState;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .merge(crate::ranking::routes::router())
        .merge(crate::ris::routes::router())
        .merge(crate::competition::routes::router())
        .merge(crate::athlete::routes::router())
}
