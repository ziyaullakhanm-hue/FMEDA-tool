use crate::models::Component;

pub mod sn29500;
mod IEC62380;
mod IEC61709;

pub fn calculate_fit(standard: &str, component: &Component, profile: &crate::models::MissionProfile) -> f64 {
    match standard {
        "SN29500" => sn29500::calc_fit(component, profile),
        "IEC62380" => IEC62380::calc_fit(component),
        // "IEC61709" => IEC61709::calc_fit(component), // Not implemented
        _ => 0.0, // Unknown standard
    }
}
