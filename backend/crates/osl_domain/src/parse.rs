use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{0}")]
pub struct ParseError(String);

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}
