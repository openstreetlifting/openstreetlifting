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
    fn supports_calculated_ris_for_full_events() {
        assert!(is_full_event(Some("MPDS")));
    }

    #[test]
    fn rejects_calculated_ris_for_partial_events() {
        for partial in ["M", "MP", "MPD", "PDS", "MPS"] {
            assert!(!is_full_event(Some(partial)), "{partial} is not full");
        }
    }

    #[test]
    fn rejects_calculated_ris_for_unknown_events() {
        assert!(!is_full_event(None));
        assert!(!is_full_event(Some("SDPM")));
    }

    #[test]
    fn includes_all_movements_in_full_event() {
        let codes: String = Movement::ALL.iter().map(Movement::code).collect();
        assert_eq!(codes, FULL_EVENT);
        assert_eq!(movements(FULL_EVENT).unwrap(), Movement::ALL.to_vec());
    }

    #[test]
    fn rejects_repeated_movements() {
        assert!(movements("MM").is_err());
    }

    #[test]
    fn rejects_out_of_order_movements() {
        assert!(movements("SDPM").is_err());
        assert!(movements("SP").is_err());
    }

    #[test]
    fn rejects_unknown_movements() {
        assert!(movements("B").is_err());
    }
}
