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
```markdown
# Verifying Resistor & Capacitor FIT (SN29500)

This document explains how to verify FIT calculations produced by the engine (SN29500 implementation in `engine/src/calc/sn29500.rs`). It covers both resistors and capacitors and describes how to reproduce the values printed by the interactive CLI.

Summary of what the code now prints during a run (useful for verification):
- per-segment temperature (`T`) and on-time (`τ`)
- per-segment PiT, weight (= τ / Στ) and weighted contribution
- ΣWeighted(PiT)
- capacitor voltage factors: `U`, `Umax`, `Uref`, `(U/Umax)^C2`, `(Uref/Umax)^C2`, exponent and `PiU`
- `ref_fit`, `PiT`, `PiU`, `PiQ` and the final FIT (explicit multiplication)

---

## 1) How to run the interactive verifier

1. Ensure `DATABASE_URL` is set to your Postgres database (or omit it to run the local demo fallback):

```powershell
$env:DATABASE_URL = 'postgres://dbuser:dbpass@db-host:5432/dbname'
cd 'c:\Users\a1094300\Documents\FMEDA_TOOL\SafeCrate\engine'
cargo run
```

2. Follow the prompts to select component type, subtype and variant. If your DB has no matching component rows the program will compute sample FITs for the selected variant(s).

---

## 2) Resistor verification (brief)

Equation used (per code):

PiT = [A·exp(Ea1·z) + (1−A)·exp(Ea2·z)] / [A·exp(Ea1·z_ref) + (1−A)·exp(Ea2·z_ref)]

where z = 11605 × (1/T_ref(K) − 1/T_mission(K)) and z_ref = 11605 × (1/T_ref(K) − 1/T_ref_fit(K)).

FIT = ref_fit × ΣWeighted(PiT) (where ΣWeighted(PiT) is the mission-profile-weighted sum of per-segment PiT values).

To verify:
- Convert temperatures (°C) to Kelvin.
- Compute z and z_ref for each segment.
- Compute PiT per segment, compute weight = τ / Στ, then ΣWeighted(PiT).
- Multiply by `ref_fit` (printed by code as `ref_fit`).

The interactive run prints all per-segment PiT and weighted terms so you can compare step-by-step.

---

## 3) Capacitor verification (detailed)

Capacitor FIT = ref_fit × ΣWeighted(PiT) × PiU × PiQ

- ΣWeighted(PiT) is computed the same way as resistors using the `a`, `ea1`, `ea2` constants stored for the variant.
- PiU (voltage factor) uses the SN29500 exponential form:

    PiU = exp(C3 × ((U / Umax)^C2 − (Uref / Umax)^C2))

    where `U` = operating voltage, `Umax` = rated voltage, `Uref = uref_umax_ratio × Umax`.

- PiQ is the quality factor (`pi_q`) taken from the variant row.

Important notes:
- If `U == Uref` then PiU = 1.0 (because the exponent is zero). This is the reason PiU was 1.0 in your example.
- The final FIT depends strongly on the `a`, `ea1`, `ea2` values stored for the variant. If you expect a specific final FIT (for example ~4.348139) ensure the variant constants match the values you provided when computing manually (A=0.999, Ea1=0.5, Ea2=1.59, C2=1.29, C3=4.0, uref_umax_ratio=0.5).

Example (from your verification):
- Mission segments: [-40°C:0.0037, 23°C:0.0122, 50°C:0.0396, 100°C:0.0049, 105°C:0.0006]
- Στ = 0.0610
- Using A=0.999, Ea1=0.5, Ea2=1.59 produced ΣWeighted(PiT) ≈ 4.349
- With U=5 V, Umax=10 V, Uref = 0.5×10 = 5 V → PiU = 1.0
- ref_fit = 1.0, pi_q = 1.0
- Final FIT ≈ 1.0 × 4.349 × 1.0 × 1.0 ≈ 4.349

The program now prints per-segment PiT, ΣWeighted(PiT), PiU intermediate values and the final multiplication so you can confirm each term.

---

## 4) Where the constants come from (DB)

- `a`, `ea1`, `ea2`, `c2`, `c3`, `uref_umax_ratio`, `pi_q`, `ref_fit`, and `ref_temp` are loaded from the `component_variants` table at runtime. The code uses these values when a variant is selected.
- If your manual check uses different constants, update the `component_variants` row in the DB to match. Example SQL to update a variant (replace UUID):

```sql
UPDATE component_variants
SET a = 0.999, ea1 = 0.5, ea2 = 1.59, c2 = 1.29, c3 = 4.0, uref_umax_ratio = 0.5, pi_q = 1.0
WHERE id = 'your-variant-uuid';
```

After updating, re-run the interactive CLI and the printed intermediate values should match your manual calculations.

---

## 5) Debugging tips

- The code prints detailed debug lines from `engine/src/calc/sn29500.rs`. Look for:
    - `[SN29500 - Capacitor] --- Temp Segments ---` (per-segment PiT details)
    - `Voltage factors and PiU computation` (U, Umax, Uref, (U/Umax)^C2, exponent, PiU)
    - `Summary of calculation values` (ref_fit, ΣWeighted(PiT), PiU, PiQ, Final FIT)

- If results differ from expectation:
    1. Confirm the variant DB row contains the constants you expect.
    2. Confirm component `operating_voltage` and `rated_voltage` are populated (interactive CLI prompts if missing).
    3. Re-run to see printed per-segment PiT and PiU debug lines and compare to manual math.

---

If you want, I can:
- Add a one-line CSV output option with all intermediate fields for easier spreadsheet verification.
- Add a unit test that computes the exact sample case and asserts the expected FIT.

Tell me which of those you'd like and I'll add it.

```
