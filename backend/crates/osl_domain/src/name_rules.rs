//! spelling rules athlete name NEED to follow for it to be a validated entry.
use crate::native_script::NativeScript;
use unicode_normalization::UnicodeNormalization;

const PARTICLES: &[&str] = &[
    "bin", "da", "de", "do", "del", "den", "der", "des", "di", "dos", "du", "e", "el", "i", "in",
    "in't", "la", "le", "los", "op", "of", "'t", "te", "ten", "ter", "und", "v", "v.", "v.d.",
    "van", "von", "zur", "y", "zu",
];

const SUFFIXES: &[&str] = &["II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];

const ALLOWED_PUNCTUATION: [char; 4] = [' ', '\'', '.', '-'];

/// ```
/// use osl_domain::name_rules::check_name;
///
/// assert!(check_name("Anne-Sophie", "Gherardi").is_empty());
/// assert!(check_name("José", "d'Almeida").is_empty());
/// assert!(check_name("Andrea", "DeFrancesco").is_empty());
/// assert!(check_name("", "Darkhan").is_empty());
/// assert!(check_name("", "Redacted Athlete #3").is_empty());
/// assert!(!check_name("antoine", "auvray").is_empty());
/// ```
#[must_use]
pub fn check_name(first_name: &str, last_name: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let full = crate::display_name(first_name, last_name);

    if full.is_empty() {
        problems.push("has no name".to_string());
        return problems;
    }

    if crate::redaction::is_redacted(first_name, last_name) {
        return problems;
    }

    check_characters(&full, &mut problems);

    if let Some(script) = NativeScript::detect(&full) {
        problems.push(format!(
            "'{full}' is written in {script}. Put the Latin spelling here and '{full}' in \
             NativeName"
        ));
        return problems;
    }

    check_words(&full, &mut problems);

    if full.ends_with('.') {
        problems.push(format!(
            "'{full}' ends with a period. Suffixes and initials are written without one"
        ));
    }

    if full.starts_with("Jr ") || full.starts_with("Sr ") {
        problems.push(format!("'{full}' needs Jr/Sr moved to the end of the name"));
    }

    problems
}

fn check_characters(full: &str, problems: &mut Vec<String>) {
    let canonical: String = full.nfkc().collect();
    if canonical != full {
        problems.push(format!(
            "'{full}' is written with styled or full-width characters. Write it as '{canonical}'"
        ));
    }

    let illegal: Vec<char> = canonical
        .chars()
        .filter(|c| !c.is_alphabetic() && !ALLOWED_PUNCTUATION.contains(c))
        .collect();

    if !illegal.is_empty() {
        let shown: Vec<String> = illegal.iter().map(|c| format!("'{c}'")).collect();
        problems.push(format!(
            "'{full}' contains {}, which a name cannot hold. Only letters, spaces, hyphens, \
             apostrophes and periods are allowed",
            shown.join(", ")
        ));
    }
}

fn check_words(full: &str, problems: &mut Vec<String>) {
    for (index, word) in full.split(' ').enumerate() {
        if word.is_empty() {
            problems.push(format!("'{full}' has a doubled space"));
            continue;
        }

        if index != 0 && PARTICLES.contains(&word) {
            continue;
        }

        if word.starts_with('\'') {
            problems.push(format!(
                "'{full}' contains a nickname. Record the name the athlete competes under, \
                 not what they are called"
            ));
            continue;
        }

        if word.chars().all(|c| !c.is_alphanumeric()) {
            problems.push(format!(
                "'{full}' has punctuation standing on its own as a word"
            ));
            continue;
        }

        let stem = word.strip_prefix("d'").unwrap_or(word);

        if stem.split('-').any(|part| {
            part.chars()
                .next()
                .is_some_and(|c| c.is_alphabetic() && !c.is_uppercase())
        }) {
            problems.push(format!("'{full}' must have '{word}' capitalized"));
            continue;
        }

        check_capitals(full, word, stem, problems);
    }
}

fn check_capitals(full: &str, word: &str, stem: &str, problems: &mut Vec<String>) {
    if word.contains('.') {
        return;
    }

    for part in stem.split('-') {
        let letters: Vec<char> = part.chars().filter(|c| c.is_alphabetic()).collect();

        if letters.len() > 1
            && letters.iter().all(|c| c.is_uppercase())
            && !SUFFIXES.contains(&part)
        {
            problems.push(format!(
                "'{full}' has '{word}' in capitals. Write it the way it is read"
            ));
            return;
        }
    }
}

/// ```
/// use osl_domain::name_rules::check_native_name;
///
/// assert!(check_native_name("Радован Репац").is_empty());
/// assert!(check_native_name("").is_empty());
/// assert!(!check_native_name("Radovan Repac").is_empty());
/// ```
pub fn check_native_name(native_name: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let native_name = native_name.trim();

    if native_name.is_empty() {
        return problems;
    }

    if native_name.chars().any(NativeScript::is_latin) {
        problems.push(format!(
            "native name '{native_name}' has Latin letters in it. It holds the name in its own \
             alphabet, and the Latin spelling belongs in FirstName and LastName"
        ));
    }

    if NativeScript::detect(native_name).is_none() {
        problems.push(format!(
            "native name '{native_name}' is not written in one alphabet this database records. \
             Use Cyrillic, Greek, Han, Japanese or Korean"
        ));
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problems(first: &str, last: &str) -> String {
        check_name(first, last).join(" | ")
    }

    #[test]
    fn a_plainly_written_name_passes() {
        for (first, last) in [
            ("Adrien", "Pelfresne"),
            ("Anne-Sophie", "Gherardi"),
            ("Nicolas", "Routier-Scappucci"),
            ("Léa", "Mérandon"),
            ("Marc", "Yeurc'h"),
            ("Olivier", "N'Guessan"),
            ("Simone", "De Simone"),
            ("Thomas", "Spigner IV"),
            ("Émilie", "L"),
        ] {
            assert!(
                check_name(first, last).is_empty(),
                "{first} {last}: {}",
                problems(first, last)
            );
        }
    }

    #[test]
    fn capitals_a_machine_cannot_restore_are_refused() {
        assert!(problems("Alexis", "GITTON").contains("in capitals"));
        assert!(problems("Camille", "BOUVOT GINHOUX").contains("in capitals"));
        assert!(problems("José", "D'ALMEIDA").contains("in capitals"));
    }

    #[test]
    fn intercaps_are_a_spelling_not_a_mistake() {
        assert_eq!(check_name("Andrea", "DeFrancesco"), [] as [String; 0]);
        assert_eq!(check_name("Sean", "McDonald"), [] as [String; 0]);
        assert_eq!(check_name("Larodrick", "LaRue"), [] as [String; 0]);
    }

    #[test]
    fn every_word_starts_capitalized() {
        assert!(problems("antoine", "auvray").contains("'antoine' capitalized"));
        assert!(problems("Mateo", "Moure losada").contains("'losada' capitalized"));
    }

    #[test]
    fn both_halves_of_a_hyphenated_name_are_capitalized() {
        assert!(problems("Jean-rodrigue", "Perle").contains("'Jean-rodrigue' capitalized"));
        assert!(problems("Lea", "Goutte-toquet").contains("'Goutte-toquet' capitalized"));
        assert_eq!(check_name("Jean-Rodrigue", "Perle"), [] as [String; 0]);
    }

    #[test]
    fn particles_keep_their_place_but_not_the_front() {
        assert_eq!(check_name("Martina", "de Iturbe"), [] as [String; 0]);
        assert_eq!(check_name("Franck", "da Silva"), [] as [String; 0]);
        assert_eq!(check_name("Kevin", "van der Berg"), [] as [String; 0]);
        assert!(problems("de", "Iturbe").contains("'de' capitalized"));
    }

    #[test]
    fn a_french_particle_hides_the_capital_behind_it() {
        assert_eq!(check_name("José", "d'Almeida"), [] as [String; 0]);
        assert!(problems("José", "d'almeida").contains("'d'almeida' capitalized"));
    }

    #[test]
    fn styled_letters_are_rewritten_to_plain_ones() {
        let flagged = problems("𝐉𝐨𝐡𝐧", "Smith");
        assert!(flagged.contains("styled or full-width"), "{flagged}");
        assert!(flagged.contains("John Smith"), "{flagged}");
        assert!(problems("Ｊｏｈｎ", "Smith").contains("styled or full-width"));
    }

    #[test]
    fn formatting_that_is_not_a_name_is_refused() {
        assert!(problems("", "uki_citywalker").contains("'_'"));
        assert!(problems("Kevin", "Smith 💪").contains("'💪'"));
        assert!(problems("Kevin", "Smith (SWE)").contains("'('"));
        assert!(problems("Kevin", "Smith2").contains("'2'"));
        assert!(problems("Kevin", "Smith’s").contains("'’'"));
    }

    #[test]
    fn nicknames_are_not_part_of_the_name() {
        assert!(problems("Loan", "'Seraf' Bernard-Bodier").contains("nickname"));
    }

    #[test]
    fn a_name_in_another_alphabet_belongs_in_the_native_column() {
        let flagged = problems("", "Радован Репац");
        assert!(flagged.contains("written in cyrillic"), "{flagged}");
        assert!(flagged.contains("NativeName"), "{flagged}");
        assert!(problems("", "조정우").contains("written in korean"));
        assert_eq!(check_name("Alexie", "Bărbieru"), [] as [String; 0]);
    }

    #[test]
    fn the_native_name_holds_only_its_own_alphabet() {
        assert_eq!(check_native_name("Радован Репац"), [] as [String; 0]);
        assert_eq!(check_native_name("조정우"), [] as [String; 0]);
        assert_eq!(check_native_name(""), [] as [String; 0]);

        assert!(
            check_native_name("Radovan Repac")
                .join(" ")
                .contains("Latin letters")
        );
        assert!(
            check_native_name("Радован 조정우")
                .join(" ")
                .contains("not written in one alphabet")
        );
    }

    #[test]
    fn initials_keep_their_capitals() {
        assert_eq!(check_name("S.E.M", "Visser"), [] as [String; 0]);
        assert_eq!(check_name("Émilie", "L"), [] as [String; 0]);
    }

    #[test]
    fn suffixes_are_written_without_a_period() {
        assert!(problems("Thomas", "Spigner Jr.").contains("ends with a period"));
        assert!(problems("Jr", "Thomas Spigner").contains("moved to the end"));
    }

    #[test]
    fn an_athlete_with_one_name_is_checked_on_that_name() {
        assert_eq!(check_name("", "Darkhan"), [] as [String; 0]);
        assert!(problems("", "svon").contains("'svon' capitalized"));
        assert!(problems("", "").contains("no name"));
    }

    #[test]
    fn a_redacted_athlete_is_allowed_its_number() {
        assert_eq!(check_name("", "Redacted Athlete #1"), [] as [String; 0]);
        assert_eq!(check_name("", "Redacted Athlete #9001"), [] as [String; 0]);
    }

    #[test]
    fn the_exemption_reaches_no_further_than_the_stand_in() {
        assert!(problems("", "Redacted Athlete #07").contains("'0'"));
        assert!(problems("", "Redacted Athlete 7").contains("'7'"));
        assert!(problems("Kevin", "Smith2").contains("'2'"));
        assert!(problems("Lifter", "2").contains("'2'"));
        assert!(problems("", "Baki_HD").contains("'_'"));
    }
}
