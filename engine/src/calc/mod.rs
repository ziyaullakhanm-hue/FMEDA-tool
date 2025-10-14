// use crate::models::Component;

// pub mod sn29500;
// mod IEC62380;
// mod IEC61709;

// pub fn calculate_fit(standard: &str, component: &Component, profile: &crate::models::MissionProfile) -> f64 {
//     match standard {
//         "SN29500" => sn29500::calc_fit(component, profile),
//         "IEC62380" => IEC62380::calc_fit(component),
//         // "IEC61709" => IEC61709::calc_fit(component), // Not implemented
//         _ => 0.0, // Unknown standard
//     }
// }

pub mod sn29500;
pub mod IEC62380;

use crate::models::{Component, MissionProfile};
use std::error::Error;

/// Generic interface for selecting reliability standards
pub fn calculate_fit(
    standard: &str,
    component: &Component,
    profile: &MissionProfile,
) -> Result<f64, Box<dyn std::error::Error>> {
    match standard {
        "SN29500" => Ok(sn29500::calc_fit(component, profile)), // expects 2 args
        "IEC62380" => Ok(IEC62380::calc_fit(component)),        // expects 1 arg
        _ => Err(format!("Unknown standard: {}", standard).into()),
    }
}
