use serde::{Deserialize, Deserializer};
use utoipa::{
    PartialSchema, ToSchema,
    openapi::{RefOr, Schema, Type, schema::SchemaType},
};

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

impl PartialSchema for Include {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(SchemaType::Type(Type::String))
                .description(Some("Comma-separated list of sections to embed"))
                .build(),
        ))
    }
}

impl ToSchema for Include {}

impl<'de> Deserialize<'de> for Include {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Option::<String>::deserialize(deserializer)?;
        Ok(Self::parse(&raw.unwrap_or_default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
