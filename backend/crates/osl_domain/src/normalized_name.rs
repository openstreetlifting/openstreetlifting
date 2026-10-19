use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedAthleteName {
    first_name: String,
    last_name: String,
}

impl NormalizedAthleteName {
    /// ```
    /// use osl_domain::normalized_name::NormalizedAthleteName;
    ///
    /// let name = NormalizedAthleteName::new("  Anne-Sophie  ", "Gherardi");
    ///
    /// assert_eq!(name.database_first_name(), "Anne-Sophie");
    /// assert_eq!(name.database_last_name(), "Gherardi");
    /// ```
    pub fn new(first_name: impl Into<String>, last_name: impl Into<String>) -> Self {
        let first_name = normalize_name_part(&first_name.into());
        let last_name = normalize_name_part(&last_name.into());

        Self {
            first_name,
            last_name,
        }
    }

    #[must_use]
    pub fn database_first_name(&self) -> &str {
        &self.first_name
    }

    #[must_use]
    pub fn database_last_name(&self) -> &str {
        &self.last_name
    }

    #[must_use]
    pub fn as_database_tuple(&self) -> (&str, &str) {
        (&self.first_name, &self.last_name)
    }

    /// ```
    /// use osl_domain::normalized_name::NormalizedAthleteName;
    ///
    /// let plain = NormalizedAthleteName::new("Lea", "MERANDON");
    /// let accented = NormalizedAthleteName::new("Léa", "Mérandon");
    ///
    /// assert_eq!(plain.match_name(), accented.match_name());
    /// assert_eq!(accented.database_last_name(), "Mérandon");
    /// ```
    #[must_use]
    pub fn match_name(&self) -> String {
        format!("{} {}", fold(&self.first_name), fold(&self.last_name))
            .trim()
            .to_string()
    }
}

/// ```
/// use osl_domain::normalized_name::display_name;
///
/// assert_eq!(display_name("Adrien", "Pelfresne"), "Adrien Pelfresne");
/// assert_eq!(display_name("", "Darkhan"), "Darkhan");
/// ```
#[must_use]
pub fn display_name(first_name: &str, last_name: &str) -> String {
    format!("{first_name} {last_name}").trim().to_string()
}

fn fold(name: &str) -> String {
    let without_accents: String = name
        .nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect();

    let spaced: String = without_accents
        .to_lowercase()
        .chars()
        .filter(|c| *c != '\'' && *c != '\u{2019}')
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect();

    spaced.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize_name_part(name: &str) -> String {
    name.trim().to_string()
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
    fn the_file_decides_how_a_name_is_capitalised() {
        for spelling in ["Anne-Sophie", "D'Almeida", "DeFrancesco", "McDonald"] {
            assert_eq!(
                NormalizedAthleteName::new(spelling, "X").database_first_name(),
                spelling
            );
        }
    }

    #[test]
    fn capitalisation_still_does_not_split_a_person() {
        let shouted = NormalizedAthleteName::new("ADRIEN", "PELFRESNE");
        let plain = NormalizedAthleteName::new("Adrien", "Pelfresne");
        assert_eq!(shouted.match_name(), plain.match_name());
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

    fn match_name(first: &str, last: &str) -> String {
        NormalizedAthleteName::new(first, last).match_name()
    }

    #[test]
    fn accents_do_not_split_a_person() {
        assert_eq!(match_name("Lea", "MERANDON"), match_name("Léa", "Mérandon"));
        assert_eq!(match_name("Joao", "Silva"), match_name("João", "Sílva"));
        assert_eq!(match_name("Muller", "X"), match_name("Müller", "X"));
    }

    #[test]
    fn the_displayed_name_keeps_its_accents() {
        let name = NormalizedAthleteName::new("Léa", "Mérandon");
        assert_eq!(name.database_first_name(), "Léa");
        assert_eq!(name.database_last_name(), "Mérandon");
        assert_eq!(name.match_name(), "lea merandon");
    }

    #[test]
    fn punctuation_does_not_split_a_person() {
        assert_eq!(
            match_name("Jean-Luc", "Picard"),
            match_name("Jean Luc", "Picard")
        );
        assert_eq!(match_name("Sean", "O'Brien"), match_name("Sean", "OBrien"));
        assert_eq!(match_name("Sean", "O’Brien"), match_name("Sean", "OBrien"));
        assert_eq!(match_name("Anna", "Smith,"), match_name("Anna", "Smith"));
        assert_eq!(
            match_name("Anna  Marie", "Smith"),
            match_name("Anna Marie", "Smith")
        );
    }

    #[test]
    fn different_people_still_differ() {
        assert_ne!(
            match_name("Adrien", "Pelfresne"),
            match_name("Adrienne", "Pelfresne")
        );
        assert_ne!(match_name("Tom", "Berthier"), match_name("Tom", "Bertier"));
    }

    #[test]
    fn non_latin_names_survive_folding() {
        assert_eq!(match_name("Иван", "Петров"), "иван петров");
        assert_eq!(match_name("大輔", "田中"), "大輔 田中");
    }
}
