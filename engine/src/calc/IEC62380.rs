use crate::models::Component;

pub fn calc_fit(component: &Component) -> f64 {
    match component.component_type.as_str() {
        "Resistor" => calc_resistor(component),
        "Capacitor" => calc_capacitor(component),
        "IC" => calc_ic(component),
        _ => 0.0,
    }
}

fn calc_resistor(comp: &Component) -> f64 {
    // IEC 62380 uses environment-dependent model
    // λ = C1 * πT * πU
    let c1 = 4.0;
    let temp_factor = 1.2;
    let usage_factor = 1.1;
    c1 * temp_factor * usage_factor
}

fn calc_capacitor(comp: &Component) -> f64 {
    3.0 * 1.1 * 1.05
}

fn calc_ic(comp: &Component) -> f64 {
    6.0 * 1.3 * 1.2
}
