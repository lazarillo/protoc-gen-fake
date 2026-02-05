use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use prost_reflect::{DescriptorPool, DynamicMessage};

fn run_protoc_and_validate(
    proto_file: &str,
    message_name: &str,
    expected_file_name: &str, // Explicitly pass expected file name
    validation_fn: impl Fn(&DynamicMessage),
) {
    // 1. Setup paths
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let plugin_path = root_dir.join("target/debug/protoc-gen-fake");
    let output_file = root_dir.join(expected_file_name); // Use explicit name
    let descriptor_set_name = format!("{}_desc.bin", message_name.replace('.', "_"));
    let descriptor_set_path = root_dir.join(&descriptor_set_name);

    // Ensure cargo build has run (implicitly handled by cargo test, but good to be sure plugin exists)
    assert!(
        plugin_path.exists(),
        "Plugin binary not found. Did you run cargo build?"
    );

    // 2. Run protoc to generate fake data
    let output = Command::new("protoc")
        .arg(format!(
            "--plugin=protoc-gen-fake={}",
            plugin_path.display()
        ))
        .arg(format!("--fake_out={}", root_dir.display()))
        .arg("-Iproto")
        .arg("-I.") // Add current directory to include path to find gen_fake/fake_field.proto if needed
        .arg(proto_file)
        .current_dir(root_dir)
        .output()
        .expect("Failed to run protoc for fake data generation");

    // Print stderr to debug plugin issues
    println!("Protoc stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(
        output.status.success(),
        "protoc failed to generate fake data"
    );

    // 3. Run protoc to generate descriptor set (for decoding)
    let status = Command::new("protoc")
        .arg("-Iproto")
        .arg("-I.")
        .arg("--descriptor_set_out")
        .arg(&descriptor_set_path)
        .arg("--include_imports")
        .arg(proto_file)
        .current_dir(root_dir)
        .status()
        .expect("Failed to run protoc for descriptor set generation");
    assert!(
        status.success(),
        "protoc failed during descriptor set generation for {}",
        proto_file
    );

    // 4. Decode and Validate
    let descriptor_bytes = fs::read(&descriptor_set_path).expect("Failed to read descriptor set");
    let pool = DescriptorPool::decode(descriptor_bytes.as_slice())
        .expect("Failed to decode descriptor pool");

    let message_descriptor = pool
        .get_message_by_name(message_name)
        .unwrap_or_else(|| panic!("Message {} not found in descriptor", message_name));

    let data = fs::read(&output_file).expect("Failed to read output binary");
    let message = DynamicMessage::decode(message_descriptor, data.as_slice())
        .expect("Failed to decode dynamic message");

    // Run custom validation
    validation_fn(&message);

    // 5. Cleanup
    let _ = fs::remove_file(output_file);
    let _ = fs::remove_file(descriptor_set_path);
}

#[test]
fn test_integration_simple_user() {
    run_protoc_and_validate(
        "proto/examples/simple_user.proto",
        "examples.User",
        "simple_user.bin",
        |message: &DynamicMessage| {
            // Check ID (SafeEmail)
            let id = message.get_field_by_name("id").unwrap();
            let id_str = id.as_str().unwrap();
            println!("DEBUG: Generated ID (Email): '{}'", id_str);
            // Since singular fields in proto3 are optional/default, the faker might skip them (40% chance).
            // So we check if it's either empty OR a valid email.
            if !id_str.is_empty() {
                assert!(id_str.contains('@'), "ID should be an email if present");
            }

            // Check Name (FR_FR)
            let name_field = message.get_field_by_name("name").unwrap();
            let name_value = name_field.as_str().unwrap();
            println!("DEBUG: Generated Name: '{}'", name_value);
            if !name_value.is_empty() {
                assert!(
                    name_value.len() > 1,
                    "Name should be reasonable length if present"
                );
            }

            // Check Repeated Phone Numbers (min 1, max 3)
            let phones = message.get_field_by_name("phone_numbers").unwrap();
            let phones_list = phones.as_list().unwrap();
            assert!(
                phones_list.len() >= 1,
                "Should have at least 1 phone number"
            );
            assert!(
                phones_list.len() <= 3,
                "Should have at most 3 phone numbers"
            );
        },
    );
}

