//! Which movements a competition contested.
//!
//! A meet declares its movements and everyone in it contests all of them, so
//! the event belongs to the competition rather than to each athlete. That is
//! the one place streetlifting differs from powerlifting, where a lifter can
//! enter bench-only at a full-power meet and OpenPowerlifting has to put the
//! event on the entry.
//!
//! A total is only a total within one event. Adding four lifts and adding one
//! produce numbers that must never be ranked against each other, and the same
//! goes for the RIS derived from them. A single movement is different: a
//! muscle-up is a muscle-up whatever else the meet ran, so those compare
//! freely across events.

/// The full four movements, in display order. The letters come from
/// `movements.code` in the database.
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
}
