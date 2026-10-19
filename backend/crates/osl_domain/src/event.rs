use crate::movement::Movement;

pub const FULL_EVENT: &str = "MPDS";

#[must_use]
pub fn is_full_event(event_code: Option<&str>) -> bool {
    event_code == Some(FULL_EVENT)
}

pub fn movements(event_code: &str) -> Result<Vec<Movement>, String> {
    if event_code.is_empty() {
        return Err("event is empty, expected letters from MPDS".to_string());
    }

    let mut movements: Vec<Movement> = Vec::new();

    for code in event_code.chars() {
        let movement = Movement::from_code(code)
            .ok_or_else(|| format!("unknown movement '{code}', expected letters from MPDS"))?;

        if let Some(previous) = movements.last()
            && previous.display_order() >= movement.display_order()
        {
            return Err(format!(
                "event '{event_code}' is out of order, write its letters as they appear in MPDS"
            ));
        }

        movements.push(movement);
    }

    Ok(movements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_four_movement_event_supports_calculated_ris() {
        assert!(is_full_event(Some("MPDS")));
    }

    #[test]
    fn a_partial_event_does_not() {
        for partial in ["M", "MP", "MPD", "PDS", "MPS"] {
            assert!(!is_full_event(Some(partial)), "{partial} is not full");
        }
    }

    #[test]
    fn an_unknown_event_does_not() {
        assert!(!is_full_event(None));
        assert!(!is_full_event(Some("SDPM")));
    }

    #[test]
    fn the_full_event_is_every_movement() {
        let codes: String = Movement::ALL.iter().map(Movement::code).collect();
        assert_eq!(codes, FULL_EVENT);
        assert_eq!(movements(FULL_EVENT).unwrap(), Movement::ALL.to_vec());
    }

    #[test]
    fn a_repeated_movement_is_rejected() {
        assert!(movements("MM").is_err());
    }

    #[test]
    fn an_out_of_order_event_is_rejected() {
        assert!(movements("SDPM").is_err());
        assert!(movements("SP").is_err());
    }

    #[test]
    fn a_letter_outside_the_four_is_rejected() {
        assert!(movements("B").is_err());
    }
}
