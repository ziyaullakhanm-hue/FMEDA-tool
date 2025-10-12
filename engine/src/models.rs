use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::DateTime;
use chrono::Utc;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Component {
    pub id: Uuid,
    pub project_id: Uuid,
    pub manufacturer_part_number: String,
    pub manufacturer: Option<String>,
    pub reference_designator: Option<String>,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,

    // Added for calculation
    pub component_type: String, // e.g., "Resistor", "Capacitor", "IC"
    pub base_fit: Option<f64>,
    pub quality_factor: Option<f64>,
    pub resistor_type: Option<String>, // e.g., "carbon film", "metal film", etc.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MissionProfile {
    pub temperature_factor: Option<f64>,
    pub environment_factor: Option<f64>,
    pub stress_factor: Option<f64>,
    pub reference_temp: Option<f64>, // Reference temperature (thetaref) in °C
    pub operating_temp: Option<f64>, // Actual operating temperature (theta) in °C
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct FailureMode {
    pub id: Uuid,
    pub mpn: Option<String>,
    pub family: Option<String>,
    pub mode: String,
    pub lambda: f64,
    pub detection_coverage: Option<f32>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Calculation {
    pub id: Uuid,
    pub project_id: Uuid,
    pub payload: serde_json::Value,
    pub result: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

// --- FMEDA Prediction Example ---
use std::collections::HashMap;

/// Example result struct for FMEDA prediction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FMEDAPredictionResult {
    pub component_id: Uuid,
    pub total_fit: f64,
    pub failure_modes: HashMap<String, f64>, // mode -> FIT
}

/// Example FMEDA prediction function
pub fn predict_fmeda(
    component: &Component,
    profile: &MissionProfile,
    failure_modes: &[FailureMode],
) -> FMEDAPredictionResult {
    // Calculate base FIT (could use SN29500 or other model)
    let base_fit = component.base_fit.unwrap_or(1.0);
    let quality = component.quality_factor.unwrap_or(1.0);
    let temp = profile.temperature_factor.unwrap_or(1.0);
    let env = profile.environment_factor.unwrap_or(1.0);
    let stress = profile.stress_factor.unwrap_or(1.0);
    let total_fit = base_fit * quality * temp * env * stress * component.quantity as f64;

    // Distribute FIT to failure modes (example: lambda is a fraction)
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
