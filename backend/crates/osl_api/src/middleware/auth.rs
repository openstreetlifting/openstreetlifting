use axum::{extract::{Request, State}, middleware::Next, response::Response};
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};
use std::collections::HashSet;

use crate::AppState;
use crate::error::WebError;

pub async fn require_auth(
    State(state): State<AppState>,
    authorization: Option<TypedHeader<Authorization<Bearer>>>,
    req: Request,
    next: Next,
) -> Result<Response, WebError> {
    match authorization {
        Some(TypedHeader(Authorization(bearer))) if state.api_keys.is_valid(bearer.token()) => {
            Ok(next.run(req).await)
        }
        Some(_) => {
            tracing::warn!("Invalid API key attempt");
            Err(WebError::Unauthorized)
        }
        None => Err(WebError::Unauthorized),
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