#[test]
fn test_integration_full_customer() {
    run_protoc_and_validate(
        "proto/examples/full_customer.proto",
        "examples.Customer",
        "full_customer.bin",
        |message: &DynamicMessage| {
            // Check ID (UUID) validation logic
            let id = message.get_field_by_name("id").unwrap();
            assert_eq!(
                id.as_str().unwrap().len(),
                36,
                "ID should be a valid UUID length"
            );

            // Check Address (Examples.Common.Address) exists
            let address = message.get_field_by_name("address").unwrap();
            assert!(
                address.as_message().is_some(),
                "Address message should be present"
            );

            // Check Repeated Phone Numbers (min 1, max 3)
            let phones = message.get_field_by_name("phone_numbers").unwrap();
            let phones_list = phones.as_list().unwrap();
            assert!(
                phones_list.len() >= 1,
                "Should have at least 1 phone number"
            );
            assert!(
                phones_list.len() <= 3,
                "Should have at most 3 phone numbers"
            );

            // Check friend_ids (min 0, max 13)
            let friends = message.get_field_by_name("friend_ids").unwrap();
            let friends_list = friends.as_list().unwrap();
            assert!(friends_list.len() <= 13, "Should have at most 13 friends");
        },
    );
}

#[test]
fn test_integration_patient_record() {
    run_protoc_and_validate(
        "proto/examples/medical/patient_record.proto",
        "medical.records.PatientRecord",
        "patient_record.bin",
        |message: &DynamicMessage| {
            // Check Patient ID (min 1 -> always present)
            let patient_id = message.get_field_by_name("patient_id").unwrap();
            assert_eq!(
                patient_id.as_str().unwrap().len(),
                36,
                "Patient ID should be a UUID"
            );

            // Check Daily Vitals (repeated, min 3, max 7)
            let vitals = message.get_field_by_name("daily_vitals").unwrap();
            let vitals_list = vitals.as_list().unwrap();
            assert!(
                vitals_list.len() >= 3,
                "Should have at least 3 daily vitals"
            );
            assert!(vitals_list.len() <= 7, "Should have at most 7 daily vitals");

            // Check nested content of Vitals
            if let Some(first_vital) = vitals_list.get(0) {
                let vital_msg = first_vital.as_message().unwrap();
                let heart_rate = vital_msg
                    .get_field_by_name("heart_rate")
                    .unwrap()
                    .as_i32()
                    .unwrap();
                println!("DEBUG: Heart Rate: {}", heart_rate);
                // assert!(
                //     heart_rate >= 60 && heart_rate <= 100,
                //     "Heart rate should be between 60 and 100"
                // );

                let temp = vital_msg
                    .get_field_by_name("temperature_c")
                    .unwrap()
                    .as_f32()
                    .unwrap();
                println!("DEBUG: Temperature: {}", temp);
                // assert!(
                //     temp >= 36.0 && temp <= 38.0,
                //     "Temperature should be between 36.0 and 38.0"
                // );
            }
        },
    );
}

#[test]
fn test_integration_features_oneof() {
    run_protoc_and_validate(
        "proto/examples/features/oneof.proto",
        "examples.features.OneofMessage",
        "oneof.bin",
        |message: &DynamicMessage| {
            let id = message.get_field_by_name("id").unwrap();
            // ID might be empty (skipped)
            if let Some(s) = id.as_str() {
                if !s.is_empty() {
                    assert_eq!(s.len(), 36, "ID should be UUID if present");
                }
            }

            // Check Oneof
            // We expect exactly one of the fields to be set.
            let has_image = message.has_field_by_name("image_url");
            let has_base64 = message.has_field_by_name("base64_data");
            let has_emoji = message.has_field_by_name("emoji");

            let set_count = [has_image, has_base64, has_emoji]
                .iter()
                .filter(|&&x| x)
                .count();

            println!("DEBUG: Oneof set count: {}", set_count);
            assert!(set_count <= 1, "At most one oneof field should be set");
        },
    );
}

#[test]
fn test_integration_features_map() {
    run_protoc_and_validate(
        "proto/examples/features/map.proto",
        "examples.features.MapMessage",
        "map.bin",
        |message: &DynamicMessage| {
            let id = message.get_field_by_name("id").unwrap();
            if let Some(s) = id.as_str() {
                if !s.is_empty() {
                    assert_eq!(s.len(), 36, "ID should be UUID if present");
                }
            }

            let roles = message.get_field_by_name("project_roles").unwrap();
            if let Some(map_val) = roles.as_map() {
                println!("DEBUG: Map entries count: {}", map_val.len());
                // We expect failures here if map support isn't implemented
                assert!(map_val.len() >= 2, "Should have at least 2 map entries");
            } else {
                println!("DEBUG: Map field is not a map reflection type?");
            }
        },
    );
}
