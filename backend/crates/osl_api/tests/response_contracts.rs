use osl_api::{
    ApiDoc,
    athlete::dto::{
        AthleteResponse, AthleteStanding, MetricStanding, StandingPlace, StrengthProfile,
    },
    competition::dto::{CategoryInfo, CompetitionResponse},
    ranking::dto::{AthleteInfo, CompetitionInfo, FederationInfo, GlobalRankingEntry},
    ris::dto::ComputeRisRequest,
};
use osl_db::{
    projections::athlete::AthleteDetail,
    rows::{athlete::AthleteRow, competition::CompetitionRow},
};
use osl_domain::{CompetitionStatus, Gender};
use serde_json::{Value, json};
use utoipa::OpenApi;
use uuid::Uuid;

const COMPETITION_SECTIONS: &[&str] = &["federation", "movements", "categories", "editions"];

fn athlete_row() -> AthleteRow {
    AthleteRow {
        athlete_id: Uuid::nil(),
        first_name: "Alex".into(),
        last_name: "Martin".into(),
        native_name: None,
        gender: Gender::M,
        created_at: chrono::NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        country: None,
        profile_picture_url: None,
        slug: "alex-martin".into(),
        slug_history: sqlx::types::Json(Vec::new()),
    }
}

fn competition() -> CompetitionResponse {
    CompetitionRow {
        competition_id: Uuid::nil(),
        name: "Test competition".into(),
        created_at: athlete_row().created_at,
        slug: "test-competition".into(),
        status: CompetitionStatus::Upcoming,
        scoring: None,
        federation_id: Uuid::nil(),
        city: None,
        venue: None,
        region: None,
        country: None,
        start_date: None,
        end_date: None,
    }
    .into()
}

#[test]
fn serializes_missing_athlete_values_as_null() {
    let body = serde_json::to_value(AthleteResponse::from(athlete_row())).unwrap();
    for field in [
        "native_name",
        "country",
        "profile_picture_url",
        "instagram_handle",
    ] {
        assert_eq!(body.get(field), Some(&Value::Null), "{field}");
    }
    for field in [
        "competitions",
        "personal_records",
        "total_competitions",
        "standing",
        "strength_profile",
    ] {
        assert!(!body.as_object().unwrap().contains_key(field), "{field}");
    }
}

#[test]
fn serializes_requested_empty_sections() {
    let detail = AthleteDetail {
        athlete: athlete_row(),
        competitions: Vec::new(),
        personal_records: Vec::new(),
        total_competitions: 0,
        instagram_handle: None,
    };
    let include =
        serde_json::from_value(json!("competitions,records,standing,strength_profile")).unwrap();
    let mut response = AthleteResponse::from_detail(detail, &include);
    response.standing = Some(None);
    response.strength_profile = Some(None);
    let body = serde_json::to_value(response).unwrap();
    assert_eq!(body["competitions"], json!([]));
    assert_eq!(body["personal_records"], json!([]));
    assert_eq!(body["total_competitions"], 0);
    assert_eq!(body.get("standing"), Some(&Value::Null));
    assert_eq!(body.get("strength_profile"), Some(&Value::Null));
}

#[test]
fn preserves_populated_sections_and_null_metrics() {
    let mut response = AthleteResponse::from(athlete_row());
    response.native_name = Some("Алекс Мартин".into());
    response.country = Some("FR".into());
    response.instagram_handle = Some("alex.martin".into());
    response.standing = Some(Some(AthleteStanding {
        ris: Some(MetricStanding {
            value: 100.into(),
            class: None,
            global: StandingPlace { place: 1, field: 2 },
            country: None,
        }),
        total: None,
        muscleup: None,
        pullup: None,
        dips: None,
        squat: None,
    }));
    response.strength_profile = Some(Some(StrengthProfile {
        category: "Men -80kg".into(),
        lifts: Vec::new(),
    }));
    let body = serde_json::to_value(response).unwrap();
    assert_eq!(body["native_name"], "Алекс Мартин");
    assert_eq!(body["country"], "FR");
    assert_eq!(body["instagram_handle"], "alex.martin");
    assert_eq!(body["standing"]["ris"]["value"], "100");
    assert_eq!(body["standing"]["ris"].get("class"), Some(&Value::Null));
    assert_eq!(body["standing"]["ris"].get("country"), Some(&Value::Null));
    for metric in ["total", "muscleup", "pullup", "dips", "squat"] {
        assert_eq!(body["standing"].get(metric), Some(&Value::Null));
    }
    assert_eq!(body["strength_profile"]["category"], "Men -80kg");
    assert_eq!(body["strength_profile"]["lifts"], json!([]));
}

