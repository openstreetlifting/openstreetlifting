//! Which movements a competition contested.
//!
//! A competition declares its movements and everyone in it contests all of them, so
//! the event belongs to the competition rather than to each athlete.
//!
//! A total is only a total within one event. Adding four lifts and adding one
//! produce numbers that must never be ranked against each other, and the same
//! goes for the RIS derived from them. A single movement is different: a
//! muscle-up is a muscle-up whatever else the competition ran, so those compare
//! freely across events.

use crate::movement::Movement;

/// The full four movements, in display order.
pub const FULL_EVENT: &str = "MPDS";

/// Whether a total and a RIS mean anything for this event.
///
/// The RIS formula divides by a benchmark fitted to four-lift totals, so
/// applying it to fewer movements measures against the wrong reference and
/// carries a bodyweight bias rather than removing one.
///
/// ```
/// use osl_domain::event::{is_full_event, FULL_EVENT};
///
/// assert!(is_full_event(Some(FULL_EVENT)));
/// assert!(!is_full_event(Some("M")));
/// assert!(!is_full_event(None));
/// ```
pub fn is_full_event(event_code: Option<&str>) -> bool {
    event_code == Some(FULL_EVENT)
}

/// The movements an event code names, rejecting a code whose letters are
/// unknown, repeated or out of display order. One set of movements therefore
/// has exactly one spelling, which is what lets `event_code` be compared as a
/// string.
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
    fn only_the_four_movement_event_carries_a_total() {
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
        // Order is fixed by display_order, so a reordered code is not the
        // full event and must not be treated as one.
        assert!(!is_full_event(Some("SDPM")));
    }

    #[test]
    fn the_full_event_is_every_movement() {
        let codes: String = Movement::ALL.iter().map(|m| m.code()).collect();
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
