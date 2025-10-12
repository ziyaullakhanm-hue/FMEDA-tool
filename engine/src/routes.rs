use axum::{extract::Json, routing::post, Router};
use serde::Deserialize;
use crate::models::Component;
use crate::calc::calculate_fit;

#[derive(Deserialize)]
struct CalcRequest {
    standard: String,
    components: Vec<Component>,
    mission_profile: crate::models::MissionProfile,
}

pub fn routes() -> Router {
    Router::new().route("/calculate", post(run_calculation))
}

async fn run_calculation(Json(req): Json<CalcRequest>) -> Json<Vec<(String, f64)>> {
    let results = req
        .components
        .iter()
        .map(|c| {
            (c.manufacturer_part_number.clone(), calculate_fit(&req.standard, c, &req.mission_profile))
        })
        .collect();

    Json(results)
}
