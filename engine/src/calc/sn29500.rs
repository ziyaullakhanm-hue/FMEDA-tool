// use crate::models::{Component, MissionProfile};
// use serde_json::Value;
// use std::error::Error;

// /// Main entry: compute FIT value based on component type and mission profile
// pub fn calc_fit(comp: &Component, profile: &MissionProfile) -> f64 {
//     let comp_type = comp.component_type.to_lowercase();
//     match comp_type.as_str() {
//         "resistor" => calc_resistor_sn29500(profile),
//         "capacitor" => calc_capacitor_sn29500(profile),
//         "ic" => calc_ic_sn29500(profile),
//         _ => {
//             println!(
//                 "[WARN] Unknown component type '{}', using base lambda.",
//                 comp_type
//             );
//             get_base_lambda(&comp_type).unwrap_or(0.0)
//         }
//     }
// }

// /// Only used if the component type is unknown
// fn get_base_lambda(comp_type: &str) -> Result<f64, Box<dyn Error>> {
//     let lambda = match comp_type.to_lowercase().as_str() {
//         "resistor" => 0.3,
//         "capacitor" => 0.8,
//         "ic" => 5.0,
//         _ => 1.0, // fallback default
//     };
//     Ok(lambda)
// }

// /// SN29500 Resistor FIT calculation using weighted PiT
// fn calc_resistor_sn29500(profile: &MissionProfile) -> f64 {
//     let ref_fit = 0.3; // base FIT value per SN29500

//     // extract temperature–τ pairs from JSON
//     let segments = extract_temp_tau_pairs(profile.temp_tau_profile.as_ref());
//     if segments.is_empty() {
//         println!("[WARN] No temperature/τ segments found, returning ref_fit");
//         return ref_fit;
//     }

//     let total_tau: f64 = segments.iter().map(|(_, tau)| *tau).sum();
//     if total_tau == 0.0 {
//         println!("[WARN] All τ values are zero, returning ref_fit");
//         return ref_fit;
//     }

//     let thetaref = 40.0;
//     let theta1 = 55.0;//refFIT temperature
//     let mut weighted_pit_sum = 0.0;

//     println!("\n[SN29500] --- Temperature Segments ---");
//     for (temp, tau) in &segments {
//         let pit = calc_pit_custom(thetaref, theta1, *temp);
//         let weighted = pit * (*tau / total_tau);
//         weighted_pit_sum += weighted;
//         println!(
//             "Temp = {:>6.2} °C | τ = {:>7.4} | PiT = {:>7.6} | Weighted = {:>7.6}",
//             temp, tau, pit, weighted
//         );
//     }

//     let fit = ref_fit * weighted_pit_sum;
//     println!(
//         "[SN29500] ref_fit = {:.6} | ΣWeighted(PiT) = {:.6} → FIT = {:.6}",
//         ref_fit, weighted_pit_sum, fit
//     );
//     fit
// }

// /// Capacitor placeholder (can use same pattern later)
// fn calc_capacitor_sn29500(_profile: &MissionProfile) -> f64 {
//     0.8
// }

// /// IC placeholder (can use same pattern later)
// fn calc_ic_sn29500(_profile: &MissionProfile) -> f64 {
//     5.0
// }

// /// SN29500 PiT function with multiple activation energies
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

// /// Parse JSON mission profile into vector of (temperature, tau)
// fn extract_temp_tau_pairs(json_opt: Option<&Value>) -> Vec<(f64, f64)> {
//     let mut pairs = Vec::new();
//     let Some(json) = json_opt else { return pairs; };

//     if let Some(segs) = json.get("segments").and_then(|v| v.as_array()) {
//         for seg in segs {
//             if let (Some(t), Some(tau)) = (
//                 seg.get("temperature").and_then(|v| v.as_f64()),
//                 seg.get("tau").and_then(|v| v.as_f64()),
//             ) {
//                 pairs.push((t, tau));
//             }
//         }
//     } else if let (Some(temps), Some(taus)) =
//         (json.get("temperature").and_then(|v| v.as_array()),
//          json.get("tau").and_then(|v| v.as_array()))
//     {
//         let n = temps.len().min(taus.len());
//         for i in 0..n {
//             if let (Some(t), Some(tau)) = (temps[i].as_f64(), taus[i].as_f64()) {
//                 pairs.push((t, tau));
//             }
//         }
//     }

//     pairs
// }

use crate::models::{Component, ComponentVariant, MissionProfile};

pub fn calc_fit(
    comp: &Component,
    profile: &MissionProfile,
    variant_opt: Option<&ComponentVariant>,
) -> f64 {
    let comp_type = comp.component_type.to_lowercase();

    match comp_type.as_str() {
        "resistor" => calc_resistor_sn29500(comp, profile, variant_opt),
        "capacitor" => calc_capacitor_sn29500(comp, profile, variant_opt),
        "ic" => 5.0,
        _ => {
            println!("[WARN] Unknown type '{}', using base λ.", comp_type);
            get_base_lambda(&comp_type)
        }
    }
}

