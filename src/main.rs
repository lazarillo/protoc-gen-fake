use once_cell::sync::Lazy;
use prost::Message;
use prost_reflect::{
    Cardinality, DescriptorPool, DynamicMessage, ExtensionDescriptor, FieldDescriptor,
    FileDescriptor, Kind as ProstFieldKind, MessageDescriptor, Value,
};
use prost_types::FileDescriptorSet;
use protobuf::Message as PbMessage;
use protobuf::descriptor::{
    FieldOptions as PbFieldOptions, FileDescriptorProto as PbFileDescriptorProto,
};
use protobuf::plugin::{
    CodeGeneratorRequest as PbCodeGeneratorRequest,
    CodeGeneratorResponse as PbCodeGeneratorResponse,
};
use rand::Rng;
use serde_json::{Value as JsonValue, json}; // Import serde_json for JSON handling
use std::cmp::{max, min};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::path::Path; // Import Path for file handling // Import locales for fake data generation // Import Lazy for static initialization

#[path = "./gen_protobuf/fake_field.rs"]
pub mod generated_proto; // Import the generated protobuf code for custom options

use crate::generated_proto::FakeDataFieldOption;
use crate::generated_proto::exts::fake_data;

pub mod fake_data; // Import your fake data generation logic
use crate::fake_data::{FakeData, get_fake_data};

static OPTION_DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    // Create a descriptor pool with the fake field option (and descriptor) registered
    let descriptor_set = include_bytes!(env!("DESCRIPTOR_SET_BIN_PATH"));
    DescriptorPool::decode(descriptor_set.as_ref())
        .expect("Failed to decode compiled descriptor set with fake field options")
});

