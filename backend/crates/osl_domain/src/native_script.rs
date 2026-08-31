//! Folding leaves a non-Latin name alone, so nobody typing `Radovan Repac`
//! finds `Радован Репац`. Transliteration is lossy and language specific, so
//! the Latin spelling is written by hand and the native one carried beside it.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeScript {
    Cyrillic,
    Greek,
    Han,
    Japanese,
    Korean,
}

impl NativeScript {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cyrillic => "cyrillic",
            Self::Greek => "greek",
            Self::Han => "han",
            Self::Japanese => "japanese",
            Self::Korean => "korean",
        }
    }

    /// A name written only in kanji is the same characters a Chinese name uses,
    /// so it reads as [`NativeScript::Han`] and only kana make it Japanese.
    ///
    /// ```
    /// use osl_domain::native_script::NativeScript;
    ///
    /// assert_eq!(NativeScript::detect("Радован Репац"), Some(NativeScript::Cyrillic));
    /// assert_eq!(NativeScript::detect("조정우"), Some(NativeScript::Korean));
    /// assert_eq!(NativeScript::detect("Mérandon"), None);
    /// ```
    pub fn detect(name: &str) -> Option<Self> {
        let mut found: Option<Self> = None;

        for c in name.chars() {
            let script = match c {
                '\u{0400}'..='\u{052F}' => Self::Cyrillic,
                '\u{0370}'..='\u{03FF}' | '\u{1F00}'..='\u{1FFF}' => Self::Greek,
                '\u{3040}'..='\u{30FF}' => Self::Japanese,
                '\u{1100}'..='\u{11FF}' | '\u{3130}'..='\u{318F}' | '\u{AC00}'..='\u{D7AF}' => {
                    Self::Korean
                }
                '\u{3400}'..='\u{4DBF}' | '\u{4E00}'..='\u{9FFF}' | '\u{F900}'..='\u{FAFF}' => {
                    Self::Han
                }
                _ => continue,
            };

            found = match (found, script) {
                (Some(Self::Japanese), Self::Han) | (Some(Self::Han), Self::Japanese) => {
                    Some(Self::Japanese)
                }
                (Some(seen), found) if seen != found => return None,
                _ => Some(script),
            };
        }

        found
    }

    pub fn is_latin(c: char) -> bool {
        c.is_ascii_alphabetic() || matches!(c, '\u{00C0}'..='\u{024F}' | '\u{1E00}'..='\u{1EFF}')
    }
}

impl fmt::Display for NativeScript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for NativeScript {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "cyrillic" => Ok(Self::Cyrillic),
            "greek" => Ok(Self::Greek),
            "han" => Ok(Self::Han),
            "japanese" => Ok(Self::Japanese),
            "korean" => Ok(Self::Korean),
            other => Err(format!(
                "'{other}' is not a script this database records. Use one of cyrillic, greek, \
                 han, japanese, korean"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_alphabet_is_recognised() {
        assert_eq!(
            NativeScript::detect("Радован Репац"),
            Some(NativeScript::Cyrillic)
        );
        assert_eq!(NativeScript::detect("Γιώργος"), Some(NativeScript::Greek));
        assert_eq!(NativeScript::detect("조정우"), Some(NativeScript::Korean));
        assert_eq!(NativeScript::detect("田中"), Some(NativeScript::Han));
    }

    #[test]
    fn kana_make_a_name_japanese() {
        assert_eq!(NativeScript::detect("たなか"), Some(NativeScript::Japanese));
        // Kanji and kana together are still one Japanese name.
        assert_eq!(
            NativeScript::detect("田中さくら"),
            Some(NativeScript::Japanese)
        );
    }

    #[test]
    fn a_latin_name_is_in_no_native_script() {
        assert_eq!(NativeScript::detect("Adrien Pelfresne"), None);
        assert_eq!(NativeScript::detect("Mérandon"), None);
        assert_eq!(NativeScript::detect("Bărbieru"), None);
        assert_eq!(NativeScript::detect(""), None);
    }

    #[test]
    fn two_unrelated_alphabets_are_not_one_name() {
        assert_eq!(NativeScript::detect("Радован 조정우"), None);
    }

    #[test]
    fn punctuation_belongs_to_no_script() {
        assert_eq!(
            NativeScript::detect("Радован-Репац"),
            Some(NativeScript::Cyrillic)
        );
    }

    #[test]
    fn accented_latin_is_still_latin() {
        for c in ['a', 'Z', 'é', 'ü', 'ő', 'ș', 'ā', 'ẞ'] {
            assert!(NativeScript::is_latin(c), "{c}");
        }
        for c in ['Р', 'γ', '조', '田'] {
            assert!(!NativeScript::is_latin(c), "{c}");
        }
    }

    #[test]
    fn names_round_trip_through_their_stored_form() {
        for script in [
            NativeScript::Cyrillic,
            NativeScript::Greek,
            NativeScript::Han,
            NativeScript::Japanese,
            NativeScript::Korean,
        ] {
            assert_eq!(script.as_str().parse::<NativeScript>().unwrap(), script);
        }
        assert!("latin".parse::<NativeScript>().is_err());
    }
}
