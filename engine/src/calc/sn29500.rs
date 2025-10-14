// use crate::models::{Component, MissionProfile};
// use bigdecimal::ToPrimitive;

// /// Main entry point
// pub fn calc_fit(component: &Component, profile: &MissionProfile) -> f64 {
//     match component.component_type.as_str() {
//         "Resistor" => calc_resistor(component, profile),
//         "Capacitor" => calc_capacitor(component, profile),
//         "IC" => calc_ic(component, profile),
//         _ => 0.0,
//     }
// }

// /// Calculates FIT for a resistor using the multi-activation energy model 
// pub fn calc_resistor(comp: &Component, profile: &MissionProfile) -> f64 {
//     let ref_fit = 0.3;

//     let thetaref = 40.0;
//     let theta1 = 55.0;

//     if let Some(temp_tau_json) = &profile.temp_tau_profile {
//         // Parse JSON array of arrays: [[temp, tau], ...]
//         let temp_tau_vec: Vec<(f64, f64)> = serde_json::from_value(temp_tau_json.clone())
//             .unwrap_or_default();

//         let on_time: f64 = temp_tau_vec.iter().map(|(_, tau)| *tau).sum();
//         let mut pit_sum = 0.0;
//         for (temp, tau) in temp_tau_vec {
//             let pit = calc_pit_custom(thetaref, theta1, temp);
//             pit_sum += pit * (tau / on_time);
//         }
//         ref_fit * pit_sum
//     } else {
//         let theta2 = profile.operating_temp.unwrap_or(40.0);
//         let pit = calc_pit_custom(thetaref, theta1, theta2);
//         ref_fit * pit
//     }
// }

// /// Calculates PiT with custom constants and temperatures
// fn calc_pit_custom(thetaref: f64, theta1: f64, theta2: f64) -> f64 {
//     let a = 0.873;
//     let ea1 = 0.16;
//     let ea2 = 0.44;

//     let turef = thetaref + 273.0;
//     let t1 = theta1 + 273.0;
//     let t2 = theta2 + 273.0;

//     let z = 11605.0 * ((1.0 / turef) - (1.0 / t2));
//     let zref = 11605.0 * ((1.0 / turef) - (1.0 / t1));

//     let numerator = a * (ea1 * z).exp() + (1.0 - a) * (ea2 * z).exp();
//     let denominator = a * (ea1 * zref).exp() + (1.0 - a) * (ea2 * zref).exp();

//     numerator / denominator
// }

// fn calc_capacitor(comp: &Component, profile: &MissionProfile) -> f64 {
//     let base_fit = comp.base_fit
//         .as_ref()
//         .and_then(|b| b.to_f64())
//         .unwrap_or(0.8);

//     let env_factor = profile.environment_factor.unwrap_or(1.0);
//     base_fit * env_factor
// }

// fn calc_ic(comp: &Component, profile: &MissionProfile) -> f64 {
//     let base_fit = comp.base_fit
//         .as_ref()
//         .and_then(|b| b.to_f64())
//         .unwrap_or(5.0);

//     let stress_factor = profile.stress_factor.unwrap_or(1.0);
//     base_fit * stress_factor
// }
use crate::models::{Component, MissionProfile};
use serde_json::Value;
use std::error::Error;

/// Main entry: compute FIT value based on component type and mission profile
pub fn calc_fit(comp: &Component, profile: &MissionProfile) -> f64 {
    let comp_type = comp.component_type.to_lowercase();
    match comp_type.as_str() {
        "resistor" => calc_resistor_sn29500(profile),
        "capacitor" => calc_capacitor_sn29500(profile),
        "ic" => calc_ic_sn29500(profile),
        _ => {
            println!(
                "[WARN] Unknown component type '{}', using base lambda.",
                comp_type
            );
            get_base_lambda(&comp_type).unwrap_or(0.0)
        }
    }
}

