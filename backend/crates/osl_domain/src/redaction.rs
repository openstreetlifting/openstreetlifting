//! Numbered replacement names for athletes who request name removal.
//! The same number identifies their results across competitions.

use std::fmt;

use crate::normalized_name::display_name;

const PREFIX: &str = "Redacted Athlete";

/// ```
/// use osl_domain::redaction::RedactedAthlete;
///
/// let athlete = RedactedAthlete::new(7).unwrap();
///
/// assert_eq!(athlete.to_string(), "Redacted Athlete #7");
/// assert_eq!(athlete.id(), 7);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RedactedAthlete(u32);

impl RedactedAthlete {
    pub fn new(id: u32) -> Option<Self> {
        (id > 0).then_some(Self(id))
    }

    pub fn from_match_key(key: &str) -> Option<Self> {
        let number = key.strip_prefix("redacted athlete ")?;
        let redacted = Self::new(number.parse().ok()?)?;
        (number == redacted.id().to_string()).then_some(redacted)
    }

    pub fn id(self) -> u32 {
        self.0
    }

    pub fn last_name(self) -> String {
        self.to_string()
    }

    /// ```
    /// use osl_domain::redaction::RedactedAthlete;
    ///
    /// assert_eq!(RedactedAthlete::parse("Redacted Athlete #7").unwrap().id(), 7);
    /// assert!(RedactedAthlete::parse("Redacted Athlete #07").is_none());
    /// assert!(RedactedAthlete::parse("Adrien Pelfresne").is_none());
    /// ```
    pub fn parse(name: &str) -> Option<Self> {
        let number = name
            .trim()
            .strip_prefix(PREFIX)?
            .trim_start()
            .strip_prefix('#')?;

        if number.len() > 1 && number.starts_with('0') {
            return None;
        }

        if !number.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        Self::new(number.parse().ok()?)
    }
}

impl fmt::Display for RedactedAthlete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{PREFIX} #{}", self.0)
    }
}

/// ```
/// use osl_domain::redaction::is_redacted;
///
/// assert!(is_redacted("", "Redacted Athlete #2"));
/// assert!(is_redacted("Redacted", "Athlete #2"));
/// assert!(!is_redacted("Adrien", "Pelfresne"));
/// ```
pub fn is_redacted(first_name: &str, last_name: &str) -> bool {
    RedactedAthlete::parse(&display_name(first_name, last_name)).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normalized_name::NormalizedAthleteName;
    use crate::slugify;

    #[test]
    fn a_stand_in_survives_being_written_and_read_back() {
        for id in [1, 2, 9, 10, 143, 9001, 99_999] {
            let athlete = RedactedAthlete::new(id).unwrap();
            assert_eq!(RedactedAthlete::parse(&athlete.to_string()), Some(athlete));
        }
    }

    #[test]
    fn the_numbering_starts_at_one() {
        assert!(RedactedAthlete::new(0).is_none());
        assert!(RedactedAthlete::parse("Redacted Athlete #0").is_none());
    }

    #[test]
    fn only_one_spelling_is_a_stand_in() {
        for spelling in [
            "Redacted Athlete #07",
            "Redacted Athlete #+7",
            "Redacted Athlete #-7",
            "Redacted Athlete #7a",
            "Redacted Athlete #7.5",
            "Redacted Athlete 7",
            "Redacted athlete #7",
            "Redacted Athlete",
            "#7",
            "",
        ] {
            assert!(
                RedactedAthlete::parse(spelling).is_none(),
                "'{spelling}' was read as a stand-in"
            );
        }
    }

    #[test]
    fn a_real_name_is_never_a_stand_in() {
        for (first, last) in [
            ("Adrien", "Pelfresne"),
            ("Anne-Sophie", "Gherardi"),
            ("", "Darkhan"),
            ("Redact", "Athletes"),
        ] {
            assert!(!is_redacted(first, last), "{first} {last}");
        }
    }

    #[test]
    fn the_number_reaches_the_url() {
        let athlete = RedactedAthlete::new(7).unwrap();
        let name = NormalizedAthleteName::new("", athlete.last_name());

        assert_eq!(name.match_name(), "redacted athlete 7");
        assert_eq!(slugify(&athlete.to_string()), "redacted-athlete-7");
    }

    #[test]
    fn two_stand_ins_are_two_athletes() {
        let first = NormalizedAthleteName::new("", RedactedAthlete::new(1).unwrap().last_name());
        let second = NormalizedAthleteName::new("", RedactedAthlete::new(2).unwrap().last_name());

        assert_ne!(first.match_name(), second.match_name());
    }
}
