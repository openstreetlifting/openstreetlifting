use unicode_normalization::UnicodeNormalization;

/// ```
/// use osl_domain::slugify;
///
/// assert_eq!(slugify("FNSL"), "fnsl");
/// assert_eq!(slugify("Fédération Française"), "federation-francaise");
/// assert_eq!(slugify("Street  Lifting / UK"), "street-lifting-uk");
/// ```
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
    fn accents_and_case_are_folded() {
        assert_eq!(slugify("Fédération"), "federation");
        assert_eq!(slugify("FNSL"), "fnsl");
    }

    #[test]
    fn runs_of_punctuation_collapse_to_one_hyphen() {
        assert_eq!(slugify("Street -- Lifting"), "street-lifting");
        assert_eq!(slugify("  FNSL  "), "fnsl");
    }

    #[test]
    fn a_name_with_nothing_to_slug_is_empty() {
        assert_eq!(slugify("---"), "");
    }
}
