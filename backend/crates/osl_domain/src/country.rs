use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// ISO 3166-1 alpha-2 country code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CountryCode([u8; 2]);

impl CountryCode {
    pub fn parse(raw: &str) -> Result<Self, String> {
        let trimmed = raw.trim();
        let bytes = trimmed.as_bytes();

        if bytes.len() != 2 || !bytes.iter().all(|b| b.is_ascii_alphabetic()) {
            return Err(format!(
                "'{}' is not an ISO 3166-1 alpha-2 country code, expected two letters like FR",
                trimmed
            ));
        }

        Ok(Self([
            bytes[0].to_ascii_uppercase(),
            bytes[1].to_ascii_uppercase(),
        ]))
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("country code is ascii")
    }
}

impl std::fmt::Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for CountryCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Serialize for CountryCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CountryCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::parse(&raw).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercases_and_trims() {
        assert_eq!(CountryCode::parse("  fr ").unwrap().as_str(), "FR");
    }

    #[test]
    fn rejects_country_names_and_alpha3() {
        for raw in ["France", "FRA", "F", ""] {
            assert!(CountryCode::parse(raw).is_err(), "accepted {raw}");
        }
    }

    #[test]
    fn same_country_written_two_ways_compares_equal() {
        assert_eq!(
            CountryCode::parse("fr").unwrap(),
            CountryCode::parse("FR").unwrap()
        );
    }
}
