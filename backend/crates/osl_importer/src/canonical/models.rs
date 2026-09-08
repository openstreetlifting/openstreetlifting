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
    pub country: CountryCode,
    pub bodyweight: Option<Decimal>,
    pub bodyweight_source: Option<BodyweightSource>,
    pub ris: Option<Decimal>,
    pub reported_ris_edition: Option<Edition>,
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

    pub fn complete_total(&self) -> Result<Decimal, String> {
        if self.status != AthleteStatus::Competed || self.status_reason.is_some() {
            return Err(
                "four-lift recovery requires a competed performance without a status reason".into(),
            );
        }
        let mut total = Decimal::ZERO;
        for movement in Movement::ALL {
            let mut lifts = self.lifts.iter().filter(|lift| lift.movement == movement);
            let lift = lifts
                .next()
                .ok_or_else(|| format!("missing {movement} result"))?;
            if lifts.next().is_some() {
                return Err(format!("duplicate {movement} result"));
            }
            total += lift.validated_best()?;
        }
        if total <= Decimal::ZERO {
            return Err("four-lift total must be positive".into());
        }
        Ok(total)
    }

    pub fn validate_score_source(
        &self,
        gender: Gender,
        movements: &[Movement],
    ) -> Result<(), String> {
        if self.bodyweight_source.is_some() && self.bodyweight.is_none() {
            return Err("BodyweightSource requires BodyweightKg".into());
        }
        if self.reported_ris_edition.is_some() && self.ris.is_none() {
            return Err("ReportedRisEdition requires the original Ris".into());
        }
        if self.bodyweight_source != Some(BodyweightSource::Recovered) {
            if self.bodyweight.is_some() && self.ris.is_some() {
                return Err("both bodyweight and ris require BodyweightSource=recovered and ReportedRisEdition".into());
            }
            return Ok(());
        }
        let bodyweight = self.bodyweight.ok_or("recovered bodyweight is missing")?;
        let ris = self
            .ris
            .ok_or("recovered bodyweight requires the original Ris")?;
        let edition = self
            .reported_ris_edition
            .ok_or("recovered bodyweight requires ReportedRisEdition")?;
        if bodyweight <= Decimal::ZERO || ris <= Decimal::ZERO {
            return Err("recovered bodyweight and original Ris must be positive".into());
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
        Ok(())
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
    pub fn validated_best(&self) -> Result<Decimal, String> {
        if self.best_lift.is_some_and(|weight| weight < Decimal::ZERO)
            || self
                .attempts
                .iter()
                .flatten()
                .any(|attempt| attempt.weight < Decimal::ZERO)
        {
            return Err(format!("negative {} lift", self.movement));
        }
        let best = self
            .best()
            .ok_or_else(|| format!("no successful {} result", self.movement))?;
        if let Some(stated) = self.best_lift
            && self.attempts.is_some()
            && stated != best
        {
            return Err(format!(
                "best {} contradicts successful attempts: expected {best}, got {}",
                self.movement, stated
            ));
        }
        Ok(best)
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
