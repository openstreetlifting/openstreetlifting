use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct RankingRow {
    pub rank: i64,
    pub athlete_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    pub country: String,
    pub gender: String,
    pub instagram_handle: Option<String>,
    pub bodyweight: Option<Decimal>,
    pub division: Option<String>,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
    pub competition_id: Uuid,
    pub competition_name: String,
    pub competition_slug: String,
    pub start_date: Option<NaiveDate>,
    pub federation_name: String,
    pub federation_abbreviation: Option<String>,
    /// Absent when the competition did not contest the movement, rather than zero.
    pub muscleup: Option<Decimal>,
    pub pullup: Option<Decimal>,
    pub dips: Option<Decimal>,
    pub squat: Option<Decimal>,
    /// The sum of what this athlete contested, so it only compares with
    /// another total from the same event.
    pub total: Option<Decimal>,
    pub event_code: Option<String>,
    pub ris_score: Option<Decimal>,
    pub ris_source: Option<String>,
}

/// Where one athlete stands on the RIS board, over everyone and inside their
/// own country. Absent for an athlete the board does not rank, since only a
/// four movement total earns the score it is ordered on.
#[derive(Debug, FromRow)]
pub struct AthleteStandingRow {
    /// The score the place was taken on, which is the best they have scored.
    pub ris_score: Option<Decimal>,
    pub global_place: i64,
    pub global_field: i64,
    pub country: String,
    pub country_place: i64,
    pub country_field: i64,
}

/// Where one athlete stands on total inside their own weight class, over
/// everyone in it and inside their country. A lifter moves between classes over
/// a career, so the class is the one they were in for their best total.
#[derive(Debug, FromRow)]
pub struct AthleteClassStandingRow {
    /// The total the place was taken on, their best in this class.
    pub total: Option<Decimal>,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
    pub country: String,
    pub class_place: i64,
    pub class_field: i64,
    pub class_country_place: i64,
    pub class_country_field: i64,
}
