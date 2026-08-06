use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde_json::{Value, json};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
}

async fn liveness() -> StatusCode {
    StatusCode::OK
}

async fn readiness(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match state.db.ping().await {
        Ok(_) => (StatusCode::OK, Json(json!({"status": "ok"}))),
        Err(e) => {
            tracing::error!("Readiness check failed: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"status": "unavailable"})),
            )
        }
    }
}
