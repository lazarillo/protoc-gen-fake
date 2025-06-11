use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::{FirstName, LastName};
use log::{error, info};
use protobuf::Message;
use protobuf::descriptor::DescriptorProto;
use std::collections::HashMap;
use std::io::{self, Read};
use std::process;

// Include the generated protobuf files
mod protos {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

// The extension field number for our fake data options (must match the number in fake_field.proto)
const FAKE_DATA_EXTENSION_NUMBER: u32 = 1491;

// Function to generate fake data based on the specified type
fn generate_fake_data(data_type: &str) -> String {
    match data_type {
        "FirstName" => FirstName().fake::<String>(),
        "LastName" => LastName().fake::<String>(),
        "SafeEmail" => SafeEmail().fake::<String>(),
        // Add more fake data types as needed
        _ => format!("<unsupported fake data type: {}>", data_type),
    }
}

// Process a message descriptor and extract fields with fake data information
fn process_message(message: &DescriptorProto) -> HashMap<String, String> {
    let mut field_fake_data = HashMap::new();
    let message_name = message.name().to_string();

    for field in &message.field {
        // Skip fields without options
        if field.options.is_none() {
            continue;
        }

        let field_name = field.name().to_string();
        let options = field.options.as_ref().unwrap();

        // Scan through unknown fields looking for our extension
        for unknown_field in options.special_fields.unknown_fields() {
            if unknown_field.0 == FAKE_DATA_EXTENSION_NUMBER {
                // Try to extract the data type from the unknown field bytes
                if let Some(value) = extract_string_from_unknown_field(&unknown_field.1) {
                    // Generate fake data based on the type
                    let fake_value = generate_fake_data(&value);
                    field_fake_data.insert(field_name.clone(), fake_value);
                    break;
                }
            }
        }
    }

    field_fake_data
}

// Helper function to extract string value from unknown field
fn extract_string_from_unknown_field(field_value: &protobuf::UnknownValueRef) -> Option<String> {
    // We expect a length-delimited field which contains our FakeDataFieldOption message
    if let protobuf::UnknownValueRef::LengthDelimited(bytes) = *field_value {
        // The data_type field in FakeDataFieldOption is a string at field number 1
        let data = bytes.as_ref();

        // Basic protobuf parsing logic for string field:
        // Field key is (field_number << 3) | wire_type
        // String wire type is 2 (length-delimited)
        // So we look for tag 0x0A (field 1, wire type 2)
        let mut pos = 0;
        while pos < data.len() {
            if data[pos] == 0x0A {
                // Found field 1, now get the length
                pos += 1;
                if pos < data.len() {
                    let len = data[pos] as usize;
                    pos += 1;
                    if pos + len <= data.len() {
                        // Extract the string
                        return String::from_utf8(data[pos..pos + len].to_vec()).ok();
                    }
                }
                break;
            } else {
                // Skip unknown field
                pos += 1;
                // Skip length for length-delimited fields
                if data[pos - 1] & 0x07 == 2 && pos < data.len() {
                    let len = data[pos] as usize;
                    pos += 1 + len;
                }
            }
        }
    }
    None
}

// Generate code to create fake instances of a message type
fn generate_fake_instance_code(message_name: &str, fake_data: &HashMap<String, String>) -> String {
    let mut code = String::new();

    // Generate function to create a fake instance
    code.push_str(&format!("// Generate fake instance of {}\n", message_name));
    code.push_str(&format!(
        "pub fn generate_fake_{}() -> {} {{\n",
        message_name.to_lowercase(),
        message_name
    ));
    code.push_str(&format!("    {} {{\n", message_name));

    // Add fake data for each field
    for (field, value) in fake_data {
        code.push_str(&format!("        {}: \"{}\".to_string(),\n", field, value));
    }

    // Close the struct and function
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code
}

fn main() {
    env_logger::init();
    info!("protoc-gen-fake plugin starting");

    // Read input data from stdin
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap_or_else(|e| {
        error!("Failed to read from stdin: {}", e);
        process::exit(1);
    });

    // Parse as FileDescriptorSet
    let descriptor_set = match protobuf::Message::parse_from_bytes(&input) {
        Ok(ds) => ds,
        Err(e) => {
            error!("Failed to parse input as FileDescriptorSet: {}", e);
            // Since protobuf 3.7.2 doesn't have compiler_plugin, we'll just handle FileDescriptorSet only
            error!("Only FileDescriptorSet format is supported with this protobuf version");
            process::exit(1);
        }
    };

    // Process FileDescriptorSet directly
    process_file_descriptor_set(descriptor_set);
}

// Note: This function is removed as compiler_plugin is not supported
// in protobuf 3.7.2. If you upgrade protobuf in the future, you can
// add back support for CodeGeneratorRequest.

// Process a FileDescriptorSet directly
fn process_file_descriptor_set(descriptor_set: protobuf::descriptor::FileDescriptorSet) {
    info!("Processing FileDescriptorSet");

    for file_proto in &descriptor_set.file {
        let file_name = file_proto.name();
        info!("Examining file: {}", file_name);

        // Only process files with messages
        if file_proto.message_type.is_empty() {
            continue;
        }

        let mut results = Vec::new();

        // Process each message in the file
        for message in &file_proto.message_type {
            let message_name = message.name();
            info!("Examining message: {}", message_name);

            // Extract fake data information
            let fake_data = process_message(message);

            if !fake_data.is_empty() {
                // Print results
                results.push(format!("Message: {}", message_name));
                for (field, value) in &fake_data {
                    results.push(format!("  Field '{}': {}", field, value));
                }
            }
        }

        if !results.is_empty() {
            println!("File: {}", file_name);
            for line in results {
                println!("{}", line);
            }
            println!();
        }
    }
}