/// Only used if the component type is unknown
fn get_base_lambda(comp_type: &str) -> Result<f64, Box<dyn Error>> {
    let lambda = match comp_type.to_lowercase().as_str() {
        "resistor" => 0.3,
        "capacitor" => 0.8,
        "ic" => 5.0,
        _ => 1.0, // fallback default
    };
    Ok(lambda)
}

/// SN29500 Resistor FIT calculation using weighted PiT
fn calc_resistor_sn29500(profile: &MissionProfile) -> f64 {
    let ref_fit = 0.3; // base FIT value per SN29500

    // extract temperature–τ pairs from JSON
    let segments = extract_temp_tau_pairs(profile.temp_tau_profile.as_ref());
    if segments.is_empty() {
        println!("[WARN] No temperature/τ segments found, returning ref_fit");
        return ref_fit;
    }

    let total_tau: f64 = segments.iter().map(|(_, tau)| *tau).sum();
    if total_tau == 0.0 {
        println!("[WARN] All τ values are zero, returning ref_fit");
        return ref_fit;
    }

    let thetaref = 40.0;
    let theta1 = 55.0;
    let mut weighted_pit_sum = 0.0;

    println!("\n[SN29500] --- Temperature Segments ---");
    for (temp, tau) in &segments {
        let pit = calc_pit_custom(thetaref, theta1, *temp);
        let weighted = pit * (*tau / total_tau);
        weighted_pit_sum += weighted;
        println!(
            "Temp = {:>6.2} °C | τ = {:>7.4} | PiT = {:>7.6} | Weighted = {:>7.6}",
            temp, tau, pit, weighted
        );
    }

    let fit = ref_fit * weighted_pit_sum;
    println!(
        "[SN29500] ref_fit = {:.6} | ΣWeighted(PiT) = {:.6} → FIT = {:.6}",
        ref_fit, weighted_pit_sum, fit
    );
    fit
}

/// Capacitor placeholder (can use same pattern later)
fn calc_capacitor_sn29500(_profile: &MissionProfile) -> f64 {
    0.8
}

/// IC placeholder (can use same pattern later)
fn calc_ic_sn29500(_profile: &MissionProfile) -> f64 {
    5.0
}

/// SN29500 PiT function with multiple activation energies
fn calc_pit_custom(thetaref: f64, theta1: f64, theta2: f64) -> f64 {
    let a = 0.873;
    let ea1 = 0.16;
    let ea2 = 0.44;
    let turef = thetaref + 273.0;
    let t1 = theta1 + 273.0;
    let t2 = theta2 + 273.0;

    let z = 11605.0 * ((1.0 / turef) - (1.0 / t2));
    let zref = 11605.0 * ((1.0 / turef) - (1.0 / t1));

    let numerator = a * (ea1 * z).exp() + (1.0 - a) * (ea2 * z).exp();
    let denominator = a * (ea1 * zref).exp() + (1.0 - a) * (ea2 * zref).exp();

    numerator / denominator
}

/// Parse JSON mission profile into vector of (temperature, tau)
fn extract_temp_tau_pairs(json_opt: Option<&Value>) -> Vec<(f64, f64)> {
    let mut pairs = Vec::new();
    let Some(json) = json_opt else { return pairs; };

    if let Some(segs) = json.get("segments").and_then(|v| v.as_array()) {
        for seg in segs {
            if let (Some(t), Some(tau)) = (
                seg.get("temperature").and_then(|v| v.as_f64()),
                seg.get("tau").and_then(|v| v.as_f64()),
            ) {
                pairs.push((t, tau));
            }
        }
    } else if let (Some(temps), Some(taus)) =
        (json.get("temperature").and_then(|v| v.as_array()),
         json.get("tau").and_then(|v| v.as_array()))
    {
        let n = temps.len().min(taus.len());
        for i in 0..n {
            if let (Some(t), Some(tau)) = (temps[i].as_f64(), taus[i].as_f64()) {
                pairs.push((t, tau));
            }
        }
    }

    pairs
}
