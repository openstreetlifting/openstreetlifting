use crate::movement::Movement;

const TOTAL: &str = "Total";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordLift {
    Movement(Movement),
    Total,
}

impl RecordLift {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Movement(movement) => movement.as_str(),
            Self::Total => TOTAL,
        }
    }
}

impl From<Movement> for RecordLift {
    fn from(movement: Movement) -> Self {
        Self::Movement(movement)
    }
}

impl std::str::FromStr for RecordLift {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == TOTAL {
            return Ok(Self::Total);
        }
        s.parse().map(Self::Movement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_lift_round_trips_through_its_name() {
        let lifts = Movement::ALL
            .into_iter()
            .map(RecordLift::from)
            .chain([RecordLift::Total]);
        for lift in lifts {
            assert_eq!(lift.as_str().parse::<RecordLift>(), Ok(lift));
        }
    }

    #[test]
    fn rejects_an_unknown_name() {
        assert!("Bench".parse::<RecordLift>().is_err());
    }
}
