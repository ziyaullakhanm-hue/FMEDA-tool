// use serde::{Serialize, Deserialize};
// use uuid::Uuid;
// use bigdecimal::{BigDecimal, ToPrimitive};
// use serde_json::Value;
// use std::collections::HashMap;
// use chrono::{DateTime, Utc};
// use sqlx::types::JsonValue;
// use sqlx::PgPool;

// // ------------------- Structs -------------------

// #[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
// pub struct Project {
//     pub id: Uuid,
//     pub name: String,
//     pub description: Option<String>,
//     pub created_at: DateTime<Utc>,
// }

// #[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
// pub struct Component {
//     pub id: Uuid,
//     pub project_id: Uuid,
//     pub manufacturer_part_number: String,
//     pub manufacturer: Option<String>,
//     pub reference_designator: Option<String>,
//     pub quantity: i32,
//     pub created_at: DateTime<Utc>,

//     pub component_type: String,
//     pub base_fit: Option<BigDecimal>,
//     pub quality_factor: Option<BigDecimal>,
//     pub resistor_type: Option<String>,
//     pub mission_profile_id: Option<Uuid>,
// }

// #[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
// pub struct MissionProfile {
//     pub id: Uuid,
//     pub name: String,
//     pub description: Option<String>,
//     pub temp_tau_profile: Option<JsonValue>,
//     pub created_at: DateTime<Utc>,

//     pub temperature_factor: Option<f64>,
//     pub environment_factor: Option<f64>,
//     pub stress_factor: Option<f64>,
//     pub reference_temp: Option<f64>,
//     pub operating_temp: Option<f64>,
// }

// #[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
// pub struct FailureMode {
//     pub id: Uuid,
//     pub mpn: Option<String>,
//     pub family: Option<String>,
//     pub mode: String,
//     pub lambda: f64,
//     pub detection_coverage: Option<f32>,
//     pub created_at: DateTime<Utc>,
// }

// #[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
// pub struct Calculation {
//     pub id: Uuid,
//     pub project_id: Uuid,
//     pub payload: Value,
//     pub result: Value,
//     pub created_at: DateTime<Utc>,
// }

// // ------------------- Database Functions -------------------

// impl Component {
//     /// Fetch a single component by ID
//     pub async fn fetch(pool: &PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
//         let row = sqlx::query_as!(
//             Component,
//             r#"
//             SELECT id, project_id, manufacturer_part_number, manufacturer,
//                    reference_designator, quantity, created_at,
//                    component_type, base_fit, quality_factor, resistor_type,
//                    mission_profile_id
//             FROM components
//             WHERE id = $1
//             "#,
//             id
//         )
//         .fetch_one(pool)
//         .await?;

//         Ok(row)
//     }
// }

// impl MissionProfile {
//     /// Fetch a single mission profile by ID including temperature_factor
//     pub async fn fetch(pool: &PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
//         let row = sqlx::query_as!(
//             MissionProfile,
//             r#"
//             SELECT id, name, description, temp_tau_profile, created_at,
//                    temperature_factor, environment_factor, stress_factor,
//                    reference_temp, operating_temp
//             FROM mission_profiles
//             WHERE id = $1
//             "#,
//             id
//         )
//         .fetch_one(pool)
//         .await?;

//         Ok(row)
//     }
// }

// // ------------------- FMEDA Prediction -------------------

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct FMEDAPredictionResult {
//     pub component_id: Uuid,
//     pub total_fit: f64,
//     pub failure_modes: HashMap<String, f64>,
// }

// pub fn predict_fmeda(
//     component: &Component,
//     profile: &MissionProfile,
//     failure_modes: &[FailureMode],
// ) -> FMEDAPredictionResult {
//     let base_fit = component.base_fit.clone().unwrap_or_else(|| BigDecimal::from(1));
//     let quality = component.quality_factor.clone().unwrap_or_else(|| BigDecimal::from(1));
//     let temp = profile.temperature_factor.unwrap_or(1.0);
//     let env = profile.environment_factor.unwrap_or(1.0);
//     let stress = profile.stress_factor.unwrap_or(1.0);

//     let total_fit = base_fit.to_f64().unwrap_or(1.0)
//         * quality.to_f64().unwrap_or(1.0)
//         * temp
//         * env
//         * stress
//         * component.quantity as f64;

//     let mut failure_modes_fit = HashMap::new();
//     let lambda_sum: f64 = failure_modes.iter().map(|fm| fm.lambda).sum();
//     for fm in failure_modes {
//         let mode_fit = if lambda_sum > 0.0 {
//             total_fit * (fm.lambda / lambda_sum)
//         } else {
//             0.0
//         };
//         failure_modes_fit.insert(fm.mode.clone(), mode_fit);
//     }

//     FMEDAPredictionResult {
//         component_id: component.id,
//         total_fit,
//         failure_modes: failure_modes_fit,
//     }
// }
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bigdecimal::{BigDecimal, ToPrimitive};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc}; // ✅ Changed from DateTime<Utc>
use sqlx::PgPool;

