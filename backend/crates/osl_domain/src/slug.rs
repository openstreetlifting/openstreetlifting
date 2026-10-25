use unicode_normalization::UnicodeNormalization;

/// ```
/// use osl_domain::slugify;
///
/// assert_eq!(slugify("FNSL"), "fnsl");
/// assert_eq!(slugify("Fédération Française"), "federation-francaise");
/// assert_eq!(slugify("Street  Lifting / UK"), "street-lifting-uk");
/// ```
#[must_use]
pub fn slugify(text: &str) -> String {
    let without_accents: String = text
        .nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect();

    without_accents
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_accents_and_case() {
        assert_eq!(slugify("Fédération"), "federation");
        assert_eq!(slugify("FNSL"), "fnsl");
    }

    #[test]
    fn collapses_punctuation_to_hyphens() {
        assert_eq!(slugify("Street -- Lifting"), "street-lifting");
        assert_eq!(slugify("  FNSL  "), "fnsl");
    }

    #[test]
    fn returns_empty_slug_for_punctuation() {
        assert_eq!(slugify("---"), "");
    }
}
