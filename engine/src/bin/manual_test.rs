/// Interactive CLI for manual SN29500 FIT calculation testing
/// Allows selecting components, mission profiles, and variants from the database
/// 
/// Run with: cargo run --bin manual_test

use fmeda_engine::db::init_db;
use fmeda_engine::models::{Component, ComponentVariant, MissionProfile};
use fmeda_engine::calc;
use std::error::Error;
use std::io::{self, Write};
use uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║       SN29500 Manual FIT Calculation Tool                   ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Connect to database
    let pool = match init_db().await {
        Ok(p) => {
            println!("✅ Connected to database\n");
            p
        }
        Err(e) => {
            eprintln!("❌ Database connection failed: {}", e);
            eprintln!("   Please ensure DATABASE_URL is set correctly.");
            return Err(e.into());
        }
    };

    loop {
        // Step 1: Select Mission Profile
        println!("╔════ Step 1: Select Mission Profile ════╗\n");
        
        let profiles: Vec<MissionProfile> = sqlx::query_as("SELECT * FROM mission_profiles ORDER BY name")
            .fetch_all(&pool)
            .await?;

        if profiles.is_empty() {
            eprintln!("❌ No mission profiles found in database");
            return Ok(());
        }

        for (i, p) in profiles.iter().enumerate() {
            println!("  {}. {} - {}", 
                i + 1, 
                p.name, 
                p.description.as_deref().unwrap_or("(no description)"));
        }

        let profile_idx = get_user_selection("Select profile number", profiles.len());
        let profile = &profiles[profile_idx];
        
        println!("\n✅ Selected: {}", profile.name);
        println!("   Temperature/Tau Segments: {}\n", profile.temp_tau_profile);

        // Step 2: Select Component
        println!("╔════ Step 2: Select Component ════╗\n");
        
        let components: Vec<Component> = sqlx::query_as(
            "SELECT * FROM components ORDER BY component_type, manufacturer_part_number"
        )
        .fetch_all(&pool)
        .await?;

        if components.is_empty() {
            eprintln!("❌ No components found in database");
            return Ok(());
        }

        for (i, c) in components.iter().enumerate() {
            println!("  {}. [{}] {} (MPN: {})", 
                i + 1,
                c.component_type,
                c.reference_designator.as_deref().unwrap_or("?"),
                c.manufacturer_part_number);
        }

        let component_idx = get_user_selection("Select component number", components.len());
        let mut component = components[component_idx].clone();
        
        println!("\n✅ Selected: {} ({})", 
            component.reference_designator.as_deref().unwrap_or("?"),
            component.component_type);

        // Step 3: Select Component Variant (filtered by component type)
        println!("\n╔════ Step 3: Select Component Variant ════╗\n");
        
        // Query for component subtypes matching the selected component type
        let subtypes: Vec<(uuid::Uuid,)> = sqlx::query_as(
            "SELECT s.id FROM component_subtypes s 
             JOIN component_types t ON s.type_id = t.id 
             WHERE LOWER(t.name) = LOWER($1)"
        )
        .bind(&component.component_type)
        .fetch_all(&pool)
        .await?;

        if subtypes.is_empty() {
            eprintln!("❌ No subtypes found for component type: {}", component.component_type);
            return Ok(());
        }

        let subtype_ids: Vec<uuid::Uuid> = subtypes.iter().map(|(id,)| *id).collect();

        // Fetch variants only for those subtypes
        let variants: Vec<ComponentVariant> = if !subtype_ids.is_empty() {
            sqlx::query_as::<_, ComponentVariant>(
                "SELECT * FROM component_variants 
                 WHERE subtype_id = ANY($1::uuid[]) 
                 ORDER BY name"
            )
            .bind(&subtype_ids)
            .fetch_all(&pool)
            .await?
        } else {
            vec![]
        };

        if variants.is_empty() {
            eprintln!("❌ No component variants found for type: {}", component.component_type);
            return Ok(());
        }

        for (i, v) in variants.iter().enumerate() {
            println!("  {}. {} (ref_fit: {:.4})", i + 1, v.name, v.ref_fit);
        }

        let variant_idx = get_user_selection("Select variant number", variants.len());
        let variant = &variants[variant_idx];
        
        println!("\n✅ Selected: {}", variant.name);

        // Step 4: Handle component-specific inputs
        if component.component_type.to_lowercase() == "capacitor" {
            println!("\n╔════ Step 4: Capacitor Voltage Parameters ════╗\n");
            
            if component.operating_voltage.is_none() {
                print!("Enter operating voltage (V): ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                component.operating_voltage = input.trim().parse().ok();
            }

            if component.rated_voltage.is_none() {
                print!("Enter rated voltage Vmax (V): ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                component.rated_voltage = input.trim().parse().ok();
            }

            println!("\n✅ Voltages set:");
            println!("   Operating: {} V", component.operating_voltage.unwrap_or(0.0));
            println!("   Rated: {} V", component.rated_voltage.unwrap_or(0.0));
        }

        // Step 5: Calculate FIT
        println!("\n╔════ Step 5: Computing FIT ════╗\n");
        
        match calc::calculate_fit("SN29500", &component, profile, Some(variant)) {
            Ok(fit) => {
                println!("\n╔════════════════════════════════════════════════════════════╗");
                println!("║                    ✅ CALCULATION RESULT                    ║");
                println!("╚════════════════════════════════════════════════════════════╝\n");
                
                println!("Component:         {}", component.reference_designator.as_deref().unwrap_or("?"));
                println!("Type:              {}", component.component_type);
                println!("MPN:               {}", component.manufacturer_part_number);
                println!("Manufacturer:      {}", component.manufacturer.as_deref().unwrap_or("N/A"));
                println!("\nVariant:           {}", variant.name);
                println!("Reference FIT:     {:.6}", variant.ref_fit);
                
                if let Some(u) = component.operating_voltage {
                    println!("Operating Voltage: {:.2} V", u);
                }
                if let Some(u) = component.rated_voltage {
                    println!("Rated Voltage:     {:.2} V", u);
                }
                
                println!("\nMission Profile:   {}", profile.name);
                println!("\n┌──────────────────────────────────┐");
                println!("│  CALCULATED FIT: {:.6}         │", fit);
                println!("│  (Failures per 10^9 hours)      │");
                println!("└──────────────────────────────────┘\n");
            }
            Err(e) => {
                eprintln!("\n❌ Calculation failed: {}", e);
            }
        }

        // Ask if user wants to test another component
        print!("Test another component? (y/n): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            break;
        }
        println!("\n");
    }

    println!("Goodbye!\n");
    Ok(())
}

/// Helper function to get user selection input
fn get_user_selection(prompt: &str, max: usize) -> usize {
    loop {
        print!("\n{} (1-{}): ", prompt, max);
        io::stdout().flush().ok();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        
        match input.trim().parse::<usize>() {
            Ok(idx) if idx > 0 && idx <= max => return idx - 1,
            _ => println!("❌ Invalid selection. Please enter a number between 1 and {}.", max),
        }
    }
}
