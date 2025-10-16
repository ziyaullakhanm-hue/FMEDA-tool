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
use models::{Component, ComponentVariant, MissionProfile};
use std::error::Error;
use std::io::{self, Write};
use itertools::Itertools; // Add to Cargo.toml: itertools = "0.10"
use crate::calc::calculate_fit;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- 1️⃣ Connect to DB ---
    let pool = init_db().await?;
    println!("[DB TEST] ✅ Connected to database.\n");

    // --- 2️⃣ Fetch latest mission profile ---
    let profiles = sqlx::query_as::<_, MissionProfile>(
        "SELECT * FROM mission_profiles"
    )
    .fetch_all(&pool)
    .await?;


    if profiles.is_empty() {
        println!("⚠️ No mission profiles found in DB.");
        return Ok(());
    }
    let profile = &profiles[0];

    println!("[Mission Profile Loaded]");
    println!("Name: {}", profile.name);
    println!("Description: {}", profile.description.as_deref().unwrap_or("None"));
    println!("Temp/Tau Profile: {:#?}\n", profile.temp_tau_profile);

    // --- 3️⃣ Fetch all components with optional variants/subtypes ---
    let components: Vec<Component> = sqlx::query_as::<_, Component>(
        "SELECT * FROM components"
    )
    .fetch_all(&pool)
    .await?;

    if components.is_empty() {
        println!("⚠️ No components found in DB.");
        return Ok(());
    }

    // --- 4️⃣ Interactive Component Type Selection ---
    let component_types: Vec<String> = components.iter()
        .map(|c| c.component_type.clone())
        .unique()
        .collect();

    println!("Select Component Type:");
    for (i, t) in component_types.iter().enumerate() {
        println!("{}: {}", i + 1, t);
    }

    print!("Enter number(s), separated by comma: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let selected_type_idxs: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect();

    let selected_types: Vec<&String> = selected_type_idxs
        .iter()
        .filter_map(|&i| component_types.get(i - 1))
        .collect();

    if selected_types.is_empty() {
        println!("⚠️ No component types selected.");
        return Ok(());
    }

    // --- 5️⃣ Interactive Subtype Selection ---
    let mut subtypes: Vec<(String, Uuid)> = Vec::new(); // (subtype_name, subtype_id)
    for c in &components {
        if selected_types.contains(&&c.component_type) {
            if let Some(vid) = c.variant_id {
                let var: ComponentVariant = sqlx::query_as(
                    "SELECT * FROM component_variants WHERE id = $1"
                )
                .bind(vid)
                .fetch_one(&pool)
                .await?;

                let subtype_name: (String,) = sqlx::query_as(
                    "SELECT name FROM component_subtypes WHERE id = $1"
                )
                .bind(var.subtype_id)
                .fetch_one(&pool)
                .await?;

                if !subtypes.iter().any(|(n, _)| n == &subtype_name.0) {
                    subtypes.push((subtype_name.0, var.subtype_id));
                }
            }
        }
    }

    if subtypes.is_empty() {
        println!("⚠️ No subtypes available for selected types.");
        return Ok(());
    }

    println!("\nSelect Subtype(s) for selected types:");
    for (i, (s, _)) in subtypes.iter().enumerate() {
        println!("{}: {}", i + 1, s);
    }

    print!("Enter number(s), separated by comma: ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let selected_subtype_idxs: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect();

    let selected_subtypes: Vec<Uuid> = selected_subtype_idxs
        .iter()
        .filter_map(|&i| subtypes.get(i - 1).map(|(_, id)| *id))
        .collect();

    if selected_subtypes.is_empty() {
        println!("⚠️ No subtypes selected.");
        return Ok(());
    }

    // --- 6️⃣ Interactive Variant Selection ---
    let mut variants: Vec<(String, Uuid, Uuid)> = Vec::new(); // (variant_name, variant_id, subtype_id)
    for c in &components {
        if selected_types.contains(&&c.component_type) {
            if let Some(vid) = c.variant_id {
                let var: ComponentVariant = sqlx::query_as(
                    "SELECT * FROM component_variants WHERE id = $1"
                )
                .bind(vid)
                .fetch_one(&pool)
                .await?;

                if selected_subtypes.contains(&var.subtype_id) {
                    if !variants.iter().any(|(n, _, _)| n == &var.name) {
                        variants.push((var.name.clone(), var.id, var.subtype_id));
                    }
                }
            }
        }
    }

    if variants.is_empty() {
        println!("⚠️ No variants available for selected subtypes.");
        return Ok(());
    }

    println!("\nSelect Variant(s) for selected subtypes:");
    for (i, (vname, _, _)) in variants.iter().enumerate() {
        println!("{}: {}", i + 1, vname);
    }

    print!("Enter number(s), separated by comma: ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let selected_variant_idxs: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect();

    let selected_variant_ids: Vec<Uuid> = selected_variant_idxs
        .iter()
        .filter_map(|&i| variants.get(i - 1).map(|(_, id, _)| *id))
        .collect();

    if selected_variant_ids.is_empty() {
        println!("⚠️ No variants selected.");
        return Ok(());
    }

    // --- 7️⃣ Compute FIT for matching components ---
    println!("\nFIT Results:");
    println!("{:<20} | {:<10} | {:<15} | {:<12} | {:<10}",
        "Manufacturer P/N", "Type", "Subtype", "Variant", "FIT");

    for c in &components {
        if selected_types.contains(&&c.component_type) {
            if let Some(vid) = c.variant_id {
                if selected_variant_ids.contains(&vid) {
                    let variant: ComponentVariant = sqlx::query_as(
                        "SELECT * FROM component_variants WHERE id = $1"
                    )
                    .bind(vid)
                    .fetch_one(&pool)
                    .await?;

                    let subtype_name: (String,) = sqlx::query_as(
                        "SELECT name FROM component_subtypes WHERE id = $1"
                    )
                    .bind(variant.subtype_id)
                    .fetch_one(&pool)
                    .await?;

                    let fit = calculate_fit("SN29500", c, profile, Some(&variant))?;

                    println!("{:<20} | {:<10} | {:<15} | {:<12} | {:<10.6}",
                        c.manufacturer_part_number,
                        c.component_type,
                        subtype_name.0,
                        variant.name,
                        fit
                    );
                }
            }
        }
    }

    println!("\n✅ FMEDA FIT computation complete.");
    Ok(())
}
