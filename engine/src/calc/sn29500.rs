// Removed duplicate and unclosed function definition

pub fn calc_fit(component: &Component, profile: &MissionProfile) -> f64 {
    match component.component_type.as_str() {
        "Resistor" => calc_resistor(component, profile),
        "Capacitor" => calc_capacitor(component, profile),
        "IC" => calc_ic(component, profile),
        _ => 0.0,
    }
}

use crate::models::{Component, MissionProfile};

/// Calculates FIT for a resistor using the multi-activation energy model
pub fn calc_resistor(comp: &Component, profile: &MissionProfile) -> f64 {
    // --- 1️⃣ Use provided constants ---
    let ref_fit = 0.3; // FIT ref

    // --- 2️⃣ Temperature Setup ---
    let thetaref = 40.0; // Theta ref (°C, constant)
    let theta1 = 55.0;  // Theta 1 (°C, constant)
    let theta2 = profile.operating_temp.unwrap_or(40.0);  // Theta 2 (°C, user-defined)

    // --- 3️⃣ Calculate PiT using dual activation energy model with provided constants ---
    let pit = calc_pit_custom(thetaref, theta1, theta2);

    // --- 4️⃣ Final FIT (no quality factor) ---
    let fit = ref_fit * pit;

    fit
}

/// Calculates PiT with custom constants and temperatures
fn calc_pit_custom(thetaref: f64, theta1: f64, theta2: f64) -> f64 {
    // Constants
    let a = 0.873;        // weighting factor between two degradation mechanisms
    let ea1 = 0.16;       // activation energy 1 (eV)
    let ea2 = 0.44;       // activation energy 2 (eV)

    // Convert to Kelvin
    let turef = thetaref + 273.0;
    let t1 = theta1 + 273.0;
    let t2 = theta2 + 273.0;

    // Compute z and zref
    let z = 11605.0 * ((1.0 / turef) - (1.0 / t2));
    let zref = 11605.0 * ((1.0 / turef) - (1.0 / t1));

    println!("[FMEDA] z: {}", z);
    println!("[FMEDA] zref: {}", zref);

    // Apply the dual Arrhenius model
    let numerator = a * (ea1 * z).exp() + (1.0 - a) * (ea2 * z).exp();
    let denominator = a * (ea1 * zref).exp() + (1.0 - a) * (ea2 * zref).exp();

    numerator / denominator
}

/// Calculates PiT = (A·e^(Ea1·z) + (1−A)·e^(Ea2·z)) / (A·e^(Ea1·zref) + (1−A)·e^(Ea2·zref))
fn calc_pit(thetaref: f64, theta: f64) -> f64 {
    // Constants
    let a = 0.6;        // weighting factor between two degradation mechanisms
    let ea1 = 0.4;      // activation energy 1 (eV)
    let ea2 = 0.8;      // activation energy 2 (eV)

    // Convert to Kelvin
    let turef = thetaref + 273.0;
    let t1 = theta + 273.0;
    let t2 = theta + 273.0; // same as T1 for steady-state operation

    // Compute z and zref
    let z = 11605.0 * ((1.0 / turef) - (1.0 / t2));
    let zref = 11605.0 * ((1.0 / turef) - (1.0 / t1));

    // Apply the dual Arrhenius model
    let numerator = a * (ea1 * z).exp() + (1.0 - a) * (ea2 * z).exp();
    let denominator = a * (ea1 * zref).exp() + (1.0 - a) * (ea2 * zref).exp();

    numerator / denominator
}



fn calc_capacitor(comp: &Component, profile: &MissionProfile) -> f64 {
    let base_fit = comp.base_fit.unwrap_or(0.8);
    let env_factor = profile.environment_factor.unwrap_or(1.0);
    base_fit * env_factor
}

fn calc_ic(comp: &Component, profile: &MissionProfile) -> f64 {
    let base_fit = comp.base_fit.unwrap_or(5.0);
    let stress_factor = profile.stress_factor.unwrap_or(1.0);
    base_fit * stress_factor
}
