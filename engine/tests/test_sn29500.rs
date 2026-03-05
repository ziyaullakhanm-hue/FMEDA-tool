use chrono::Utc;
use serde_json::{json, Value};
use uuid::Uuid;

// Import from the library
use fmeda_engine::calc::sn29500;
use fmeda_engine::models::{Component, ComponentVariant, MissionProfile};

#[test]
fn test_resistor_calculation() {
    // Create sample mission profile with temperature/tau segments
    let profile = MissionProfile {
        id: Uuid::new_v4(),
        name: "Resistor Test Profile".to_string(),
        description: Some("Test profile for resistor calculations".to_string()),
        temp_tau_profile: json!([
            { "temperature": 25.0, "tau": 0.3 },
            { "temperature": 55.0, "tau": 0.4 },
            { "temperature": 85.0, "tau": 0.3 }
        ]),
        created_at: Utc::now(),
        reference_temp: Some(40.0),
        operating_temp: Some(55.0),
    };

    // Create sample resistor component
    let comp = Component {
        id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        manufacturer_part_number: "1/4W_100K".to_string(),
        manufacturer: Some("Vishay".to_string()),
        reference_designator: Some("R1".to_string()),
        quantity: 1,
        created_at: Utc::now(),
        component_type: "Resistor".to_string(),
        base_fit: None,
        quality_factor: None,
        resistor_type: None,
        mission_profile_id: None,
        subtype_id: None,
        variant_id: None,
        operating_voltage: None,
        rated_voltage: None,
    };

    // Create resistor variant with SN29500 parameters
    let variant = ComponentVariant {
        id: Uuid::new_v4(),
        subtype_id: Uuid::new_v4(),
        name: "Standard Resistor".to_string(),
        ref_fit: 0.3,
        ref_temp: Some(40.0),
        a: Some(0.873),
        ea1: Some(0.16),
        ea2: Some(0.44),
        c2: None,
        c3: None,
        uref_umax_ratio: None,
        pi_q: None,
        notes: Some("SN29500 Resistor".to_string()),
        created_at: Utc::now(),
    };

    println!("\n========== RESISTOR SN29500 TEST ==========");
    println!("Component: {:?}", comp.reference_designator);
    println!("Manufacturer: {:?}", comp.manufacturer);
    
    let fit = sn29500::calc_fit(&comp, &profile, Some(&variant));
    
    println!("\n========== RESULT ==========");
    println!("FIT Result: {:.6}", fit);
    println!("Expected: ~0.5-0.8 (depends on temperature weighting)\n");
    
    assert!(fit > 0.0, "FIT should be positive");
    assert!(fit < 10.0, "FIT should be reasonable");
}

#[test]
fn test_capacitor_calculation() {
    // Create sample mission profile with temperature/tau segments
    let profile = MissionProfile {
        id: Uuid::new_v4(),
        name: "Capacitor Test Profile".to_string(),
        description: Some("Test profile for capacitor calculations".to_string()),
        temp_tau_profile: json!([
            { "temperature": 20.0, "tau": 0.25 },
            { "temperature": 50.0, "tau": 0.50 },
            { "temperature": 70.0, "tau": 0.25 }
        ]),
        created_at: Utc::now(),
        reference_temp: Some(40.0),
        operating_temp: Some(50.0),
    };

    // Create sample capacitor component
    let comp = Component {
        id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        manufacturer_part_number: "C0805_10uF".to_string(),
        manufacturer: Some("Murata".to_string()),
        reference_designator: Some("C1".to_string()),
        quantity: 1,
        created_at: Utc::now(),
        component_type: "Capacitor".to_string(),
        base_fit: None,
        quality_factor: None,
        resistor_type: None,
        mission_profile_id: None,
        subtype_id: None,
        variant_id: None,
        operating_voltage: Some(3.3),  // Operating at 3.3V
        rated_voltage: Some(10.0),     // Rated for 10V
    };

    // Create capacitor variant with SN29500 parameters
    let variant = ComponentVariant {
        id: Uuid::new_v4(),
        subtype_id: Uuid::new_v4(),
        name: "Ceramic Capacitor".to_string(),
        ref_fit: 0.8,
        ref_temp: Some(40.0),
        a: Some(0.873),
        ea1: Some(0.16),
        ea2: Some(0.44),
        c2: Some(2.5),              // Voltage exponent
        c3: Some(0.5),              // Voltage sensitivity
        uref_umax_ratio: Some(0.5), // Reference voltage is 50% of rated
        pi_q: Some(1.0),            // Quality factor
        notes: Some("SN29500 Ceramic Capacitor".to_string()),
        created_at: Utc::now(),
    };

    println!("\n========== CAPACITOR SN29500 TEST ==========");
    println!("Component: {:?}", comp.reference_designator);
    println!("Manufacturer: {:?}", comp.manufacturer);
    println!("Operating Voltage: {:?} V", comp.operating_voltage);
    println!("Rated Voltage: {:?} V", comp.rated_voltage);
    
    let fit = sn29500::calc_fit(&comp, &profile, Some(&variant));
    
    println!("\n========== RESULT ==========");
    println!("FIT Result: {:.6}", fit);
    println!("Expected: ~0.5-1.2 (depends on voltage and temperature weighting)\n");
    
    assert!(fit > 0.0, "FIT should be positive");
    assert!(fit < 10.0, "FIT should be reasonable");
}

