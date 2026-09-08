use serde::{Deserialize, Serialize};

/// One of the four lifts. The spellings are the `movements.name` primary key
/// every `movement_name` column points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Movement {
    #[serde(rename = "Muscle-up")]
    MuscleUp,
    #[serde(rename = "Pull-up")]
    PullUp,
    Dips,
    Squat,
}

impl Movement {
    pub const ALL: [Movement; 4] = [
        Movement::MuscleUp,
        Movement::PullUp,
        Movement::Dips,
        Movement::Squat,
    ];

    pub fn code(&self) -> char {
        match self {
            Self::MuscleUp => 'M',
            Self::PullUp => 'P',
            Self::Dips => 'D',
            Self::Squat => 'S',
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MuscleUp => "Muscle-up",
            Self::PullUp => "Pull-up",
            Self::Dips => "Dips",
            Self::Squat => "Squat",
        }
    }

    pub fn display_order(&self) -> i16 {
        match self {
            Self::MuscleUp => 1,
            Self::PullUp => 2,
            Self::Dips => 3,
            Self::Squat => 4,
        }
    }

    pub fn column_prefix(&self) -> &'static str {
        match self {
            Self::MuscleUp => "MuscleUp",
            Self::PullUp => "PullUp",
            Self::Dips => "Dips",
            Self::Squat => "Squat",
        }
    }

    pub fn from_code(code: char) -> Option<Self> {
        Self::ALL.into_iter().find(|m| m.code() == code)
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|m| m.as_str() == name)
    }
}

impl std::str::FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s)
            .ok_or_else(|| format!("unknown movement: {s}, expected one of MPDS by name"))
    }
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_and_names_round_trip() {
        for movement in Movement::ALL {
            assert_eq!(Movement::from_code(movement.code()), Some(movement));
            assert_eq!(Movement::from_name(movement.as_str()), Some(movement));
        }
    }

    #[test]
    fn all_is_in_display_order() {
        let orders: Vec<_> = Movement::ALL.iter().map(|m| m.display_order()).collect();
        assert_eq!(orders, vec![1, 2, 3, 4]);
    }

    #[test]
    fn serde_spells_them_the_way_the_database_does() {
        for movement in Movement::ALL {
            assert_eq!(
                serde_json::to_string(&movement).unwrap(),
                format!("\"{}\"", movement.as_str())
            );
        }
    }

    #[test]
    fn nothing_shares_a_letter_or_a_column() {
        for (index, movement) in Movement::ALL.iter().enumerate() {
            for other in &Movement::ALL[index + 1..] {
                assert_ne!(movement.code(), other.code());
                assert_ne!(movement.column_prefix(), other.column_prefix());
            }
        }
    }
}
