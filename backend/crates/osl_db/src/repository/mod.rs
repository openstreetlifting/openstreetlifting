pub mod athlete;
pub mod competition;
pub mod ranking;

use osl_domain::Gender;
use std::str::FromStr;

use crate::error::{Result, StorageError};

/// The gender column is constrained to M, F or MX, so anything else means the
/// database has been written to by something that is not this application.
pub(crate) fn parse_gender(raw: &str) -> Result<Gender> {
    Gender::from_str(raw).map_err(StorageError::ConstraintViolation)
}