#[test]
fn test_no_variant_fallback() {
    let profile = MissionProfile {
        id: Uuid::new_v4(),
        name: "Fallback Test".to_string(),
        description: None,
        temp_tau_profile: json!([{ "temperature": 45.0, "tau": 1.0 }]),
        created_at: Utc::now(),
        reference_temp: None,
        operating_temp: None,
    };

    let comp = Component {
        id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        manufacturer_part_number: "TEST_COMP".to_string(),
        manufacturer: None,
        reference_designator: Some("R_TEST".to_string()),
        quantity: 1,
        created_at: Utc::now(),
        component_type: "Resistor".to_string(),
        base_fit: None,
        quality_factor: None,
        resistor_type: None,
        mission_profile_id: None,
        subtype_id: None,
        variant_id: None,
        operating_voltage: None,
        rated_voltage: None,
    };

    println!("\n========== NO VARIANT FALLBACK TEST ==========");
    
    // Should use default FIT when no variant provided
    let fit = sn29500::calc_fit(&comp, &profile, None);
    
    println!("FIT Result (no variant): {:.6}", fit);
    println!("Expected: 0.3 (default resistor fallback)\n");
    
    assert_eq!(fit, 0.3, "Should return default resistor FIT of 0.3");
}

#[test]
fn test_complex_temperature_profile() {
    // Test with more complex temperature profile
    let profile = MissionProfile {
        id: Uuid::new_v4(),
        name: "Complex Temperature Profile".to_string(),
        description: Some("Multiple temperature segments".to_string()),
        temp_tau_profile: json!([
            { "temperature": 5.0, "tau": 0.1 },
            { "temperature": 25.0, "tau": 0.3 },
            { "temperature": 45.0, "tau": 0.3 },
            { "temperature": 65.0, "tau": 0.2 },
            { "temperature": 85.0, "tau": 0.1 }
        ]),
        created_at: Utc::now(),
        reference_temp: Some(40.0),
        operating_temp: Some(45.0),
    };

    let comp = Component {
        id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        manufacturer_part_number: "COMPLEX_TEST".to_string(),
        manufacturer: Some("Test Mfg".to_string()),
        reference_designator: Some("R_COMPLEX".to_string()),
        quantity: 1,
        created_at: Utc::now(),
        component_type: "Resistor".to_string(),
        base_fit: None,
        quality_factor: None,
        resistor_type: None,
        mission_profile_id: None,
        subtype_id: None,
        variant_id: None,
        operating_voltage: None,
        rated_voltage: None,
    };

    let variant = ComponentVariant {
        id: Uuid::new_v4(),
        subtype_id: Uuid::new_v4(),
        name: "Standard Resistor".to_string(),
        ref_fit: 0.3,
        ref_temp: Some(40.0),
        a: Some(0.873),
        ea1: Some(0.16),
        ea2: Some(0.44),
        c2: None,
        c3: None,
        uref_umax_ratio: None,
        pi_q: None,
        notes: None,
        created_at: Utc::now(),
    };

    println!("\n========== COMPLEX TEMPERATURE PROFILE TEST ==========");
    println!("5 temperature segments with varying time allocations");
    
    let fit = sn29500::calc_fit(&comp, &profile, Some(&variant));
    
    println!("\n========== RESULT ==========");
    println!("FIT Result: {:.6}\n", fit);
    
    assert!(fit > 0.0, "FIT should be positive");
}
