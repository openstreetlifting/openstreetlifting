use thiserror::Error;

/// A value read from outside that does not name a variant of the type it was
/// read into.
///
/// Every enum here parses from a string at some boundary: a database column, a
/// query parameter, a canonical file. `FromStr` reports that as a plain message
/// because most callers only want to show it, but a decoder has to hand back
/// something that is an `Error`, so this wraps the same message in one.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{0}")]
pub struct ParseError(String);

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl From<String> for ParseError {
    fn from(message: String) -> Self {
        Self(message)
    }
}
