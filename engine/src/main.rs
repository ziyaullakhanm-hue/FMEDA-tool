// mod db;
// mod models;
// mod calc;

// use db::init_db;
// use models::{Component, MissionProfile};
// // use calc::sn29500::calc_fit;
// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // --- 1️⃣ Connect to DB ---
//     let pool = init_db().await?;
//     println!("[DB TEST] ✅ Connected to database.\n");

//     // --- 2️⃣ Fetch mission profile ---
//     let profiles: Vec<MissionProfile> = sqlx::query_as::<_, MissionProfile>(
//         "SELECT * FROM mission_profiles ORDER BY created_at DESC LIMIT 1"
//     )
//     .fetch_all(&pool)
//     .await?;

//     if profiles.is_empty() {
//         println!("⚠️ No mission profiles found in DB.");
//         return Ok(());
//     }

//     let profile = &profiles[0];
//     println!("[Mission Profile Loaded]");
//     println!("Name: {}", profile.name);
//     println!("Description: {}", profile.description.as_deref().unwrap_or("None"));
//     println!("Temp/Tau Profile: {:#?}\n", profile.temp_tau_profile);

//     // --- 3️⃣ Fetch components ---
//     let components: Vec<Component> = sqlx::query_as::<_, Component>(
//         "SELECT * FROM components"
//     )
//     .fetch_all(&pool)
//     .await?;

//     if components.is_empty() {
//         println!("⚠️ No components found in DB.");
//         return Ok(());
//     }

//     println!("[Component Reliability Results]");
//     println!("{:<30} | {:<15} | {:<10}", "Manufacturer P/N", "Type", "FIT (Failures/1e9h)");
//     println!("{}", "-".repeat(70));

// // --- 4️⃣ Compute FIT for each component ---
// for comp in &components {
//     match crate::calc::calculate_fit("SN29500", comp, profile) {
//         Ok(fit) => println!(
//             "{:<30} | {:<15} | {:<10.6}",
//             comp.manufacturer_part_number,
//             comp.component_type,
//             fit
//         ),
//         Err(e) => println!(
//             "{:<30} | {:<15} | Error: {}",
//             comp.manufacturer_part_number,
//             comp.component_type,
//             e
//         ),
//     }
// }


//     println!("\n✅ FMEDA FIT computation complete.");
//     Ok(())
// }
mod db;
mod models;
mod calc;

