use serde::{
    Deserialize, Deserializer,
    de::{DeserializeOwned, IntoDeserializer, value::Error as ValueError},
};

pub async fn load_if<T>(
    needed: bool,
    load: impl Future<Output = osl_db::error::Result<T>>,
) -> osl_db::error::Result<Option<T>> {
    if needed {
        load.await.map(Some)
    } else {
        Ok(None)
    }
}

/// Comma-separated `?include=` list, e.g. `?include=competitions,records`.
#[derive(Debug, Default, Clone)]
pub struct Include(Vec<String>);

impl Include {
    fn parse(raw: &str) -> Self {
        Self(
            raw.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect(),
        )
    }

    #[must_use]
    pub fn has(&self, name: &str) -> bool {
        self.0.iter().any(|v| v == name)
    }

    pub fn parse_as<T: DeserializeOwned>(&self) -> Result<Vec<T>, String> {
        self.0
            .iter()
            .map(|value| {
                T::deserialize(value.as_str().into_deserializer())
                    .map_err(|error: ValueError| format!("invalid include '{value}': {error}"))
            })
            .collect()
    }

    pub fn validate(&self, allowed: &[&str]) -> Result<(), String> {
        for value in &self.0 {
            if !allowed.contains(&value.as_str()) {
                return Err(format!(
                    "unknown include '{}', expected one of: {}",
                    value,
                    allowed.join(", ")
                ));
            }
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Include {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Option::<String>::deserialize(deserializer)?;
        Ok(Self::parse(&raw.unwrap_or_default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum Section {
        Competitions,
        LiftBalance,
    }

    #[tokio::test]
    async fn skips_a_section_that_is_not_needed() {
        let loaded = load_if::<()>(false, async { panic!("must not be polled") }).await;
        assert!(matches!(loaded, Ok(None)));
    }

    #[tokio::test]
    async fn loads_a_needed_section() {
        assert!(matches!(load_if(true, async { Ok(1) }).await, Ok(Some(1))));
    }

    #[test]
    fn parses_typed_values() {
        let include = Include::parse("competitions, lift_balance");
        assert_eq!(
            include.parse_as::<Section>().unwrap(),
            vec![Section::Competitions, Section::LiftBalance]
        );
    }

    #[test]
    fn rejects_unknown_typed_values_and_names_the_allowed_ones() {
        let err = Include::parse("competitions,bogus")
            .parse_as::<Section>()
            .unwrap_err();
        assert!(err.contains("bogus"), "{err}");
        assert!(err.contains("lift_balance"), "{err}");
    }

    #[test]
    fn returns_empty_includes_when_absent() {
        let include = Include::parse("");
        assert!(!include.has("competitions"));
        assert!(include.validate(&["competitions"]).is_ok());
    }

    #[test]
    fn splits_and_trims_values() {
        let include = Include::parse("competitions, records");
        assert!(include.has("competitions"));
        assert!(include.has("records"));
    }

    #[test]
    fn rejects_unknown_values() {
        let include = Include::parse("competitions,bogus");
        let err = include.validate(&["competitions", "records"]).unwrap_err();
        assert!(err.contains("bogus"), "{err}");
        assert!(err.contains("competitions, records"), "{err}");
    }

    #[test]
    fn ignores_empty_segments() {
        let include = Include::parse(",,competitions,");
        assert!(include.has("competitions"));
        assert!(include.validate(&["competitions"]).is_ok());
    }
}
