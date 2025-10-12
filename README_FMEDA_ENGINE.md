# FMEDA Engine Verification & Validation Guide

This guide explains how to verify and validate the FMEDA prediction engine in this project.

## 1. Prerequisites
- Rust toolchain installed
- Node.js (for frontend, if needed)
- All dependencies installed (`cargo build` for Rust, `npm install` for frontend)

## 2. Project Structure
- `engine/src/models.rs`: Data models for components, mission profiles, failure modes, and calculations.
- `engine/src/calc/sn29500.rs`: Calculation logic for different component types.

## 3. How to Update Equations
- Edit the functions in `engine/src/calc/sn29500.rs` (e.g., `calc_resistor`, `calc_capacitor`, `calc_ic`) to update the failure rate equations for each component type.
- Use fields from `Component` and `MissionProfile` to make equations flexible and user-driven.

## 4. How to Use a Single Mission Profile for Verification
- Define a `MissionProfile` instance with your desired values (temperature, environment, stress, etc.).
- Pass this instance to all calculation functions for all components you want to verify.

**Example:**
```rust
let profile = MissionProfile {
    temperature_factor: Some(1.2),
    environment_factor: Some(1.1),
    stress_factor: Some(1.0),
};
let fit = calc_fit(&component, &profile);
```

## 5. How to Validate
- Write unit tests for your calculation functions in Rust (see `tests/` or add new test files).
- Compare calculated FIT and failure mode results with reference data or hand calculations.
- Use the `predict_fmeda` function to get a full FMEDA prediction for a component and mission profile.

## 6. Running the Engine
- Build the backend: `cd engine && cargo build`
- Run tests: `cargo test`
- (Optional) Run the backend: `cargo run`

## 7. Troubleshooting
- Ensure all required fields in `Component` and `MissionProfile` are set.
- Check for errors in calculation logic if results are unexpected.

---
For further help, see the code comments or contact the project maintainer.