// ------------------- Structs -------------------

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>, // updated
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Component {
    pub id: Uuid,
    pub project_id: Uuid,
    pub manufacturer_part_number: String,
    pub manufacturer: Option<String>,
    pub reference_designator: Option<String>,
    pub quantity: i32,
    pub created_at: DateTime<Utc>, // updated

    pub component_type: String,
    pub base_fit: Option<BigDecimal>,
    pub quality_factor: Option<BigDecimal>,
    pub resistor_type: Option<String>,
    pub mission_profile_id: Option<Uuid>,
    pub variant_id: Option<Uuid>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct ComponentVariant {
    pub id: Uuid,
    pub subtype_id: Uuid,
    pub name: String,
    pub ref_fit: f64,
    pub ref_temp: Option<f64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>, // updated
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct MissionProfile {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub temp_tau_profile: serde_json::Value,
    pub created_at: DateTime<Utc>, // updated

    pub reference_temp: Option<f64>,
    pub operating_temp: Option<f64>,
}

impl MissionProfile {
    pub fn parse_temp_tau_profile(&self) -> Vec<(f64, f64)> {
        if let Value::Array(items) = &self.temp_tau_profile {
            items
                .iter()
                .filter_map(|v| {
                    let t = v.get("temp")?.as_f64()?;
                    let tau = v.get("tau")?.as_f64()?;
                    Some((t, tau))
                })
                .collect()
        } else {
            vec![]
        }
    }

    pub async fn fetch(pool: &PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        let row = sqlx::query_as::<_, MissionProfile>(
            r#"
            SELECT id, name, description, temp_tau_profile, created_at,
                reference_temp, operating_temp
            FROM mission_profiles
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;



        Ok(row)
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct FailureMode {
    pub id: Uuid,
    pub mpn: Option<String>,
    pub family: Option<String>,
    pub mode: String,
    pub lambda: f64,
    pub detection_coverage: Option<f32>,
    pub created_at: DateTime<Utc>, // updated
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Calculation {
    pub id: Uuid,
    pub project_id: Uuid,
    pub payload: Value,
    pub result: Value,
    pub created_at: DateTime<Utc>, // updated
}

// ------------------- FMEDA Prediction -------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FMEDAPredictionResult {
    pub component_id: Uuid,
    pub total_fit: f64,
    pub failure_modes: HashMap<String, f64>,
}

pub fn predict_fmeda(
    component: &Component,
    profile: &MissionProfile,
    failure_modes: &[FailureMode],
) -> FMEDAPredictionResult {
    let base_fit = component.base_fit.clone().unwrap_or_else(|| BigDecimal::from(1));
    let quality = component.quality_factor.clone().unwrap_or_else(|| BigDecimal::from(1));

    let temp_tau = profile.parse_temp_tau_profile();
    let temp_factor = if !temp_tau.is_empty() {
        temp_tau.iter().map(|(t, tau)| tau * t).sum::<f64>()
    } else {
        1.0
    };

    let total_fit = base_fit.to_f64().unwrap_or(1.0)
        * quality.to_f64().unwrap_or(1.0)
        * temp_factor
        * component.quantity as f64;

    let mut failure_modes_fit = HashMap::new();
    let lambda_sum: f64 = failure_modes.iter().map(|fm| fm.lambda).sum();
    for fm in failure_modes {
        let mode_fit = if lambda_sum > 0.0 {
            total_fit * (fm.lambda / lambda_sum)
        } else {
            0.0
        };
        failure_modes_fit.insert(fm.mode.clone(), mode_fit);
    }

    FMEDAPredictionResult {
        component_id: component.id,
        total_fit,
        failure_modes: failure_modes_fit,
    }
}
