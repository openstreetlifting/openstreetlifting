#![expect(clippy::option_if_let_else)]

use osl_db::params::{Page, SortDirection};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr<Err = String>,
{
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

pub fn optional_from_str<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr<Err = String>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|raw| raw.parse().map_err(serde::de::Error::custom))
        .transpose()
}

#[derive(Debug, Default, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct PaginationParams {
    #[serde(default = "default_page", deserialize_with = "number_from_query")]
    pub page: u32,
    #[serde(default = "default_page_size", deserialize_with = "number_from_query")]
    pub page_size: u32,
}

fn number_from_query<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    use serde::de::Error;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumberOrString {
        Number(u32),
        String(String),
    }

    match NumberOrString::deserialize(deserializer)? {
        NumberOrString::Number(n) => Ok(n),
        NumberOrString::String(s) => s.parse().map_err(D::Error::custom),
    }
}

const fn default_page() -> u32 {
    1
}

const fn default_page_size() -> u32 {
    50
}

impl PaginationParams {
    pub fn validate(&self) -> Result<(), String> {
        if self.page < 1 {
            return Err("page must be >= 1".to_string());
        }
        if self.page_size < 1 || self.page_size > 100 {
            return Err("page_size must be between 1 and 100".to_string());
        }
        Ok(())
    }

    #[must_use]
    pub fn offset(&self) -> i64 {
        (i64::from(self.page) - 1) * i64::from(self.page_size)
    }

    #[must_use]
    pub const fn limit(&self) -> u32 {
        self.page_size
    }

    #[must_use]
    pub fn to_page(&self) -> Page {
        Page {
            limit: i64::from(self.limit()),
            offset: self.offset(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub page: u32,
    pub page_size: u32,
    pub total_items: i64,
    pub total_pages: u32,
}

impl PaginationMeta {
    /// # Panics
    ///
    /// Panics if `page_size` is zero; query validation requires positive size.
    #[must_use]
    pub fn new(page: u32, page_size: u32, total_items: i64) -> Self {
        let items = u64::try_from(total_items).unwrap_or_default();
        let pages = items.div_ceil(u64::from(page_size));
        let total_pages = u32::try_from(pages).unwrap_or(u32::MAX);
        Self {
            page,
            page_size,
            total_items,
            total_pages,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

impl<T> PaginatedResponse<T> {
    #[must_use]
    pub fn new(data: Vec<T>, page: u32, page_size: u32, total_items: i64) -> Self {
        Self {
            data,
            pagination: PaginationMeta::new(page, page_size, total_items),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[default]
    Desc,
    Asc,
}

impl From<Direction> for SortDirection {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Desc => Self::Desc,
            Direction::Asc => Self::Asc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PaginationMeta, PaginationParams};
    use axum::extract::Query;
    use osl_domain::{CompetitionStatus, Gender};
    use serde_json::json;

    use crate::competition::handlers::CompetitionListQuery;
    use crate::ranking::dto::{ClassesFilter, GlobalRankingFilter};
    use crate::ris::dto::ComputeRisRequest;
    use crate::shared::filters::RankedGender;

    #[test]
    fn rounds_up_page_counts() {
        for (total_items, page_size, expected_pages) in [
            (0, 50, 0),
            (50, 50, 1),
            (51, 50, 2),
            (i64::from(u32::MAX) * 100, 100, u32::MAX),
            (i64::MAX, 100, u32::MAX),
        ] {
            let meta = PaginationMeta::new(1, page_size, total_items);
            assert_eq!(meta.total_items, total_items);
            assert_eq!(meta.total_pages, expected_pages);
        }
    }

    #[test]
    fn preserves_large_pagination_offsets() {
        let pagination = PaginationParams {
            page: u32::MAX,
            page_size: 100,
        };
        assert!(pagination.validate().is_ok());
        assert_eq!(pagination.offset(), 429_496_729_400);
        assert_eq!(pagination.to_page().offset, 429_496_729_400);

        let ranking = Query::<GlobalRankingFilter>::try_from_uri(
            &"/rankings?page=4294967295&page_size=100".parse().unwrap(),
        )
        .unwrap();
        assert!(ranking.validate().is_ok());
        assert_eq!(ranking.to_db_filter().offset, 429_496_729_400);
    }

    #[test]
    fn parses_lenient_request_genders() {
        for (raw, expected) in [(" f ", Gender::F), ("m", Gender::M), (" mx ", Gender::Mx)] {
            let request: ComputeRisRequest = serde_json::from_value(json!({
                "bodyweight": "60", "total": "200", "gender": raw
            }))
            .unwrap();
            assert_eq!(request.gender, expected);
        }
        for gender in [json!("nonsense"), json!(null), json!(42)] {
            assert!(
                serde_json::from_value::<ComputeRisRequest>(json!({
                    "bodyweight": "60", "total": "200", "gender": gender
                }))
                .is_err()
            );
        }
        assert!(
            serde_json::from_value::<ComputeRisRequest>(json!({
                "bodyweight": "60", "total": "200"
            }))
            .is_err()
        );
    }

    #[test]
    fn parses_optional_query_enums() {
        let query = Query::<CompetitionListQuery>::try_from_uri(
            &"/competitions?status=%20COMPLETED%20".parse().unwrap(),
        )
        .unwrap();
        assert_eq!(query.status, Some(CompetitionStatus::Completed));
        let query = Query::<ClassesFilter>::try_from_uri(
            &"/rankings/classes?gender=%20f%20".parse().unwrap(),
        )
        .unwrap();
        assert_eq!(query.gender, Some(Gender::F));
        let mixed =
            Query::<ClassesFilter>::try_from_uri(&"/rankings/classes?gender=mx".parse().unwrap())
                .unwrap();
        assert_eq!(mixed.gender, Some(Gender::Mx));

        for value in [json!({}), json!({"status": null, "gender": null})] {
            assert!(
                serde_json::from_value::<CompetitionListQuery>(value.clone())
                    .unwrap()
                    .status
                    .is_none()
            );
            assert!(
                serde_json::from_value::<ClassesFilter>(value)
                    .unwrap()
                    .gender
                    .is_none()
            );
        }
        for raw in ["nonsense", ""] {
            assert!(
                Query::<CompetitionListQuery>::try_from_uri(
                    &format!("/competitions?status={raw}").parse().unwrap(),
                )
                .is_err()
            );
            assert!(
                Query::<ClassesFilter>::try_from_uri(
                    &format!("/rankings/classes?gender={raw}").parse().unwrap(),
                )
                .is_err()
            );
        }
    }

    #[test]
    fn requires_competition_when_gender_is_mixed() {
        for (raw, expected) in [
            ("%20m%20", RankedGender::M),
            ("f", RankedGender::F),
            ("mx", RankedGender::Mx),
        ] {
            let query = Query::<GlobalRankingFilter>::try_from_uri(
                &format!("/rankings?gender={raw}").parse().unwrap(),
            )
            .unwrap();
            assert_eq!(query.gender, Some(expected));
            assert_eq!(query.to_db_filter().gender, Some(Gender::from(expected)));
        }
        let mut mixed =
            Query::<GlobalRankingFilter>::try_from_uri(&"/rankings?gender=MX".parse().unwrap())
                .unwrap()
                .0;
        assert!(mixed.validate().is_err());
        mixed.competition_id = Some(uuid::Uuid::new_v4());
        assert!(mixed.validate().is_ok());
        for raw in ["nonsense", ""] {
            assert!(
                Query::<GlobalRankingFilter>::try_from_uri(
                    &format!("/rankings?gender={raw}").parse().unwrap(),
                )
                .is_err()
            );
        }
    }
}
