use once_cell::sync::Lazy;
use prost::{Message, bytes};
use prost_reflect::{
    Cardinality, DescriptorPool, DynamicMessage, ExtensionDescriptor, FieldDescriptor,
    FileDescriptor, Kind as ProstFieldKind, MessageDescriptor, Value,
};
use prost_types::{FileDescriptorSet, field};
use protobuf::Message as PbMessage;
use protobuf::descriptor::{
    FieldOptions as PbFieldOptions, FileDescriptorProto as PbFileDescriptorProto,
};
use protobuf::plugin::{
    CodeGeneratorRequest as PbCodeGeneratorRequest,
    CodeGeneratorResponse as PbCodeGeneratorResponse,
};
use rand::Rng;
use serde::de;
use serde_json::{Map as JsonMap, Value as JsonValue, json, to_vec_pretty}; // Import serde_json for JSON handling
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
            let mut all_messages: Vec<DynamicMessage> = Vec::new();
            let mut all_json_messages: Vec<JsonMap<String, JsonValue>> = Vec::new();
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
                let mut json_message: JsonMap<String, JsonValue> = JsonMap::new();
                for field_descr in message_descr.fields() {
                    let field_name = field_descr.name();
                    // Whether it is optional, required, or repeated
                    let field_cardinality = field_descr.cardinality();
                    let field_kind = &field_descr.kind();
                    let is_list_field = field_descr.is_list();
                    let is_map_field = field_descr.is_map();

                    let mut fake_prost_value: Option<Value> = None;
                    let mut fake_json_value: Option<JsonValue> = None;
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
                                if output_format == "json" {
                                    fake_json_value = Some(JsonValue::Array(json_values));
                                } else {
                                    fake_prost_value = Some(Value::List(repeated_values));
                                }
                            } else if field_cardinality == Cardinality::Required {
                                // For required fields, we generate a single value
                                if let Some(fake_value) = get_fake_data(data_type, language) {
                                    let fake_value_for_logging = fake_value.clone();
                                    if output_format == "json" {
                                        fake_json_value = Some(json!(fake_value.to_string()));
                                    } else {
                                        fake_prost_value =
                                            Some(fake_value.into_prost_reflect_value(field_kind));
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
                                            fake_json_value = Some(json!(fake_value.to_string()));
                                        } else {
                                            fake_prost_value = Some(
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
                                // The field is repeated, but not a list
                                // Map fields are not supported yet
                                if output_format == "json" {
                                    fake_json_value = Some(JsonValue::Object(Default::default()));
                                } else {
                                    fake_prost_value = Some(Value::Map(Default::default()));
                                }
                                log::warn!(
                                    "Field '{}' has unsupported cardinality mapping: {:?}. Skipping.",
                                    field_name,
                                    is_map_field,
                                );
                            }
                        // end of `if let Some(fake_data_option) = fake_data.get(pb_options)`
                        } else {
                            log::debug!(
                                "  Field '{}' has no custom FakeDataFieldOption, populating with defaults.",
                                field_name
                            );
                        }
                    // end of `if let Some(pb_options) = field_proto.options.as_ref()`
                    } else {
                        log::debug!(
                            "  Field '{}' has no options on it, populating with defaults.",
                            field_name
                        );
                    }
                    if output_format == "json" {
                        if let Some(json_value) = fake_json_value {
                            json_message.insert(field_name.to_string(), json_value);
                        }
                    } else {
                        if let Some(prost_value) = fake_prost_value {
                            message.set_field(&field_descr, prost_value);
                        }
                    }
                }
                if output_format == "json" {
                    all_json_messages.push(json_message);
                } else {
                    all_messages.push(message);
                }
            }
            let mut generated_file_content: Vec<u8> = Vec::new();
            let mut generated_file = protobuf::plugin::code_generator_response::File::new();
            generated_file.set_name(output_name);
            if output_format == "json" {
                generated_file_content = to_vec_pretty(&all_json_messages).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to serialize to JSON: {}", e),
                    )
                })?;
                let stringified_content =
                    String::from_utf8(generated_file_content).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Failed to convert UTF-8 bytes to String: {}", e),
                        )
                    })?;
                generated_file.set_content(stringified_content);
            } else {
                for next_message in all_messages {
                    let msg_bytes = next_message.encode_to_vec();
                    generated_file_content.extend_from_slice(&msg_bytes);
                }
            }
            response.file.push(generated_file);
        }
    }

    // Encode the CodeGeneratorResponse and write to stdout
    let mut output_buffer = Vec::new();
    response.write_to_vec(&mut output_buffer)?; // Use write_to_vec() for rust-protobuf
    io::stdout().write_all(&output_buffer)?;

    Ok(())
}
