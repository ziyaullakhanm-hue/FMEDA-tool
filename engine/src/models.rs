use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Component {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub manufacturer_part_number: String,
    pub manufacturer: Option<String>,
    pub reference_designator: Option<String>,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FailureMode {
    pub id: Uuid,
    pub mpn: Option<String>,
    pub family: Option<String>,
    pub mode: String,
    // stored as FIT (failures per 1e9 hours)
    pub lambda: f64,
    // detection coverage 0.0 - 1.0
    pub detection_coverage: Option<f32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationRequest {
    pub project_id: uuid::Uuid,
    pub component_ids: Vec<uuid::Uuid>,
    // optional overrides or parameters
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationResult {
    pub project_id: uuid::Uuid,
    pub summary: serde_json::Value,
}
