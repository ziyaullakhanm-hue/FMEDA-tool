# FMEDA Component Calculation Design Guide

This guide explains how to structure and implement calculation logic for each component type in the FMEDA engine, ensuring flexibility for user-defined, constant, and formula-based values.

## 1. Component Struct Design
- Each component struct (e.g., `Component`) should include:
  - User-defined fields (e.g., `base_fit`, `quality_factor`, `resistor_type`)
  - Fields for subtypes (e.g., `resistor_type`, `capacitor_type`)
  - Optional fields for values that may be set by the user or calculated

## 2. Calculation Function Pattern
- Each calculation function (e.g., `calc_resistor`) should:
  - Use user-defined values from the struct if provided
  - Use constants for standard values (e.g., default base FITs for subtypes)
  - Apply formulas as required, combining user input, constants, and mission profile factors

### Example: Resistor Calculation
```rust
fn calc_resistor(comp: &Component, profile: &MissionProfile) -> f64 {
  // 1. Constant values (default base FITs)
  let base_fit = match comp.resistor_type.as_deref() {
    Some("carbon film") => 0.4, // constant
    Some("metal film") => 0.3,  // constant
    // ... other types ...
    _ => comp.base_fit.unwrap_or(0.5), // user-defined or fallback
  };
  // 2. User-defined values
  let quality_factor = comp.quality_factor.unwrap_or(1.0); // user-defined or default
  // 3. User-defined reference and operating temperature
  let thetaref = profile.reference_temp.unwrap_or(40.0); // user-defined reference temp
  let theta = profile.operating_temp.unwrap_or(40.0);    // user-defined operating temp
  // 4. Formula-based values (example: PiT)
  let pit = calc_pit(thetaref, theta);
  // Final calculation (formula)
  base_fit * quality_factor * pit
}
```

## 3. User Input
- Allow users to select subtypes (e.g., resistor type) and enter custom values where needed.
- If a value is not provided by the user, use the constant or default in the calculation.

## 4. Extending to Other Components
- Follow the same pattern for capacitors, ICs, etc.:
  - Add a subtype field if needed (e.g., `capacitor_type`)
  - Update the calculation function to use subtype, user input, constants, and formulas

## 5. Mission Profile
- Use a single `MissionProfile` instance for all calculations to ensure consistent environmental and operational factors.

## 6. Summary
- This approach ensures all component calculations are flexible, user-driven, and maintainable.
- Update the calculation logic as standards or requirements evolve.

---
For further help, see the code comments or contact the project maintainer.
