// pub mod sn29500;
// pub mod IEC62380;

// use crate::models::{Component, MissionProfile};
// use std::error::Error;

// /// Generic interface for selecting reliability standards
// pub fn calculate_fit(
//     standard: &str,
//     component: &Component,
//     profile: &MissionProfile,
// ) -> Result<f64, Box<dyn std::error::Error>> {
//     match standard {
//         "SN29500" => Ok(sn29500::calc_fit(component, profile)), // expects 2 args
//         "IEC62380" => Ok(IEC62380::calc_fit(component)),        // expects 1 arg
//         _ => Err(format!("Unknown standard: {}", standard).into()),
//     }
// }
pub mod sn29500;
// pub mod IEC62380; // Uncomment if you have this file

use crate::models::{Component, ComponentVariant, MissionProfile};
use std::error::Error;

/// Generic interface to calculate FIT based on reliability standard
pub fn calculate_fit(
    standard: &str,
    component: &Component,
    profile: &MissionProfile,
    variant_opt: Option<&ComponentVariant>,
) -> Result<f64, Box<dyn Error>> {
    match standard {
        "SN29500" => {
            if variant_opt.is_none() {
                // Warning only - calculation might proceed with base values
            }
            Ok(sn29500::calc_fit(component, profile, variant_opt))
        }
        "IEC62380" => {
            // IEC62380 logic would go here
            Ok(10.0) // placeholder
        }
        _ => Err(format!("Unknown standard: {}", standard).into()),
    }
}
