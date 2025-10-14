// mod db;
// mod models;
// mod calc;

// use db::init_db;
// use models::{Component, MissionProfile};
// use calc::sn29500::calc_fit;
// use sqlx::FromRow;
// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // --- 1️⃣ Connect to DB ---
//     let pool = init_db().await?;
//     println!("[DB TEST] Connected to database.");

//     // --- 2️⃣ Fetch first component dynamically ---
//     let component: Component = sqlx::query_as::<_, Component>(
//         "SELECT * FROM components LIMIT 1"
//     )
//     .fetch_one(&pool)
//     .await?;

//     // --- 3️⃣ Fetch related mission profile ---
//     let mission_profile_id = component.mission_profile_id.unwrap();
//     let mission_profile: MissionProfile = sqlx::query_as::<_, MissionProfile>(
//         "SELECT * FROM mission_profiles WHERE id = $1"
//     )
//     .bind(mission_profile_id)
//     .fetch_one(&pool)
//     .await?;

//     // --- 4️⃣ Calculate FIT from DB temp/tau ---
//     let fit = if let Some(temp_tau_json) = &mission_profile.temp_tau_profile {
//         let temp_tau_vec: Vec<(f64, f64)> = serde_json::from_value(temp_tau_json.clone())
//             .unwrap_or_default();

//         let on_time: f64 = temp_tau_vec.iter().map(|(_, tau)| *tau).sum();
//         let mut pit_sum = 0.0;

//         for (temp, tau) in temp_tau_vec {
//             let temp_profile = MissionProfile {
//                 operating_temp: Some(temp),
//                 ..mission_profile.clone()
//             };
//             let pit = calc_fit(&component, &temp_profile);
//             pit_sum += pit * (tau / on_time);
//             println!(
//                 "[FMEDA] Temp: {:.1}°C, Tau: {:.1}h, PiT: {:.6}",
//                 temp, tau, pit
//             );
//         }
//         pit_sum
//     } else {
//         calc_fit(&component, &mission_profile)
//     };

//     println!(
//         "[FMEDA] Final FIT for component {}: {:.6}",
//         component.manufacturer_part_number, fit
//     );

//     Ok(())
// }
mod db;
mod models;
mod calc;

use db::init_db;
use models::{Component, MissionProfile};
// use calc::sn29500::calc_fit;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- 1️⃣ Connect to DB ---
    let pool = init_db().await?;
    println!("[DB TEST] ✅ Connected to database.\n");

    // --- 2️⃣ Fetch mission profile ---
    let profiles: Vec<MissionProfile> = sqlx::query_as::<_, MissionProfile>(
        "SELECT * FROM mission_profiles ORDER BY created_at DESC LIMIT 1"
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

    // --- 3️⃣ Fetch components ---
    let components: Vec<Component> = sqlx::query_as::<_, Component>(
        "SELECT * FROM components"
    )
    .fetch_all(&pool)
    .await?;

    if components.is_empty() {
        println!("⚠️ No components found in DB.");
        return Ok(());
    }

    println!("[Component Reliability Results]");
    println!("{:<30} | {:<15} | {:<10}", "Manufacturer P/N", "Type", "FIT (Failures/1e9h)");
    println!("{}", "-".repeat(70));

// --- 4️⃣ Compute FIT for each component ---
for comp in &components {
    match crate::calc::calculate_fit("SN29500", comp, profile) {
        Ok(fit) => println!(
            "{:<30} | {:<15} | {:<10.6}",
            comp.manufacturer_part_number,
            comp.component_type,
            fit
        ),
        Err(e) => println!(
            "{:<30} | {:<15} | Error: {}",
            comp.manufacturer_part_number,
            comp.component_type,
            e
        ),
    }
}


    println!("\n✅ FMEDA FIT computation complete.");
    Ok(())
}
