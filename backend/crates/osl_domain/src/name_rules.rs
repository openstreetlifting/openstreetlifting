//! The spelling rules an athlete name has to follow to enter the database.
//!
//! Names arrive from federation spreadsheets, registration forms and screenshots,
//! each with their own idea of formatting: full capitals, styled Unicode fonts
//! pasted out of Instagram, emoji, nicknames in quotes. The importer used to
//! rewrite what it received, which quietly mangled every name a machine cannot
//! case correctly (`Anne-Sophie`, `D'Almeida`, `DeFrancesco`).
//!
//! So nothing is rewritten here. The file holds the name as it should be read,
//! and a name that breaks these rules is reported and fixed by hand.
//!
//! The rules follow OpenPowerlifting's checker, the largest curated database of
//! international lifter names, so a name spelled for one reads correctly in the
//! other.

use crate::native_script::NativeScript;
use unicode_normalization::UnicodeNormalization;

/// Words that stay lowercase inside a name, mostly articles and patronymic
/// particles. `van`, `de` and `von` are part of the surname, not the start of
/// it, so they only keep their case away from the front of the name.
const PARTICLES: &[&str] = &[
    "bin", "da", "de", "do", "del", "den", "der", "des", "di", "dos", "du", "e", "el", "i", "in",
    "in't", "la", "le", "los", "op", "of", "'t", "te", "ten", "ter", "und", "v", "v.", "v.d.",
    "van", "von", "zur", "y", "zu",
];

/// Generational suffixes, which are read as letters rather than as a word and
/// so are the one place capitals are correct.
const SUFFIXES: &[&str] = &["II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];

/// Punctuation a name may contain. Everything else — digits, underscores,
/// brackets, emoji, currency and the rest — is formatting that came along with
/// the name rather than part of it.
const ALLOWED_PUNCTUATION: [char; 4] = [' ', '\'', '.', '-'];

/// Every way `first_name last_name` fails the spelling rules, worded so the fix
/// is obvious to whoever opens the file.
///
/// ```
/// use osl_domain::name_rules::check_name;
///
/// assert!(check_name("Anne-Sophie", "Gherardi").is_empty());
/// assert!(check_name("José", "d'Almeida").is_empty());
/// assert!(check_name("Andrea", "DeFrancesco").is_empty());
/// assert!(check_name("", "Darkhan").is_empty());
/// assert!(!check_name("antoine", "auvray").is_empty());
/// ```
pub fn check_name(first_name: &str, last_name: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let full = crate::display_name(first_name, last_name);

    if full.is_empty() {
        problems.push("has no name".to_string());
        return problems;
    }

    check_characters(&full, &mut problems);

    // Identity, search and the URL are built on the Latin spelling.
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

/// Rejects anything that is not a letter or name punctuation, and any letter
/// that is a font variant of one.
///
/// Styled Unicode alphabets are the trap here: a name pasted from a social
/// profile can be mathematical bold or fullwidth, which reads as letters to a
/// human and to `is_alphabetic`, but sorts and matches as a different string
/// entirely. NFKC folds those variants onto the plain letters they imitate, so
/// a name that survives it unchanged is already written in plain characters.
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

        // A French name carries its particle in front of the surname it belongs
        // to, so the capital to look at is the one after the apostrophe.
        let stem = word.strip_prefix("d'").unwrap_or(word);

        // Either side of a hyphen is a name in its own right and carries its own
        // capital, so `Jean-Rodrigue` is two names joined rather than one word.
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

/// A name shouted in capitals is how a spreadsheet was formatted, not how the
/// name is read, and it is the one casing a machine cannot restore: `MCDONALD`
/// could be `McDonald` or `Macdonald`. So it is refused rather than guessed at.
fn check_capitals(full: &str, word: &str, stem: &str, problems: &mut Vec<String>) {
    // Initials are read letter by letter, so their capitals are the spelling
    // rather than a shout: `S.E.M Visser` is written exactly that way.
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
        assert!(check_name("Andrea", "DeFrancesco").is_empty());
        assert!(check_name("Sean", "McDonald").is_empty());
        assert!(check_name("Larodrick", "LaRue").is_empty());
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
        assert!(check_name("Jean-Rodrigue", "Perle").is_empty());
    }

    #[test]
    fn particles_keep_their_place_but_not_the_front() {
        assert!(check_name("Martina", "de Iturbe").is_empty());
        assert!(check_name("Franck", "da Silva").is_empty());
        assert!(check_name("Kevin", "van der Berg").is_empty());
        // The same word opening the name is a first name that lost its capital.
        assert!(problems("de", "Iturbe").contains("'de' capitalized"));
    }

    #[test]
    fn a_french_particle_hides_the_capital_behind_it() {
        assert!(check_name("José", "d'Almeida").is_empty());
        assert!(problems("José", "d'almeida").contains("'d'almeida' capitalized"));
    }

    #[test]
    fn styled_letters_are_rewritten_to_plain_ones() {
        // Mathematical bold reads as a name but matches as a different string.
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
        // Latin with accents is not another alphabet.
        assert!(check_name("Alexie", "Bărbieru").is_empty());
    }

    #[test]
    fn the_native_name_holds_only_its_own_alphabet() {
        assert!(check_native_name("Радован Репац").is_empty());
        assert!(check_native_name("조정우").is_empty());
        assert!(check_native_name("").is_empty());

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
        assert!(check_name("S.E.M", "Visser").is_empty());
        assert!(check_name("Émilie", "L").is_empty());
    }

    #[test]
    fn suffixes_are_written_without_a_period() {
        assert!(problems("Thomas", "Spigner Jr.").contains("ends with a period"));
        assert!(problems("Jr", "Thomas Spigner").contains("moved to the end"));
    }

    #[test]
    fn an_athlete_with_one_name_is_checked_on_that_name() {
        assert!(check_name("", "Darkhan").is_empty());
        assert!(problems("", "svon").contains("'svon' capitalized"));
        assert!(problems("", "").contains("no name"));
    }
}