fn calc_pit_weighted(
    comp_type: &str,
    profile: &MissionProfile,
    variant: &ComponentVariant,
    thetaref_const: f64,
) -> f64 {
    let segments = profile.parse_temp_tau_profile();
    if segments.is_empty() { return 1.0; }

    let total_tau: f64 = segments.iter().map(|(_, tau)| *tau).sum();
    if total_tau <= 0.0 { return 1.0; }

    let a = variant.a.unwrap_or(0.873);
    let ea1 = variant.ea1.unwrap_or(0.16);
    let ea2 = variant.ea2.unwrap_or(0.44);
    let theta_ref_fit = variant.ref_temp.unwrap_or(40.0);

    let mut weighted_pit_sum = 0.0;
    println!("\n[SN29500 - {}] --- Temp Segments ---", comp_type);
    println!("Total on-time (Στ) = {:.6}", total_tau);
    println!("Thermal constants: a = {:.6}, Ea1 = {:.6}, Ea2 = {:.6}, theta_ref_fit = {:.3}°C", a, ea1, ea2, theta_ref_fit);

    for (temp, tau) in &segments {
        let pit = calc_pit_custom(thetaref_const, theta_ref_fit, *temp, a, ea1, ea2);
        let weight = *tau / total_tau;
        let weighted = pit * weight;
        weighted_pit_sum += weighted;
        println!("Temp={:>6.1}°C | τ={:>8.6} | PiT={:>8.6} | weight={:>7.6} | weighted={:>8.6}",
            temp, tau, pit, weight, weighted);
    }
    println!("ΣWeighted(PiT) = {:.6}", weighted_pit_sum);
    weighted_pit_sum
}

fn calc_capacitor_sn29500(
    comp: &Component,
    profile: &MissionProfile,
    variant_opt: Option<&ComponentVariant>,
) -> f64 {
    let variant = match variant_opt {
        Some(v) => v,
        None => return 0.8,
    };

    let pit = calc_pit_weighted("Capacitor", profile, variant, 40.0);
    
    let c2 = variant.c2.unwrap_or(1.0);
    let c3 = variant.c3.unwrap_or(1.0);
    let u = comp.operating_voltage.unwrap_or(5.0);
    let umax = comp.rated_voltage.unwrap_or(10.0);
    let uref = variant.uref_umax_ratio.unwrap_or(0.5) * umax;
    // PiU (voltage) calculation — print all intermediate values
    println!("\n[SN29500 - Capacitor] Voltage factors:");
    println!("Operating U = {:.6} V", u);
    println!("Rated Umax = {:.6} V", umax);
    println!("Uref (ratio {:.6}) = {:.6} V", variant.uref_umax_ratio.unwrap_or(0.5), uref);
    println!("c2 = {:.6}, c3 = {:.6}", c2, c3);

    let piu = if umax > 0.0 && u > 0.0 {
        let u_over_umax = (u / umax).powf(c2);
        let uref_over_umax = (uref / umax).powf(c2);
        let exponent = c3 * (u_over_umax - uref_over_umax);
        println!("(U/Umax)^c2 = {:.6}", u_over_umax);
        println!("(Uref/Umax)^c2 = {:.6}", uref_over_umax);
        println!("PiU exponent = {:.6}", exponent);
        let val = exponent.exp();
        println!("PiU = exp(exponent) = {:.6}", val);
        val
    } else {
        println!("[WARN] Voltage inputs invalid, defaulting PiU=1.0");
        1.0
    };

    let piq = variant.pi_q.unwrap_or(1.0);
    let fit = variant.ref_fit * pit * piu * piq;

    println!("\n[SN29500 - Capacitor] Summary:");
    println!("ref_fit = {:.6}", variant.ref_fit);
    println!("ΣWeighted(PiT) = {:.6}", pit);
    println!("PiU = {:.6}", piu);
    println!("PiQ = {:.6}", piq);
    println!("Final FIT = ref_fit * ΣWeighted(PiT) * PiU * PiQ = {:.6} * {:.6} * {:.6} * {:.6} = {:.6}",
        variant.ref_fit, pit, piu, piq, fit);
    fit
}

// Renamed comp to _comp to fix warning
fn calc_resistor_sn29500(_comp: &Component, profile: &MissionProfile, variant_opt: Option<&ComponentVariant>) -> f64 {
    let variant = match variant_opt {
        Some(v) => v,
        None => return 0.3,
    };
    let pit = calc_pit_weighted("Resistor", profile, variant, 40.0);
    let fit = variant.ref_fit * pit;
    println!(">>> Final FIT: {:.6}", fit);
    fit
}

fn calc_pit_custom(thetaref: f64, theta_ref_fit: f64, theta_mission: f64, a: f64, ea1: f64, ea2: f64) -> f64 {
    let turef = thetaref + 273.0; 
    let tref_fit = theta_ref_fit + 273.0;
    let tmission = theta_mission + 273.0;

    let z = 11605.0 * ((1.0 / turef) - (1.0 / tmission));
    let zref = 11605.0 * ((1.0 / turef) - (1.0 / tref_fit));

    let num = a * (ea1 * z).exp() + (1.0 - a) * (ea2 * z).exp();
    let den = a * (ea1 * zref).exp() + (1.0 - a) * (ea2 * zref).exp();

    if den == 0.0 { 1.0 } else { num / den }
}

fn get_base_lambda(comp_type: &str) -> f64 {
    match comp_type {
        "resistor" => 0.3,
        "capacitor" => 0.8,
        _ => 1.0,
    }
}