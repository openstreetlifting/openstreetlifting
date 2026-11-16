/// Wide enough for youth and masters entries; anything outside is a typo.
const RANGE: std::ops::RangeInclusive<u8> = 5..=100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Age(u8);

impl Age {
    pub fn parse(raw: &str) -> Result<Self, String> {
        let trimmed = raw.trim();
        let age = trimmed
            .parse::<u8>()
            .ok()
            .filter(|age| RANGE.contains(age))
            .ok_or_else(|| {
                format!(
                    "Age '{trimmed}' is not a whole number from {} to {}",
                    RANGE.start(),
                    RANGE.end()
                )
            })?;
        Ok(Self(age))
    }

    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl std::fmt::Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_plausible_ages() {
        assert_eq!(Age::parse(" 27 ").unwrap().get(), 27);
    }

    #[test]
    fn rejects_implausible_ages() {
        for raw in ["0", "4", "101", "150", "-3", "27.5", "twenty"] {
            assert!(Age::parse(raw).is_err(), "accepted {raw}");
        }
    }
}
