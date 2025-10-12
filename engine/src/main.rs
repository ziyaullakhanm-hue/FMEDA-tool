mod db;
mod models;
mod calc;
mod errors;
mod routes;

use db::init_db;
use tokio;

fn main() {
    // --- FMEDA Resistor FIT Verification Example ---
    use crate::models::{Component, MissionProfile};
    use crate::calc::sn29500::calc_resistor;
    use uuid::Uuid;
    use chrono::Utc;

    let component = Component {
        id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        manufacturer_part_number: "R-1234".to_string(),
        manufacturer: Some("TestManu".to_string()),
        reference_designator: Some("R1".to_string()),
        quantity: 1,
        created_at: Utc::now(),
        component_type: "Resistor".to_string(),
        base_fit: None,
        quality_factor: None,
        resistor_type: Some("metal film".to_string()),
    };
    let profile = MissionProfile {
        temperature_factor: None,
        environment_factor: None,
        stress_factor: None,
        reference_temp: None,
        operating_temp: Some(40.0), // Theta2 in °C, set to 40.0 to match thetaref
    };
    let fit = calc_resistor(&component, &profile);
    println!("[FMEDA] Calculated FIT for Resistor: {}", fit);
}
