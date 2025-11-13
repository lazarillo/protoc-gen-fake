//! # (Protoc) Gen Fake
//!
//! `protoc-gen-fake` is a custom plugin for `protoc` that uses annotation on the proto file schema
//! to generate a file with fake data well-aligned with the expected data types of the fields.
//!
//! It can generate fake data in either JSON or binary format, depending on the request parameters.
//!
//! ## Features
//!
//! - Generates files with fake data of many common types, such as names, addresses, dates, etc.,
//!   leveraging the Rust `fake` crate: https://docs.rs/fake/latest/fake/index.html
//! - Generates fake data in many different languages and regions, defaulting to English.
//!   All of the languages and types of fake data supported are listed at: <PROVIDE THE GITHUB URL HERE>
//! - Supports both JSON and binary output formats, with the default being binary.
//! - Allows owners of the proto files to specify the type of fake data to generate for each field.
//!
//! ## Usage
//!
//! After installing the plugin, you can use it with `protoc` like this:
//!
//! ```bash
//! protoc --fake_out my_output_dir --fake_opt output=my_output_dir -I proto ./proto/examples/user.proto
//! ```
//!
//! Breaking down the command:
//! --fake_out: This is where *some of the output* will be written. See `--fake_opt` below for details.
//!             (`protoc` uses the name of the plugin, after the `protoc-gen-`, as the option name.)
//! --fake_opt: This is used to pass options to the plugin. Unfortunately, since `protoc` is designed to
//!             enhance or alter **code generation**, the official output path at `--fake_out` can only
//!             be used to write text files, not binary files. Therefore, you need to additionally
//!             supply `--fake_opt output_path=<path>` to specify where the generated protobuf binary
//!             file(s) should be written. Note: the full flag is `output_path`, but `out` or `output`
//!             is sufficient.
//!
//! ## Options
//!
//! The following plugin-level options are supported:
//! - `output_path`: The path where the generated protobuf binary file(s) will be written.
//!                  This is passed as `--fake_opt output_path=<path>`.
//!                  Default: Current path (with respect to where `protoc` is run).
//! - `format`: The format of the output file(s). Can be either `json` or `protobuf`.
//!             This is passed as `--fake_opt format=<format>`.
//!             Default: `protobuf`.
//! - `language`: The language to use globally for generating all fake data. The language can
//!               also be specified on a per-field basis using the `fake_data` field option.
//!               The field-set language will override the global language, unless the
//!               force_language field option is set to `true`.
//!               This is passed as `--fake_opt language=<lang>`.
//!               Default: `en` (English).
//! - `force_language`: If set , the global language will be used for all fake data
//!                     generation, regardless of the field-set language.
//!                     This is passed as `--fake_opt force_language=<anything>`.
//!                     Default: `false`.
//!
//! It unfortunately needs to use both `prost` and `protobuf` crates to manage this, because`prost` does
//! not expose custom options, and `protobuf` does not support dynamic messages.

use base64::{engine::general_purpose, Engine as _};
use prost::Message;
use prost_reflect::{Cardinality, DynamicMessage, Value};
use protobuf::plugin::{
    CodeGeneratorRequest as PbCodeGeneratorRequest,
    CodeGeneratorResponse as PbCodeGeneratorResponse,
};
use protobuf::Message as PbMessage;
use rand::Rng;
use serde_json::{to_vec_pretty, Map as JsonMap, Value as JsonValue}; // Import serde_json for JSON handling
use std::cmp::max;
use std::collections::HashMap;
use std::fs;
pub mod utils; // Import utility functions for parsing request parameters
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;
use utils::{
    choose_language, get_fake_data_cardinality, get_key_files, parse_request_parameters, DataType,
    DesiredOutputFormat, SupportedLanguage,
}; // Import Path for file handling // Import locales for fake data generation // Import Lazy for static initialization

#[path = "./gen_protobuf/fake_field.rs"]
pub mod generated_proto; // Import the generated protobuf code for custom options

use crate::generated_proto::exts::fake_data;

pub mod fake_data;
use crate::utils::{get_fake_data_output_value, get_runtime_descriptor_pool};

