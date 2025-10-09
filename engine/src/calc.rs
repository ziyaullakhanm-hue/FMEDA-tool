use crate::models::{Component, FailureMode};
use serde_json::json;

/// Convert FIT to failures per hour
fn fit_to_per_hour(fit: f64) -> f64 {
    fit / 1_000_000_000.0
}

/// Simple per-component FMEDA calculation:
/// For a component, combine failure modes (sum of lambdas),
/// calculate detected failures based on detection_coverage.
pub fn calc_component_fmeda(component: &Component, modes: &[FailureMode]) -> serde_json::Value {
    // sum FIT for relevant modes
    let total_fit: f64 = modes.iter().map(|m| m.lambda).sum();
    let total_failure_per_hour = fit_to_per_hour(total_fit);

    // Weighted detection coverage: average of provided detection_coverage (if present) weighted by lambda
    let mut weighted_dc = 0.0;
    let mut total_lambda = 0.0;
    for m in modes {
        let lambda = m.lambda;
        let dc = m.detection_coverage.unwrap_or(0.0) as f64;
        weighted_dc += lambda * dc;
        total_lambda += lambda;
    }
    let detection_coverage = if total_lambda > 0.0 { weighted_dc / total_lambda } else { 0.0 };

    // Safe failure rate (detected * dc)
    let safe_per_hour = total_failure_per_hour * detection_coverage;
    let dangerous_per_hour = total_failure_per_hour - safe_per_hour;

    json!({
        "component_id": component.id,
        "mpn": component.manufacturer_part_number,
        "quantity": component.quantity,
        "total_fit": total_fit,
        "total_failure_per_hour": total_failure_per_hour,
        "detection_coverage": detection_coverage,
        "safe_per_hour": safe_per_hour,
        "dangerous_per_hour": dangerous_per_hour
    })
}

/// Calculate for whole BOM: aggregate by summing per-component failure/hour * quantity
pub fn calc_project_fmeda(components: &[Component], modes_map: &std::collections::HashMap<uuid::Uuid, Vec<FailureMode>>) -> serde_json::Value {
    use serde_json::Value;
    let mut comp_results: Vec<Value> = Vec::new();
    let mut total_dangerous_per_hour = 0.0_f64;
    let mut total_safe_per_hour = 0.0_f64;

    for comp in components {
        let modes = modes_map.get(&comp.id).cloned().unwrap_or_default();
        let comp_res = calc_component_fmeda(comp, &modes);
        let dangerous = comp_res["dangerous_per_hour"].as_f64().unwrap_or(0.0) * (comp.quantity as f64);
        let safe = comp_res["safe_per_hour"].as_f64().unwrap_or(0.0) * (comp.quantity as f64);
        total_dangerous_per_hour += dangerous;
        total_safe_per_hour += safe;
        comp_results.push(comp_res);
    }

    // Convert to FIT scale again for readability: per hour -> per 1e9 hours
    let total_dangerous_fit = total_dangerous_per_hour * 1_000_000_000.0;
    let total_safe_fit = total_safe_per_hour * 1_000_000_000.0;

    json!({
        "components": comp_results,
        "aggregate": {
            "total_dangerous_per_hour": total_dangerous_per_hour,
            "total_safe_per_hour": total_safe_per_hour,
            "total_dangerous_fit": total_dangerous_fit,
            "total_safe_fit": total_safe_fit,
            "dangerous_per_million_hours": total_dangerous_per_hour * 1_000_000.0
        }
    })
}