fn main() -> io::Result<()> {
    // Initialize logging for better debugging output
    env_logger::init(); // RUST_LOG=info, debug, or trace for more detail

    // Read the CodeGeneratorRequest from stdin
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    // Create a random number generator object
    let mut rng = rand::rng();

    // Log raw stdin buffer at TRACE level (only shows with RUST_LOG=trace)
    log::trace!("Raw stdin buffer (hex): {:x?}", buffer);

    // Decode the request using protobuf::Message::parse_from_bytes
    let request = PbCodeGeneratorRequest::parse_from_bytes(&buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let request_copy = request.clone();

    // Log full request at DEBUG level (only shows with RUST_LOG=debug or lower)
    log::debug!("Full CodeGeneratorRequest received: {:#?}", request);

    let mut output_format = "protobuf_binary".to_string(); // Default output format
    if let Some(params) = request.parameter.as_ref() {
        for param in params.split(',') {
            let key_val = param.split('=').collect::<Vec<&str>>();
            if key_val.len() == 2 && key_val[0] == "format" {
                if key_val[1].to_lowercase().starts_with("proto") {
                    output_format = "protobuf_binary".to_string();
                    log::info!(
                        "Parameter input '{}' found, output format set to: {}",
                        params,
                        output_format
                    )
                } else if key_val[1].to_lowercase().starts_with("json") {
                    output_format = "json".to_string();
                    log::info!(
                        "Parameter input '{}' found, output format set to: {}",
                        params,
                        output_format
                    )
                } else {
                    log::warn!(
                        "Unrecognized output format '{}', defaulting to '{}'",
                        key_val[1],
                        output_format
                    );
                }
            } else {
                log::warn!("Unrecognized parameter '{}', expected 'format=...'", param);
            }
        }
    } else {
        log::info!(
            "No parameters provided, using default output format: '{}'",
            output_format
        );
    }

    // Get the set of files to generate (explicitly requested by protoc)
    let key_files: HashSet<String> = request
        .file_to_generate
        .iter()
        .map(|s_ref| s_ref.clone())
        .collect();

    let mut response = PbCodeGeneratorResponse::new(); // Use .new() for rust-protobuf messages

    // Build the runtime descriptor pool, including what is passed by the user
    log::debug!("Building runtime descriptor pool, including user-provided files");
    let mut runtime_file_descriptor_set = FileDescriptorSet::default();
    // Convert between `rust-protobuf` and `prost` types
    runtime_file_descriptor_set.file = request
        .proto_file
        .into_iter()
        .map(|pb_fd| {
            let pb_bytes = pb_fd
                .write_to_bytes()
                .expect("Failed to serialize PbFileDescriptorProto");
            prost_types::FileDescriptorProto::decode(pb_bytes.as_ref())
                .expect("Failed to decode prost_types::FileDescriptorProto")
        })
        .collect();
    let runtime_descriptor_pool =
        DescriptorPool::from_file_descriptor_set(runtime_file_descriptor_set).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to build runtime descriptor pool: {}", e),
            )
        })?;
    log::debug!(
        "Runtime descriptor pool built successfully with the following {} files: {}",
        runtime_descriptor_pool.files().len(),
        runtime_descriptor_pool
            .files()
            .map(|f| f.name().to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    // I will need to double check this line immediately below... cannot know until I run it
    let fake_data_option_descr = OPTION_DESCRIPTOR_POOL
        .get_extension_by_name("gen_fake.fake_data")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Extension 'gen_fake.fake_data' definition not found in static descriptor pool. Ensure fake_field.proto is compiled into DESCRIPTOR_SET_BIN_PATH."))?;

    // Iterate through the key file(s) to use for generating fake data
    // This is the main entry point for processing the request.
    for filename in key_files.iter() {
        if let Some(file_descr) = request_copy
            .proto_file
            .iter()
            .find(|f| f.name.as_ref() == Some(filename))
        {
            log::info!(
                "Processing file of interest: {}",
                filename // file_descr.name.as_deref().unwrap_or_default()
            );
            let output_file_path = Path::new(filename);
            let file_stem = output_file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let output_name = match output_format.as_str() {
                "json" => format!("{}.json", file_stem),
                _ => format!("{}.bin", file_stem),
            };
            let mut fake_data_content: Vec<u8> = Vec::new();
            let runtime_file_descriptor = runtime_descriptor_pool.get_file_by_name(filename)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("File '{}' not found in (`prost-reflect`) runtime descriptor pool. This should not happen if {} is valid", filename, filename),
                    )
                })?;
            for message_descr in runtime_file_descriptor.messages() {
                let message_name = message_descr.name();
                log::debug!(" Message: {}", message_name);
                let mut message = DynamicMessage::new(message_descr.clone());
                // Maybe this can be created only if the json flag is set?
                let mut json_message = serde_json::Map::new();
                for field_descr in message_descr.fields() {
                    let field_name = field_descr.name();
                    // Whether it is optional, required, or repeated
                    let field_cardinality = field_descr.cardinality();
                    let field_kind = &field_descr.kind();
                    let is_list_field = field_descr.is_list();
                    let is_map_field = field_descr.is_map();
                    // --- 4. Find the *corresponding* rust-protobuf FieldDescriptorProto to get options ---
                    // We need to iterate the raw protobuf::descriptor::DescriptorProto (message_type)
                    // and then its field_protos to find the matching field by name.
                    let message_proto = file_descr
                        .message_type
                        .iter()
                        .find(|m| m.name.as_deref() == Some(message_name))
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::NotFound,
                                format!("MessageProto for '{}' not found.", message_name),
                            )
                        })?;

                    let field_proto = message_proto
                        .field
                        .iter()
                        .find(|f| f.name.as_deref() == Some(field_name))
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::NotFound,
                                format!("PbFieldProto for '{}' not found.", field_name),
                            )
                        })?;

                    // Now, access options using rust-protobuf's FieldOptions
                    if let Some(pb_options) = field_proto.options.as_ref() {
                        // Use rust-protobuf's get_extension for your custom option
                        if let Some(fake_data_option) = fake_data.get(pb_options) {
                            let data_type = fake_data_option.data_type.as_str();
                            let language = fake_data_option.language.as_str();
                            let min_count = max(fake_data_option.min_count, 0);
                            let max_count = max(fake_data_option.max_count, max(min_count, 1));
                            if is_list_field {
                                let mut json_values = Vec::new();
                                let mut repeated_values = Vec::new();

                                let num_values = rng.random_range(min_count..=max_count);

                                for idx in 0..num_values {
                                    if let Some(fake_value) = get_fake_data(data_type, language) {
                                        let fake_value_for_logging = fake_value.clone();
                                        if output_format == "json" {
                                            json_values.push(json!(&fake_value.to_string()));
                                        } else {
                                            repeated_values.push(
                                                fake_value.into_prost_reflect_value(field_kind),
                                            );
                                        }
                                        /// TODO: I am right here. I need to:
                                        /// 1. Look at the cardinality or is_list type to see if it should be repeated
                                        /// 2. Generate a vector of values based on min and max counts (setting
                                        ///    the defaults in min and max according to proto options documentation)
                                        /// 3. Generate a *maybe not present* value if the field is optional
                                        /// 4. Set the value in the DynamicMessage
                                        /// 5. Set the value in the JSON message
                                        log::info!(
                                            "  Field '{}' - fake data type '{}' in '{}' with min '{}' and max'{}' iteration {}:  '{}'",
                                            field_name,
                                            data_type,
                                            language,
                                            min_count,
                                            max_count,
                                            idx + 1,
                                            fake_value_for_logging,
                                        )
                                    } else {
                                        log::info!(
                                            "  Field '{}' - requested fake data of type '{}' in '{}' (iter {}), but failed to generate it",
                                            field_name,
                                            data_type,
                                            language,
                                            idx + 1,
                                        )
                                    }
                                }
                            } else if field_cardinality == Cardinality::Required {
                                // For required fields, we generate a single value
                                if let Some(fake_value) = get_fake_data(data_type, language) {
                                    let fake_value_for_logging = fake_value.clone();
                                    if output_format == "json" {
                                        json_message.insert(
                                            field_name.to_string(),
                                            json!(fake_value.to_string()),
                                        );
                                    } else {
                                        message.set_field(
                                            &field_descr,
                                            fake_value.into_prost_reflect_value(field_kind),
                                        );
                                    }
                                    log::info!(
                                        "  Field '{}' - fake data type '{}' in '{}' with min '{}' and max'{}':  '{}'",
                                        field_name,
                                        data_type,
                                        language,
                                        min_count,
                                        max_count,
                                        fake_value_for_logging
                                    )
                                } else {
                                    log::info!(
                                        "  Field '{}' - requested fake data of type '{}' in '{}', but failed to generate it",
                                        field_name,
                                        data_type,
                                        language
                                    )
                                }
                            } else if field_cardinality == Cardinality::Optional {
                                // For optional fields, we generate a single value or leave it unset
                                let should_generate_value = rng.random_bool(0.5);
                                if should_generate_value || min_count > 0 {
                                    if let Some(fake_value) = get_fake_data(data_type, language) {
                                        let fake_value_for_logging = fake_value.clone();
                                        if output_format == "json" {
                                            json_message.insert(
                                                field_name.to_string(),
                                                json!(fake_value.to_string()),
                                            );
                                        } else {
                                            message.set_field(
                                                &field_descr,
                                                fake_value.into_prost_reflect_value(field_kind),
                                            );
                                        }
                                        log::info!(
                                            "  Field '{}' - fake data type '{}' in '{}' with min '{}' and max'{}':  '{}'",
                                            field_name,
                                            data_type,
                                            language,
                                            min_count,
                                            max_count,
                                            fake_value_for_logging
                                        )
                                    } else {
                                        log::info!(
                                            "  Field '{}' - requested fake data of type '{}' in '{}', but failed to generate it",
                                            field_name,
                                            data_type,
                                            language
                                        )
                                    }
                                } else {
                                    log::info!(
                                        "  Field '{}' is optional, not generating value (50% chance)",
                                        field_name
                                    )
                                }
                            } else {
                                // Map fields are not supported yet
                                log::warn!(
                                    "Field '{}' has unsupported cardinality mapping: {:?}. Skipping.",
                                    field_name,
                                    is_map_field,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    //             // Add a dummy generated file to the response
    //             let mut generated_file = protobuf::plugin::code_generator_response::File::new();
    //             generated_file.set_name(format!("{}.txt", file_name));
    //             generated_file.set_content(format!("// Generated content for {}", file_name));
    //             response.file.push(generated_file);
    //         } else {
    //             log::debug!("Skipping dependency/non-primary file: {}", file_name);
    //         }
    //     }
    // }

    // Encode the CodeGeneratorResponse and write to stdout
    let mut output_buffer = Vec::new();
    response.write_to_vec(&mut output_buffer)?; // Use write_to_vec() for rust-protobuf
    io::stdout().write_all(&output_buffer)?;

    Ok(())
}

// fn iterate_over_file_descriptor(
//     file_descr: &FileDescriptor,
//     fake_data_extension: &ExtensionDescriptor,
// ) {
//     for message_descr in file_descr.messages() {
//         log::info!("  Message: {}", message_descr.name());
//         for field_descr in message_descr.fields() {
//             log::info!("    Field: {}", field_descr.name());

//             let field_options = field_descr.options();
//             let option_value = field_options.get_extension(&fake_data_extension);

//             if let Value::Message(option_message) = option_value.as_ref() {
//                 let serialized_option = option_message.encode_to_vec();
//                 match FakeDataFieldOption::decode(serialized_option.as_slice()) {
//                     Ok(fake_data_option) => {
//                         log::info!(
//                             "      SUCCESS: Found custom FakeDataFieldOption on field '{}': data_type = '{}'",
//                             field_descr.name(),
//                             fake_data_option.data_type
//                         );
//                     }
//                     Err(e) => {
//                         log::error!(
//                             "      ERROR: Failed to decode FakeDataFieldOption from reflected data for field '{}': {}",
//                             field_descr.name(),
//                             e
//                         );
//                     }
//                 }
//             }
//         }
//     }
// }

//                                     for _ in 0..num_items {
//                                         let fake_data_item = get_fake_data(data_type_str, *locale_to_use);
//                                         prost_values_for_list.push(fake_data_item.into_prost_reflect_value(prost_field_kind));
//                                         json_array_values.push(json!(fake_data_item.to_string()));
//                                     }
//                                     dynamic_message.set_field(prost_field_descriptor, Value::List(prost_values_for_list));
//                                     json_map.insert(field_name.to_string(), JsonValue::Array(json_array_values));
//                                 } else if is_map_field {
//                                     log::warn!(
//                                         "Map field '{}' has a custom option. Dynamic generation of map entries is not fully supported by this plugin yet. Generating an empty map.",
//                                         field_name
//                                     );
//                                     dynamic_message.set_field(prost_field_descriptor, Value::Map(Default::default()));
//                                     json_map.insert(field_name.to_string(), JsonValue::Object(Default::default()));
//                                 } else {
//                                     let fake_data = get_fake_data(data_type_str, *locale_to_use);
//                                     let prost_value = fake_data.into_prost_reflect_value(prost_field_kind);
//                                     dynamic_message.set_field(prost_field_descriptor, prost_value);
//                                     json_map.insert(field_name.to_string(), json!(fake_data.to_string()));
//                                 }
//                             } else {
//                                 log::debug!("      No FakeDataFieldOption found via `fake_data.get()` for field '{}', skipping population. Setting default.", field_name);
//                                 if let Some(default_value_ref) = dynamic_message.get_field(prost_field_descriptor) {
//                                     json_map.insert(field_name.to_string(), default_value_ref.to_json_value());
//                                 }
//                             }
//                         } else {
//                             log::debug!("      Field '{}' has no options, skipping population. Setting default.", field_name);
//                             if let Some(default_value_ref) = dynamic_message.get_field(prost_field_descriptor) {
//                                 json_map.insert(field_name.to_string(), default_value_ref.to_json_value());
//                             }
//                         }
//                         // --- HIGHLIGHT END ---
//                     }

//                     // --- 5. Serialize the populated message (using prost-reflect / prost) ---
//                     match output_format.as_str() {
//                         "json" => {
//                             generated_file_content = serde_json::to_vec_pretty(&json_map)
//                                 .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to serialize to JSON: {}", e)))?;
//                             content_is_string = true;
//                         },
//                         _ => { // Default: protobuf_binary
//                             generated_file_content = dynamic_message.encode_to_vec()
//                                 .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to encode DynamicMessage to Protobuf bytes: {}", e)))?;
//                         },
//                     }
//                 }

//                 // --- 6. Prepare CodeGeneratorResponse (using rust-protobuf) ---
//                 let mut generated_file = PbCodeGeneratorResponse::new_file();
//                 generated_file.set_name(output_name);
//                 if content_is_string {
//                     generated_file.set_content(String::from_utf8(generated_file_content)
//                         .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to convert UTF-8 bytes to String for response: {}", e)))?);
//                 } else {
//                     generated_file.set_content_bytes(generated_file_content);
//                 }
//                 response.file.push(generated_file);
//             } else {
//                 log::debug!("Skipping dependency/non-primary file: {}", proto_file_name);
//             }
//         }
//     }

//     // --- 7. Encode CodeGeneratorResponse and write to stdout (using rust-protobuf) ---
//     let mut output_buffer = Vec::new();
//     response.write_to_vec(&mut output_buffer)?;
//     io::stdout().write_all(&output_buffer)?;

//     Ok(())
// }
