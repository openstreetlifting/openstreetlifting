use axum::Router;

use crate::AppState;

pub mod athletes;
pub mod competitions;
pub mod health;
pub mod ranking;
pub mod ris;

pub fn api_router(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(ranking::router())
        .merge(ris::router(state.clone()))
        .merge(competitions::router(state.clone()))
        .merge(athletes::router(state))
}
