# Instructions to Verify the Resistor FIT Value (SN29500)

This guide explains how to verify the calculated FIT value for a resistor using the current implementation in `sn29500.rs`.

## 1. Understand the Equation
The FIT value is calculated as:

```
FIT = FIT_ref × PiT
```
Where:
- **FIT_ref**: Constant (0.3)
- **PiT**: Calculated using the dual activation energy model:

```
PiT = [A·exp(Ea1·z) + (1−A)·exp(Ea2·z)] / [A·exp(Ea1·zref) + (1−A)·exp(Ea2·zref)]
```
with:
- **A** = 0.873 (constant)
- **Ea1** = 0.16 (constant)
- **Ea2** = 0.44 (constant)
- **Theta_ref** = 40°C (constant)
- **Theta1** = 55°C (constant)
- **Theta2** = user-defined (from `profile.operating_temp`, default 40°C)
- **z** = 11605 × (1/Theta_ref(K) − 1/Theta2(K))
- **zref** = 11605 × (1/Theta_ref(K) − 1/Theta1(K))

## 2. Prepare the Input
- Set the operating temperature (Theta2) in your `MissionProfile` struct, e.g.:

```rust
let profile = MissionProfile {
    operating_temp: Some(50.0), // Theta2 in °C
    ..Default::default()
};
```

## 3. Call the Calculation
- Use the following call in your code:

```rust
let fit = calc_resistor(&component, &profile);
```

## 4. Manual Verification
- Calculate z and zref:
    - Convert all temperatures to Kelvin: T(K) = T(°C) + 273.0
    - Compute z and zref using the formulas above.
- Plug z and zref into the PiT formula.
- Multiply PiT by FIT_ref (0.3) to get the expected FIT.
- Compare this manual result to the value returned by the code.

## 5. Example
Suppose `operating_temp` (Theta2) is 50°C:
- Theta_ref = 40°C → 313 K
- Theta1 = 55°C → 328 K
- Theta2 = 50°C → 323 K

Calculate:
- z = 11605 × (1/313 − 1/323)
- zref = 11605 × (1/313 − 1/328)
- PiT = ... (use the formula)
- FIT = 0.3 × PiT

## 6. Troubleshooting
- Ensure all constants and formulas match the code.
- If the code and manual calculation differ, check for unit mismatches or typos.

---
For further help, see the code comments in `sn29500.rs` or contact the project maintainer.
