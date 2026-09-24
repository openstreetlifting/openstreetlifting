use chrono::NaiveDate;
use osl_domain::{
    AthleteStatus, CompetitionStatus, CountryCode, Edition, Gender, Movement, WeightClassSlug,
    category_label, display_name,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct CanonicalFormat {
    pub sources: Vec<String>,
    pub competition: CompetitionData,
    pub movements: Vec<Movement>,
    pub categories: Vec<CategoryData>,
}

#[derive(Debug, Clone)]
pub struct CompetitionData {
    pub name: String,
    pub slug: String,
    pub federation: FederationData,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub city: Option<String>,
    pub venue: Option<String>,
    pub region: Option<String>,
    pub country: CountryCode,
    pub status: Option<CompetitionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FederationData {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<CountryCode>,
}

#[derive(Debug, Clone)]
pub struct CategoryData {
    pub division: Option<String>,
    pub gender: Gender,
    pub weight_class_slug: Option<WeightClassSlug>,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
    pub athletes: Vec<AthleteData>,
}

impl CategoryData {
    pub fn bounds(&self) -> (Option<Decimal>, Option<Decimal>) {
        match self.weight_class_slug.as_ref() {
            Some(slug) => slug.bounds(),
            None => (self.weight_class_min, self.weight_class_max),
        }
    }

    pub fn label(&self) -> String {
        let (min, max) = self.bounds();
        category_label(self.division.as_deref(), self.gender, min, max)
    }
}

#[derive(Debug, Clone)]
pub struct AthleteData {
    pub first_name: String,
    pub last_name: String,
    pub native_name: Option<String>,
    pub disambiguation: Option<i16>,
    pub gender: Option<Gender>,
    pub country: Option<CountryCode>,
    pub bodyweight: Option<Decimal>,
    pub bodyweight_source: Option<BodyweightSource>,
    pub reported_ris: Option<Decimal>,
    pub reported_ris_edition: Option<Edition>,
    pub total: Option<Decimal>,
    pub status: AthleteStatus,
    pub status_reason: Option<String>,
    pub lifts: Vec<LiftData>,
}

impl AthleteData {
    pub fn display_name(&self) -> String {
        display_name(&self.first_name, &self.last_name)
    }

    pub fn bodyweight_source(&self) -> Option<BodyweightSource> {
        self.bodyweight_source
            .or(self.bodyweight.map(|_| BodyweightSource::Reported))
    }

    /// Sum the event's successful bests only when every movement is present.
    pub fn total_from_lifts(&self, movements: &[Movement]) -> Option<Decimal> {
        if self.status != AthleteStatus::Competed || movements.is_empty() {
            return None;
        }
        movements.iter().try_fold(Decimal::ZERO, |total, movement| {
            self.lifts
                .iter()
                .find(|lift| lift.movement == *movement)
                .and_then(LiftData::best)
                .map(|best| total + best)
        })
    }

    pub fn validate_total(&self, movements: &[Movement]) -> Result<(), String> {
        let mut seen = std::collections::HashSet::new();
        let mut known = Decimal::ZERO;
        for lift in &self.lifts {
            if !movements.contains(&lift.movement) {
                return Err(format!("{} is not in the event", lift.movement));
            }
            if !seen.insert(lift.movement) {
                return Err(format!("duplicate {} result", lift.movement));
            }
            lift.validate_evidence()?;
            if self.status == AthleteStatus::Competed {
                known += lift
                    .best()
                    .ok_or_else(|| format!("no successful {} result", lift.movement))?;
            }

        }
        let complete = self.total_from_lifts(movements);
        match self.total {
            Some(total) => {
                if total < Decimal::ZERO {
                    return Err("TotalKg cannot be negative".into());
                }
                if self.status != AthleteStatus::Competed || self.status_reason.is_some() {
                    return Err("TotalKg requires a competed result without a status reason".into());
                }
                if movements.is_empty() {
                    return Err("TotalKg requires an event".into());
                }
                if total < known || complete.is_some_and(|sum| sum != total) {
                    return Err(format!(
                        "TotalKg {total} contradicts the {}lift sum {known}",
                        if complete.is_some() {
                            "complete "
                        } else {
                            "known "
                        }
                    ));
                }
            }
            None if complete.is_some() => {
                return Err(
                    "TotalKg is missing for a complete breakdown; run `osl-import prepare`".into(),
                );
            }
            None => {}
        }
        Ok(())
    }

    pub fn complete_total(&self) -> Result<Decimal, String> {
        self.validate_total(&Movement::ALL)?;
        let total = self
            .total
            .ok_or("TotalKg is missing; run `osl-import prepare` when all lifts are available")?;
        if total <= Decimal::ZERO {
            return Err("RIS requires a positive TotalKg".into());
        }
        Ok(total)
    }

    /// Reject inconsistent evidence and explain when a published score cannot be checked.
    pub fn validate_score_source(
        &self,
        gender: Gender,
        movements: &[Movement],
    ) -> Result<Option<String>, String> {
        self.validate_total(movements)?;
        if self.bodyweight_source.is_some() && self.bodyweight.is_none() {
            return Err("BodyweightSource requires BodyweightKg".into());
        }
        if self.reported_ris_edition.is_some() && self.reported_ris.is_none() {
            return Err("ReportedRisEdition requires ReportedRis".into());
        }
        if self.bodyweight_source != Some(BodyweightSource::Recovered) {
            let (Some(bodyweight), Some(reported_ris)) = (self.bodyweight, self.reported_ris)
            else {
                return Ok(None);
            };
            let Some(edition) = self.reported_ris_edition else {
                return Ok(Some(
                    "ReportedRis is unverified: its formula edition is unknown; supply ReportedRisEdition when established by the source".into(),
                ));
            };
            let gender = self.gender.unwrap_or(gender);
            let total = match self.complete_total() {
                Ok(total) if movements == Movement::ALL && gender != Gender::Mx => total,
                _ => return Ok(Some(
                    "ReportedRis is unverified: comparison requires a complete, competed four-movement result and an M or F scoring formula".into(),
                )),
            };
            let computed = osl_domain::ris::compute(bodyweight, total, gender, edition);
            if computed != reported_ris {
                return Err(format!(
                    "ReportedRis {reported_ris} contradicts BodyweightKg and the complete total: edition {} gives {computed}",
                    edition.year()
                ));
            }
            return Ok(None);
        }
        let bodyweight = self.bodyweight.ok_or("recovered bodyweight is missing")?;
        let ris = self
            .reported_ris
            .ok_or("recovered bodyweight requires ReportedRis")?;
        let edition = self
            .reported_ris_edition
            .ok_or("recovered bodyweight requires ReportedRisEdition")?;
        if bodyweight <= Decimal::ZERO || ris <= Decimal::ZERO {
            return Err("recovered bodyweight and ReportedRis must be positive".into());
        }
        if self.status != AthleteStatus::Competed || movements != Movement::ALL {
            return Err(
                "recovered bodyweight requires a competed four-movement performance".into(),
            );
        }
        let total = self.complete_total()?;
        let computed =
            osl_domain::ris::compute(bodyweight, total, self.gender.unwrap_or(gender), edition);
        if computed != ris {
            return Err(format!(
                "recovered bodyweight reproduces RIS {computed} under edition {}, but the source reports {ris}",
                edition.year()
            ));
        }
        Ok(None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyweightSource {
    Reported,
    Recovered,
}

impl BodyweightSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reported => "reported",
            Self::Recovered => "recovered",
        }
    }
}

impl std::str::FromStr for BodyweightSource {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "reported" => Ok(Self::Reported),
            "recovered" => Ok(Self::Recovered),
            _ => Err(format!(
                "unknown BodyweightSource '{value}', expected reported or recovered"
            )),
        }
    }
}

