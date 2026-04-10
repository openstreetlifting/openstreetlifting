use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use std::collections::HashSet;

use crate::AppState;
use crate::error::WebError;

pub async fn require_auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, WebError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];
            if state.api_keys.is_valid(token) {
                Ok(next.run(req).await)
            } else {
                tracing::warn!("Invalid API key attempt");
                Err(WebError::Unauthorized)
            }
        }
        _ => Err(WebError::Unauthorized),
    }
}

#[derive(Clone)]
pub struct ApiKeys {
    keys: HashSet<String>,
}

impl ApiKeys {
    pub fn from_comma_separated(keys_str: &str) -> Self {
        let keys = keys_str
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();

        Self { keys }
    }

    pub fn is_valid(&self, key: &str) -> bool {
        self.keys.contains(key)
    }
}