fn main() -> io::Result<()> {
    //////////////////////////////////////////////////////////////////////////////////
    // All of the prep work before looping through the files                       ///
    //////////////////////////////////////////////////////////////////////////////////
    // Initialize logging for better debugging output
    env_logger::init(); // RUST_LOG=info, debug, or trace for more detail

    // Read the CodeGeneratorRequest from stdin
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    // Log raw stdin buffer at TRACE level (only shows with RUST_LOG=trace)
    log::trace!("Raw stdin buffer (hex): {:x?}", buffer);

    // Decode the request using protobuf::Message::parse_from_bytes
    let request = PbCodeGeneratorRequest::parse_from_bytes(&buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Log full request at DEBUG level (only shows with RUST_LOG=debug or lower)
    log::trace!("Full CodeGeneratorRequest received: {:#?}", request);

    // Parse the request parameters to get output format and output path, or populate defaults
    let (output_format, output_path, global_language, force_global_language) =
        parse_request_parameters(&request);

    // Get the set of files to generate (explicitly requested by protoc)
    let key_files = get_key_files(&request);
    log::debug!(
        "{} key file(s) to generate fake data over:\n{}",
        key_files.len(),
        key_files
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<&str>>()
            .join("\n")
    );

    // Build the runtime descriptor pool, including what is passed by the user
    let runtime_descriptor_pool = get_runtime_descriptor_pool(&request);

    // Build the empty response object to populate while iterating through the files
    let mut response = PbCodeGeneratorResponse::new(); // Use .new() for rust-protobuf messages

    // Create a random number generator object to use for generating fake data
    let mut rng = rand::rng();

    //////////////////////////////////////////////////////////////////////////////////
    // Iterate through the key file(s) to use for generating fake data             ///
    // This is the main entry point for processing the request.                    ///
    //////////////////////////////////////////////////////////////////////////////////
    for filename in key_files.iter() {
        if let Some(file_descr) = request
            .proto_file
            .iter()
            .find(|f| f.name.as_ref() == Some(filename))
        {
            log::info!("Processing file of interest: {}", filename);
            let output_file_path = Path::new(filename);
            let file_stem = output_file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let output_name = match output_format {
                DesiredOutputFormat::Json => format!("{}.json", file_stem),
                DesiredOutputFormat::Protobuf => format!("{}.b64", file_stem),
            };
            log::info!("    output file: {}", output_name);
            let mut binary_name = "output.bin".to_string();
            if output_format == DesiredOutputFormat::Protobuf {
                let binary_path = Path::new(&output_path)
                    .join(file_stem)
                    .with_extension("bin");
                binary_name = binary_path.to_string_lossy().to_string();
                log::info!("    binary path: {}", binary_name);
            }
            let mut all_messages: Vec<DynamicMessage> = Vec::new();
            let mut all_json_messages: Vec<JsonMap<String, JsonValue>> = Vec::new();
            let runtime_file_descriptor = runtime_descriptor_pool.get_file_by_name(filename)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "File '{}' not found in (`prost-reflect`) runtime descriptor pool. This should not happen if {} is valid",
                            filename, filename
                        ),
                    )
                })?;
            let mut overall_min_count: HashMap<String, Option<u32>> = HashMap::new();
            let mut overall_max_count: HashMap<String, Option<u32>> = HashMap::new();
            ////////////////////////////////////////////////////////////////////////////
            // Iterate through the messages in the key files                         ///
            ////////////////////////////////////////////////////////////////////////////
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

                    let mut fake_field_value: Option<DataType> = None;
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
                            get_fake_data_cardinality(
                                &field_descr,
                                &fake_data_option,
                                &mut overall_min_count,
                                &mut overall_max_count,
                            );
                            log::error!("\nFld: {} min:\n    {:?}", field_name, overall_min_count);
                            log::error!("\nFld: {} max:\n    {:?}", field_name, overall_max_count);
                            // TODO: I need to figure out a better way to handle messages:
                            // The message *count* should impact the number of times the message
                            // *underlying fields* are generated, but it needs to be repeated as
                            // part of the entire message. Ie, if the "phone_numbers" field
                            // has a min_count of 2, then the message should be repeated twice,
                            // meaning that there is a *single message* with *both* the type and
                            // actual number, and then there is another *single message* with
                            // the type and actual number, and so on.
                            // I *think that* I'm now creating it so that there are two types and
                            // two numbers, not two messages with one each.

                            let data_type = fake_data_option.data_type.as_str();
                            let field_level_language = match SupportedLanguage::from_str(
                                fake_data_option.language.as_str(),
                            ) {
                                Ok(lang) => lang,
                                Err(_) => SupportedLanguage::Default,
                            };
                            let language = &choose_language(
                                &field_level_language,
                                &global_language,
                                force_global_language,
                            );
                            let min_count = max(fake_data_option.min_count, 0);
                            let max_count = max(fake_data_option.max_count, max(min_count, 1));
                            if is_list_field {
                                // Generate multiple values for list fields
                                let mut json_values = Vec::new();
                                let mut repeated_values = Vec::new();

                                let num_values = rng.random_range(min_count..=max_count);
                                if num_values == 0 {
                                    log::info!(
                                        "Field '{}' has a minimum of {} values and no value was generated.",
                                        field_name,
                                        min_count,
                                    );
                                }
                                for _ in 0..num_values {
                                    let fake_value = get_fake_data_output_value(
                                        data_type,
                                        language,
                                        &output_format,
                                        field_kind,
                                    );
                                    match fake_value {
                                        DataType::Json(json_value) => {
                                            json_values.push(json_value);
                                        }
                                        DataType::Protobuf(proto_value) => {
                                            repeated_values.push(proto_value);
                                        }
                                    }
                                }
                                fake_field_value = match output_format {
                                    DesiredOutputFormat::Json => {
                                        Some(DataType::Json(JsonValue::Array(json_values)))
                                    }
                                    _ => Some(DataType::Protobuf(Value::List(repeated_values))),
                                };
                            } else if field_cardinality == Cardinality::Required {
                                // For required fields, we generate a single value
                                fake_field_value = Some(get_fake_data_output_value(
                                    data_type,
                                    language,
                                    &output_format,
                                    field_kind,
                                ));
                            } else if field_cardinality == Cardinality::Optional {
                                // For optional fields, we generate a single value or leave it unset
                                // 0.5 gives 50/50 chance of generating a value, which seems too
                                // low, so we increase it to 0.6 to fill out optionals more often
                                let should_generate_value = rng.random_bool(0.6);
                                if should_generate_value || min_count > 0 {
                                    fake_field_value = Some(get_fake_data_output_value(
                                        data_type,
                                        language,
                                        &output_format,
                                        field_kind,
                                    ));
                                } else {
                                    fake_field_value = None::<DataType>; // Leave it unset
                                    log::info!(
                                        "Field '{}' is optional and no value was generated.",
                                        field_name
                                    );
                                }
                            } else {
                                // The field is repeated, but not a list
                                // Map fields are not supported yet
                                fake_field_value = match output_format {
                                    DesiredOutputFormat::Json => {
                                        Some(DataType::Json(JsonValue::Object(Default::default())))
                                        // Placeholder for JSON
                                    }
                                    _ => {
                                        Some(DataType::Protobuf(Value::Map(Default::default())))
                                        // Placeholder for Protobuf
                                    }
                                };
                                log::warn!(
                                    "Field '{}' has unsupported cardinality mapping: {:?}. Using defaults.",
                                    field_name,
                                    is_map_field,
                                );
                            }
                        // end of `if let Some(fake_data_option) = fake_data.get(pb_options)`
                        } else {
                            log::debug!(
                                "  Field '{}' has no custom FakeDataFieldOption, skipping.",
                                field_name
                            );
                        }
                    // end of `if let Some(pb_options) = field_proto.options.as_ref()`
                    } else {
                        log::debug!("  Field '{}' has no options on it, skipping.", field_name);
                    }
                    match fake_field_value {
                        Some(DataType::Json(fake_value)) => {
                            // Insert the JSON field value into the json_message
                            json_message.insert(field_name.to_string(), fake_value);
                        }
                        Some(DataType::Protobuf(fake_value)) => {
                            log::error!(
                                "AM I GETTING THIS!!! Message:\n  '{}'\nis setting field\n  '{:?}' with value\n  '{:?}'",
                                message,
                                field_descr,
                                fake_value
                            );
                            // Insert the Protobuf field value into the message
                            message.set_field(&field_descr, fake_value);
                        }
                        None => {
                            // If no fake data was generated, we can skip setting the field
                            log::trace!(
                                "    Field '{}' had no fake data generation requested.",
                                field_name
                            );
                        }
                    }
                }
                match output_format {
                    DesiredOutputFormat::Json => {
                        all_json_messages.push(json_message);
                    }
                    _ => {
                        all_messages.push(message);
                    }
                }
            } // end of message iteration
            let mut generated_file_content: Vec<u8> = Vec::new();
            let mut generated_file = protobuf::plugin::code_generator_response::File::new();
            generated_file.set_name(output_name);
            match output_format {
                DesiredOutputFormat::Json => {
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
                    log::info!(
                        "Writing JSON to the following path: {}",
                        generated_file.name.clone().unwrap_or("unknown".to_string())
                    );
                }
                DesiredOutputFormat::Protobuf => {
                    for next_message in all_messages {
                        let msg_bytes = next_message.encode_to_vec();
                        generated_file_content.extend_from_slice(&msg_bytes);
                    }
                    fs::write(&binary_name, &generated_file_content)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                    log::info!("Writing binary to the following path: {}", binary_name);
                    let file_content_string =
                        general_purpose::STANDARD.encode(&generated_file_content);
                    generated_file.set_content(file_content_string);
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