#[test]
fn omits_unrequested_competition_sections() {
    let mut response = competition();
    let body = serde_json::to_value(&response).unwrap();
    for field in ["city", "venue", "country", "event_code", "lifter_count"] {
        assert_eq!(body.get(field), Some(&Value::Null), "{field}");
    }
    for &field in COMPETITION_SECTIONS {
        assert!(!body.as_object().unwrap().contains_key(field), "{field}");
    }
    response.movements = Some(Vec::new());
    response.categories = Some(Vec::new());
    response.editions = Some(Vec::new());
    let body = serde_json::to_value(response).unwrap();
    assert_eq!(body["movements"], json!([]));
    assert_eq!(body["categories"], json!([]));
    assert_eq!(body["editions"], json!([]));
    let category = CategoryInfo {
        name: "Men -80kg".into(),
        division: None,
        gender: Gender::M,
        weight_class: "-80kg".into(),
    };
    assert_eq!(
        serde_json::to_value(category).unwrap().get("division"),
        Some(&Value::Null)
    );
}

#[test]
fn preserves_zero_and_null_ranking_values() {
    let response = GlobalRankingEntry {
        participant_id: Uuid::nil(),
        rank: 1,
        athlete: AthleteInfo {
            athlete_id: Uuid::nil(),
            first_name: "Alex".into(),
            last_name: "Martin".into(),
            slug: "alex-martin".into(),
            country: None,
            gender: Gender::M,
            bodyweight: None,
            instagram_handle: None,
        },
        category: "Men -80kg".into(),
        division: None,
        ris: None,
        ris_source: None,
        total: Some(0.0),
        muscleup: Some(0.0),
        pullup: None,
        dips: None,
        squat: None,
        event: None,
        competition: CompetitionInfo {
            competition_id: Uuid::nil(),
            name: "Test competition".into(),
            slug: "test-competition".into(),
            date: None,
        },
        federation: FederationInfo {
            name: "Test federation".into(),
            abbreviation: None,
        },
    };
    let body = serde_json::to_value(response).unwrap();
    for field in [
        "division",
        "ris",
        "ris_source",
        "pullup",
        "dips",
        "squat",
        "event",
    ] {
        assert_eq!(body.get(field), Some(&Value::Null), "{field}");
    }
    assert_eq!(body["athlete"].get("instagram_handle"), Some(&Value::Null));
    assert_eq!(body["total"], 0.0);
    assert_eq!(body["muscleup"], 0.0);
}

#[test]
fn requires_fields_except_conditional_sections() {
    let schema = serde_json::to_value(ApiDoc::openapi()).unwrap();
    for (name, definition) in schema["components"]["schemas"].as_object().unwrap() {
        if matches!(name.as_str(), "ComputeRisRequest" | "PaginationParams") {
            continue;
        }
        let Some(properties) = definition["properties"].as_object() else {
            continue;
        };
        for field in properties.keys() {
            let optional = match name.as_str() {
                "AthleteResponse" => matches!(
                    field.as_str(),
                    "competitions"
                        | "personal_records"
                        | "total_competitions"
                        | "standing"
                        | "strength_profile"
                ),
                "CompetitionResponse" => COMPETITION_SECTIONS.contains(&field.as_str()),
                _ => false,
            };
            let required = definition["required"]
                .as_array()
                .is_some_and(|fields| fields.iter().any(|value| value == field));
            assert_eq!(required, !optional, "{name}.{field}");
        }
    }
    for field in ["standing", "strength_profile"] {
        let property = &schema["components"]["schemas"]["AthleteResponse"]["properties"][field];
        assert!(
            property["oneOf"]
                .as_array()
                .unwrap()
                .iter()
                .any(|branch| branch["type"] == "null")
        );
    }
}

#[test]
fn validates_request_field_presence() {
    for extra in [
        json!({}),
        json!({"formula_year": null}),
        json!({"formula_year": 2025}),
    ] {
        let mut body = json!({"bodyweight": "80", "total": "300", "gender": "M"});
        body.as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        let request: ComputeRisRequest = serde_json::from_value(body).unwrap();
        assert_eq!(
            request.formula_year,
            extra["formula_year"]
                .as_i64()
                .map(|year| i32::try_from(year).unwrap())
        );
    }
    assert!(
        serde_json::from_value::<ComputeRisRequest>(json!({"total": "300", "gender": "M"}))
            .is_err()
    );
    assert!(
        serde_json::from_value::<ComputeRisRequest>(
            json!({"bodyweight": null, "total": "300", "gender": "M"})
        )
        .is_err()
    );
}