impl LiftData {
    pub fn validate_evidence(&self) -> Result<(), String> {
        if self.best_lift.is_some_and(|weight| weight < Decimal::ZERO)
            || self
                .attempts
                .iter()
                .flatten()
                .any(|attempt| attempt.weight < Decimal::ZERO)
        {
            return Err(format!("negative {} lift", self.movement));
        }
        if let Some(attempts) = &self.attempts {
            let mut numbers = std::collections::HashSet::new();
            if attempts.is_empty()
                || attempts.iter().any(|attempt| {
                    !(1..=super::entries::ATTEMPTS_PER_MOVEMENT).contains(&attempt.attempt_number)
                        || !numbers.insert(attempt.attempt_number)
                })
            {
                return Err(format!(
                    "{} attempts must have distinct numbers from 1 to 3",
                    self.movement
                ));
            }
            if let Some(stated) = self.best_lift
                && self.best() != Some(stated)
            {
                return Err(format!(
                    "best {} contradicts successful attempts",
                    self.movement
                ));
            }
        }
        Ok(())
    }

    pub fn validated_best(&self) -> Result<Decimal, String> {
        self.validate_evidence()?;
        self.best()
            .ok_or_else(|| format!("no successful {} result", self.movement))
    }

    /// What the competition page shows for the movement, and what the importer
    /// stores as `max_weight`. Derived whenever the attempts are known, so the
    /// column can never contradict them.
    pub fn best(&self) -> Option<Decimal> {
        match self.attempts.as_ref() {
            Some(attempts) => attempts
                .iter()
                .filter(|attempt| attempt.is_successful)
                .map(|attempt| attempt.weight)
                .max(),
            None => self.best_lift,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiftData {
    pub movement: Movement,
    pub attempts: Option<Vec<AttemptData>>,
    pub best_lift: Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct AttemptData {
    pub attempt_number: i16,
    pub weight: Decimal,
    pub is_successful: bool,
}
