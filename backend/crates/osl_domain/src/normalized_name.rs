/// Athlete name normalized to a single canonical form.
/// Sources spell the same athlete differently ("ADRIEN PELFRESNE", "Adrien Pelfresne").
/// This can create duplicates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedAthleteName {
    first_name: String,
    last_name: String,
}

impl NormalizedAthleteName {
    /// # Examples
    ///
    /// ```
    /// use osl_domain::normalized_name::NormalizedAthleteName;
    ///
    /// let name1 = NormalizedAthleteName::new("adrien", "pelfresne");
    /// let name2 = NormalizedAthleteName::new("ADRIEN", "PELFRESNE");
    /// let name3 = NormalizedAthleteName::new("Adrien", "Pelfresne");
    ///
    /// assert_eq!(name1.database_first_name(), "Adrien");
    /// assert_eq!(name1.database_last_name(), "Pelfresne");
    /// assert_eq!(name1, name2);
    /// assert_eq!(name2, name3);
    /// ```
    pub fn new(first_name: impl Into<String>, last_name: impl Into<String>) -> Self {
        let first_name = normalize_name_part(first_name.into());
        let last_name = normalize_name_part(last_name.into());

        Self {
            first_name,
            last_name,
        }
    }

    pub fn database_first_name(&self) -> &str {
        &self.first_name
    }

    pub fn database_last_name(&self) -> &str {
        &self.last_name
    }

    pub fn as_database_tuple(&self) -> (&str, &str) {
        (&self.first_name, &self.last_name)
    }
}

fn normalize_name_part(name: String) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let mut chars = trimmed.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization_preserves_order() {
        let name = NormalizedAthleteName::new("Adrien", "Pelfresne");
        assert_eq!(name.database_first_name(), "Adrien");
        assert_eq!(name.database_last_name(), "Pelfresne");
    }

    #[test]
    fn test_normalization_title_case() {
        let name1 = NormalizedAthleteName::new("adrien", "pelfresne");
        let name2 = NormalizedAthleteName::new("ADRIEN", "PELFRESNE");
        let name3 = NormalizedAthleteName::new("Adrien", "Pelfresne");

        assert_eq!(name1.database_first_name(), "Adrien");
        assert_eq!(name1.database_last_name(), "Pelfresne");
        assert_eq!(name1, name2);
        assert_eq!(name2, name3);
    }

    #[test]
    fn test_normalization_trims_whitespace() {
        let name = NormalizedAthleteName::new("  Adrien  ", "  Pelfresne  ");
        assert_eq!(name.database_first_name(), "Adrien");
        assert_eq!(name.database_last_name(), "Pelfresne");
    }

    #[test]
    fn test_different_names_not_equal() {
        let name1 = NormalizedAthleteName::new("Adrien", "Pelfresne");
        let name2 = NormalizedAthleteName::new("Adrienne", "Pelfresne");
        assert_ne!(name1, name2);
    }
}
