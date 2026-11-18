use osl_domain::{AthleteStatus, Gender, Movement, RecordMark, RisSource, is_full_event};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashSet;
use uuid::Uuid;

use crate::projections::competition::AttemptSummary;
use crate::rows::athlete::AthleteRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AthleteLiftRow {
    pub movement_name: Movement,
    pub best_weight: Option<Decimal>,
    pub record: Option<RecordMark>,
    pub attempts: Vec<AttemptSummary>,
}

#[derive(Debug, FromRow)]
pub struct AthleteCompetitionRow {
    pub competition_id: Uuid,
    pub competition_name: String,
    pub competition_slug: String,
    pub competition_date: chrono::NaiveDate,
    pub division: Option<String>,
    pub category_gender: Gender,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
    pub rank: Option<i32>,
    pub total: Option<Decimal>,
    pub total_record: Option<RecordMark>,
    pub ris_score: Option<Decimal>,
    pub ris_source: Option<RisSource>,
    pub status: AthleteStatus,
    pub event_code: Option<String>,
    pub lifts: Vec<AthleteLiftRow>,
}

#[derive(Debug)]
pub struct PersonalRecordRow {
    pub movement_name: Movement,
    pub max_weight: Decimal,
    pub record: Option<RecordMark>,
    pub competition_name: String,
    pub competition_slug: String,
    pub date: chrono::NaiveDate,
}

#[derive(Debug)]
pub struct BestTotalRow {
    pub total: Decimal,
    pub record: Option<RecordMark>,
    pub competition_name: String,
    pub competition_slug: String,
    pub date: chrono::NaiveDate,
}

#[derive(Debug)]
pub struct BestRisRow {
    pub ris_score: Decimal,
    pub ris_source: RisSource,
    pub competition_name: String,
    pub competition_slug: String,
    pub date: chrono::NaiveDate,
}

#[derive(Debug)]
pub struct AthleteRecords {
    pub personal_records: Vec<PersonalRecordRow>,
    pub best_total: Option<BestTotalRow>,
    pub best_ris: Option<BestRisRow>,
}

impl AthleteRecords {
    #[must_use]
    pub fn from_history(history: &[AthleteCompetitionRow]) -> Self {
        let personal_records = Movement::ALL
            .into_iter()
            .filter_map(|movement| {
                history
                    .iter()
                    .filter(|competition| competition.status.competed())
                    .filter_map(|competition| {
                        let lift = competition
                            .lifts
                            .iter()
                            .find(|lift| lift.movement_name == movement)?;
                        Some((competition, lift.best_weight?, lift.record))
                    })
                    .max_by_key(|(competition, weight, record)| {
                        (*weight, competition.competition_date, record.is_some())
                    })
                    .map(|(competition, max_weight, record)| PersonalRecordRow {
                        movement_name: movement,
                        max_weight,
                        record,
                        competition_name: competition.competition_name.clone(),
                        competition_slug: competition.competition_slug.clone(),
                        date: competition.competition_date,
                    })
            })
            .collect();

        let best_total = full_meets(history)
            .filter_map(|competition| Some((competition, competition.total?)))
            .max_by_key(|(competition, total)| {
                (
                    *total,
                    competition.total_record.is_some(),
                    competition.competition_date,
                )
            })
            .map(|(competition, total)| BestTotalRow {
                total,
                record: competition.total_record,
                competition_name: competition.competition_name.clone(),
                competition_slug: competition.competition_slug.clone(),
                date: competition.competition_date,
            });
        let best_ris = full_meets(history)
            .filter_map(|competition| {
                Some((competition, competition.ris_score?, competition.ris_source?))
            })
            .max_by_key(|(competition, ris_score, _)| (*ris_score, competition.competition_date))
            .map(|(competition, ris_score, ris_source)| BestRisRow {
                ris_score,
                ris_source,
                competition_name: competition.competition_name.clone(),
                competition_slug: competition.competition_slug.clone(),
                date: competition.competition_date,
            });

        Self {
            personal_records,
            best_total,
            best_ris,
        }
    }
}

fn full_meets(history: &[AthleteCompetitionRow]) -> impl Iterator<Item = &AthleteCompetitionRow> {
    history.iter().filter(|competition| {
        competition.status.competed() && is_full_event(competition.event_code.as_deref())
    })
}

// Entering multiple divisions at one meet still counts as one competition.
#[must_use]
#[expect(clippy::cast_possible_wrap)]
pub fn count_competitions(history: &[AthleteCompetitionRow]) -> i64 {
    history
        .iter()
        .map(|competition| competition.competition_id)
        .collect::<HashSet<_>>()
        .len() as i64
}

#[derive(Debug)]
pub struct AthleteDetail {
    pub athlete: AthleteRow,
    pub competitions: Vec<AthleteCompetitionRow>,
    pub personal_records: Vec<PersonalRecordRow>,
    pub best_total: Option<BestTotalRow>,
    pub best_ris: Option<BestRisRow>,
    pub total_competitions: i64,
    pub instagram_handle: Option<String>,
}

#[derive(Debug)]
pub struct AthleteBalanceRow {
    pub category_gender: Gender,
    pub weight_class_min: Option<Decimal>,
    pub weight_class_max: Option<Decimal>,
    pub movement_name: Movement,
    pub value: Decimal,
    pub share: f64,
    pub total: Decimal,
    pub competition_name: String,
    pub competition_slug: String,
    pub competition_date: chrono::NaiveDate,
    pub field: i64,
    pub class_share: Option<f64>,
    pub class_spread: Option<f64>,
}
