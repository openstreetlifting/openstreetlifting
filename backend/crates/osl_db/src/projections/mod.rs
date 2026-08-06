//! Query-shaped results that do not correspond to a single table.
//!
//! Rows in `crate::rows` mirror tables one-for-one. Projections are what
//! multi-table joins and aggregate queries return. Like rows they carry
//! no serde or utoipa derives: osl_api maps them onto its own response
//! DTOs.

pub mod athlete;
pub mod competition;
pub mod ranking;