use db::init_db;
use models::{Component, ComponentVariant, MissionProfile, SubtypeLookup};
use std::error::Error;
use std::io::{self, Write};
use itertools::Itertools; 
use crate::calc::calculate_fit;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- 1️⃣ Connect to DB ---
    let pool = match init_db().await {
        Ok(p) => {
            println!("[DB TEST] ✅ Connected to database.\n");
            p
        },
        Err(e) => {
            println!("[DB WARN] Could not connect to DB: {}", e);
            println!("Falling back to local demo mode (no DB).\n");

            // Create a demo mission profile (temp/tau segments)
            let demo_profile = MissionProfile {
                id: Uuid::new_v4(),
                name: "DemoProfile".to_string(),
                description: Some("Local demo mission profile".to_string()),
                temp_tau_profile: json!({
                    "segments": [
                        {"temperature": -40.0, "tau": 0.0037},
                        {"temperature": 23.0, "tau": 0.0122},
                        {"temperature": 50.0, "tau": 0.0396},
                        {"temperature": 100.0, "tau": 0.0049},
                        {"temperature": 105.0, "tau": 0.0006}
                    ]
                }),
                created_at: Utc::now(),
                reference_temp: Some(40.0),
                operating_temp: Some(23.0),
            };

            // Example capacitor variant (from your DB snapshot)
            let variant = ComponentVariant {
                id: Uuid::parse_str("7de86a2a-47b1-4be5-bbb3-0e59b01cac47").unwrap_or(Uuid::new_v4()),
                subtype_id: Uuid::new_v4(),
                name: "Polycarbonate (demo)".to_string(),
                ref_fit: 1.0,
                ref_temp: Some(40.0),
                a: Some(0.998),
                ea1: Some(0.57),
                ea2: Some(1.63),
                c2: Some(1.5),
                c3: Some(4.56),
                uref_umax_ratio: Some(0.5),
                pi_q: Some(1.0),
                notes: Some("Demo variant".to_string()),
                created_at: Utc::now(),
            };

            // Sample component
            let sample_comp = Component {
                id: Uuid::new_v4(),
                project_id: Uuid::new_v4(),
                manufacturer_part_number: "SAMPLE-PC".to_string(),
                manufacturer: None,
                reference_designator: None,
                quantity: 1,
                created_at: Utc::now(),
                component_type: "Capacitor".to_string(),
                base_fit: None,
                quality_factor: None,
                resistor_type: None,
                mission_profile_id: None,
                subtype_id: Some(variant.subtype_id),
                variant_id: Some(variant.id),
                operating_voltage: Some(5.0),
                rated_voltage: Some(10.0),
            };

            println!("Demo: computing FIT for sample capacitor variant '{}'...", variant.name);
            match calculate_fit("SN29500", &sample_comp, &demo_profile, Some(&variant)) {
                Ok(fit) => println!("SAMPLE | Capacitor | {} | FIT = {:.4}", variant.name, fit),
                Err(err) => println!("Demo calculation failed: {}", err),
            }

            println!("Demo run complete. Set a valid DATABASE_URL to connect to your DB and run full calculations.");
            return Ok(());
        }
    };

    // --- 2️⃣ Fetch Mission Profile ---
    let profiles = sqlx::query_as::<_, MissionProfile>("SELECT * FROM mission_profiles")
        .fetch_all(&pool).await?;

    if profiles.is_empty() {
        println!("⚠️ No mission profiles found.");
        return Ok(());
    }
    let profile = &profiles[0];
    println!("[Mission Profile Loaded]: {}", profile.name);

    // --- 3️⃣ Fetch Components ---
    let mut components: Vec<Component> = sqlx::query_as::<_, Component>("SELECT * FROM components")
        .fetch_all(&pool).await?;

    if components.is_empty() {
        println!("⚠️ No components found in the database.");
        return Ok(());
    }

    // --- 4️⃣ Interactive Component Type Selection ---
    let mut component_types: Vec<String> = components.iter()
        .map(|c| c.component_type.clone())
        .unique()
        .collect();

    for default in &["Resistor", "Capacitor", "IC"] {
        if !component_types.iter().any(|t| t.eq_ignore_ascii_case(default)) {
            component_types.push(default.to_string());
        }
    }

    println!("\nSelect Component Type:");
    for (i, t) in component_types.iter().enumerate() { println!("{}: {}", i + 1, t); }

    print!("Enter number(s), separated by comma: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let selected_types: Vec<String> = input.trim().split(',')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .filter_map(|i| component_types.get(i - 1).cloned())
        .collect();

    if selected_types.is_empty() { return Ok(()); }

    // --- 5️⃣ Fetch Subtypes ---
    let mut subtypes = Vec::new();
    for stype in &selected_types {
        let st_rows: Vec<SubtypeLookup> = sqlx::query_as(
            "SELECT s.id, s.name FROM component_subtypes s 
             JOIN component_types t ON s.type_id = t.id 
             WHERE t.name ILIKE $1"
        )
        .bind(stype)
        .fetch_all(&pool).await?;
        subtypes.extend(st_rows);
    }

    if subtypes.is_empty() {
        println!("⚠️ No subtypes found.");
        return Ok(());
    }

    println!("\nSelect Subtype(s):");
    for (i, s) in subtypes.iter().enumerate() { println!("{}: {}", i + 1, s.name); }
    
    print!("Enter number(s): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    
    let selected_subtype_ids: Vec<uuid::Uuid> = input.trim().split(',')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .filter_map(|i| subtypes.get(i - 1).map(|s| s.id))
        .collect();

    // --- 6️⃣ Fetch Variants ---
    let mut variants: Vec<ComponentVariant> = Vec::new();
    for sid in &selected_subtype_ids {
        let vars = sqlx::query_as::<_, ComponentVariant>("SELECT * FROM component_variants WHERE subtype_id = $1")
            .bind(sid).fetch_all(&pool).await?;
        variants.extend(vars);
    }

    println!("\nSelect Variant(s):");
    for (i, v) in variants.iter().enumerate() { println!("{}: {}", i + 1, v.name); }
    
    print!("Enter number(s): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    
    let selected_variant_indices: Vec<usize> = input.trim().split(',')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .map(|i| i - 1)
        .collect();

    // --- 7️⃣ Compute FIT ---
    println!("\n{:<20} | {:<10} | {:<15} | {:<10}", "P/N", "Type", "Variant", "FIT");
    println!("{}", "-".repeat(65));

    let mut processed = 0usize;

    for c in &mut components {
        // Match by type first
        let type_match = selected_types.iter().any(|t| t.eq_ignore_ascii_case(&c.component_type));
        if !type_match { continue; }

        for &v_idx in &selected_variant_indices {
            let variant = &variants[v_idx];
            
            // If the component already has a different variant linked in DB, skip it.
            // If it has NO variant linked (NULL), we let the user selection apply.
            if let Some(own_vid) = c.variant_id {
                if own_vid != variant.id { continue; }
            }

            // Capacitor Voltage Input Handling
            if c.component_type.to_lowercase() == "capacitor" {
                if c.operating_voltage.is_none() || c.rated_voltage.is_none() {
                    println!("\n[INPUT NEEDED] Capacitor: {}", c.manufacturer_part_number);
                    print!("  > Enter Operating Voltage (V): "); io::stdout().flush()?;
                    let mut v_in = String::new(); io::stdin().read_line(&mut v_in)?;
                    c.operating_voltage = v_in.trim().parse().ok();

                    print!("  > Enter Rated Voltage (Vmax): "); io::stdout().flush()?;
                    v_in.clear(); io::stdin().read_line(&mut v_in)?;
                    c.rated_voltage = v_in.trim().parse().ok();
                }
            }

            match calculate_fit("SN29500", c, profile, Some(variant)) {
                Ok(fit) => {
                    println!("{:<20} | {:<10} | {:<15} | {:<10.4}",
                        c.manufacturer_part_number, c.component_type, variant.name, fit);
                    processed += 1;
                },
                Err(e) => println!("Error calculating {}: {}", c.manufacturer_part_number, e),
            }
        }
    }

    // If nothing matched, compute sample FIT per selected variant
    if processed == 0 {
        println!("\n[WARN] No matching components found in DB for selected type(s). Computing sample FIT for selected variant(s).");

        // If capacitor is among selected types, ask for sample voltages once
        let mut sample_u: Option<f64> = None;
        let mut sample_umax: Option<f64> = None;
        if selected_types.iter().any(|t| t.to_lowercase().contains("capacit")) {
            print!("Enter sample Operating Voltage (V) [default 5]: "); io::stdout().flush()?;
            let mut v_in = String::new(); io::stdin().read_line(&mut v_in)?;
            sample_u = v_in.trim().parse::<f64>().ok().or(Some(5.0));

            print!("Enter sample Rated Voltage (Vmax) [default 10]: "); io::stdout().flush()?;
            v_in.clear(); io::stdin().read_line(&mut v_in)?;
            sample_umax = v_in.trim().parse::<f64>().ok().or(Some(10.0));
        }

        for &v_idx in &selected_variant_indices {
            let variant = &variants[v_idx];

            let sample_comp = Component {
                id: Uuid::new_v4(),
                project_id: Uuid::new_v4(),
                manufacturer_part_number: format!("SAMPLE-{}", variant.name),
                manufacturer: None,
                reference_designator: None,
                quantity: 1,
                created_at: Utc::now(),
                component_type: selected_types.get(0).cloned().unwrap_or_else(|| "Unknown".to_string()),
                base_fit: None,
                quality_factor: None,
                resistor_type: None,
                mission_profile_id: None,
                subtype_id: Some(variant.subtype_id),
                variant_id: Some(variant.id),
                operating_voltage: sample_u,
                rated_voltage: sample_umax,
            };

            match calculate_fit("SN29500", &sample_comp, profile, Some(variant)) {
                Ok(fit) => println!("{:<20} | {:<10} | {:<15} | {:<10.4}",
                    sample_comp.manufacturer_part_number, sample_comp.component_type, variant.name, fit),
                Err(e) => println!("Error computing sample FIT for variant {}: {}", variant.name, e),
            }
        }
    }

    println!("\n✅ FMEDA FIT computation complete.");
    Ok(())
}