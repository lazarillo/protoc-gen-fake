//! # (Protoc) Gen Fake
//!
//! `protoc-gen-fake` is a custom plugin for `protoc` that uses annotation on the proto file schema
//! to generate a file with fake data well-aligned with the expected data types of the fields.
//!
//! It generates fake data in binary format.
//!
//! ## Features
//!
//! - Generates files with fake data of many common types, such as names, addresses, dates, etc.,
//!   leveraging the Rust `fake` crate: https://docs.rs/fake/latest/fake/index.html
//! - Generates fake data in many different languages and regions, defaulting to English.
//!   All of the languages and types of fake data supported are listed at: <PROVIDE THE GITHUB URL HERE>
//! - Supports binary output format, with Base64 encoding for `protoc` compatibility.
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
use prost_reflect::{Cardinality, DynamicMessage, MessageDescriptor, Value};
use protobuf::descriptor::FileDescriptorProto;
use protobuf::plugin::{
    CodeGeneratorRequest as PbCodeGeneratorRequest,
    CodeGeneratorResponse as PbCodeGeneratorResponse,
};
use protobuf::Message as PbMessage;
use rand::Rng;
use std::cmp::max;
use std::fs;
pub mod utils; // Import utility functions for parsing request parameters
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;
use utils::{
    choose_language, find_message_proto, get_key_files, parse_request_parameters, DataType,
    DesiredOutputFormat, OutputEncoding, SupportedLanguage,
}; // Import Path for file handling // Import locales for fake data generation // Import Lazy for static initialization

#[path = "./gen_protobuf/fake_field.rs"]
pub mod generated_proto; // Import the generated protobuf code for custom options

use crate::generated_proto::exts::{fake_data, fake_msg};

pub mod fake_data;
use crate::utils::{get_fake_data_output_value, get_runtime_descriptor_pool};

fn main() -> io::Result<()> {
    //////////////////////////////////////////////////////////////////////////////////
    // All of the prep work before looping through the files                       ///
    //////////////////////////////////////////////////////////////////////////////////
    // Initialize logging for better debugging output
    env_logger::init(); // RUST_LOG=info, debug, or trace for more detail

    // Handle command line arguments like --version or --help
    for arg in std::env::args() {
        match arg.as_str() {
            "--version" | "-V" => {
                println!("protoc-gen-fake {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--help" | "-h" => {
                println!("protoc-gen-fake: Protocol Buffer Fake Data Generator");
                println!("Usage: protoc --plugin=protoc-gen-fake=<path_to_plugin> --fake_out=. --fake_opt=<options> -I <proto_path> <proto_file>");
                println!("       Or: protoc-gen-fake [--version | -V] [--help | -h]");
                println!("\nFor detailed usage with protoc, see the project README.");
                return Ok(());
            }
            _ => {}
        }
    }

    // Debugging: Log all arguments
    for (i, arg) in std::env::args().enumerate() {
        log::debug!("Arg {}: {}", i, arg);
    }

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
    let (output_format, output_path, global_language, force_global_language, output_encoding) =
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
            let output_name = format!("{}.b64", file_stem);

            // Determine the actual binary file path for fs::write
            let binary_name: String;
            let user_supplied_output_path_str = output_path.to_string_lossy();

            if user_supplied_output_path_str == "." {
                // If output_path is the default ".", we output to the current directory
                // with the proto's file stem and a .bin extension.
                binary_name = Path::new(file_stem).with_extension("bin").to_string_lossy().to_string();
            } else if user_supplied_output_path_str.ends_with(".bin") {
                // If the user provided a full path ending in .bin, use it as is.
                binary_name = user_supplied_output_path_str.to_string();
            } else {
                // If the user provided a path that doesn't end in .bin, treat it as a directory
                // and append the proto's file stem with a .bin extension.
                binary_name = Path::new(user_supplied_output_path_str.as_ref())
                                .join(file_stem)
                                .with_extension("bin")
                                .to_string_lossy()
                                .to_string();
            }
            log::debug!("    output file (Base64): {}", output_name); // For protoc response
            log::warn!("    binary output path: {}", binary_name); // For actual file write
            let mut all_messages: Vec<DynamicMessage> = Vec::new();
            let runtime_file_descriptor = runtime_descriptor_pool
                .get_file_by_name(filename)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "File '{}' not found in (`prost-reflect`) runtime descriptor pool. This should not happen if {} is valid",
                            filename, filename
                        ),
                    )
                })?;
            ////////////////////////////////////////////////////////////////////////////
            // Iterate through the messages in the key files                         ///
            ////////////////////////////////////////////////////////////////////////////
            let mut message_processed = false;
            for message_descr in runtime_file_descriptor.messages() {
                let message_name = message_descr.name();
                let message_proto = find_message_proto(file_descr, message_name)
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("MessageProto for '{}' not found.", message_name),
                        )
                    })?;

                let should_include_message = if let Some(pb_options) = message_proto.options.as_ref() {
                    fake_msg.get(pb_options).map_or(false, |opt| opt.include)
                } else {
                    false
                };

                if should_include_message {
                    if message_processed {
                        log::warn!(
                            "Skipping message '{}' because another message with `(gen_fake.fake_msg).include = true` was already processed.",
                            message_name
                        );
                        continue;
                    }
                    message_processed = true;
                    log::debug!(" Message: {}", message_name);
                    let mut message = DynamicMessage::new(message_descr.clone());
                    for field_descr in message_descr.fields() {
                        let field_name = field_descr.name();
                        let field_kind = &field_descr.kind();
                        let is_list_field = field_descr.is_list();
                        let field_cardinality = field_descr.cardinality();

                        let message_proto = find_message_proto(file_descr, message_name)
                            .ok_or_else(|| {
                                io::Error::new(
                                    io::ErrorKind::NotFound,
                                    format!("MessageProto for '{}' not found.", message_name),
                                )
                            })?;

                        let field_proto = match message_proto
                            .field
                            .iter()
                            .find(|f| f.name.as_deref() == Some(field_name))
                        {
                            Some(fp) => fp,
                            None => continue, // Skip if field not found in proto
                        };

                        if let Some(pb_options) = field_proto.options.as_ref() {
                            if let Some(fake_data_option) = fake_data.get(pb_options) {
                                let min_count = max(fake_data_option.min_count, 0);
                                let max_count = max(fake_data_option.max_count, max(min_count, 1));
                                let data_type = fake_data_option.data_type.as_str();
                                let field_level_language =
                                    SupportedLanguage::from_str(fake_data_option.language.as_str())
                                        .unwrap_or_default();
                                let language = &choose_language(
                                    &field_level_language,
                                    &global_language,
                                    force_global_language,
                                );

                                if let prost_reflect::Kind::Message(nested_message_descr) = field_kind.clone() {
                                    // RECURSIVE CALL FOR NESTED MESSAGE
                                    if is_list_field {
                                        let mut nested_messages = Vec::new();
                                        let num_to_create = rng.random_range(min_count..=max_count);
                                        for _ in 0..num_to_create {
                                            let nested_msg = generate_fake_message_field(
                                                &nested_message_descr,
                                                file_descr,
                                                &mut rng,
                                                &output_format,
                                                &global_language,
                                                force_global_language,
                                                &runtime_descriptor_pool,
                                            )?;
                                            nested_messages.push(Value::Message(nested_msg));
                                        }
                                        message.set_field(&field_descr, Value::List(nested_messages));
                                    } else {
                                        let nested_msg = generate_fake_message_field(
                                            &nested_message_descr,
                                            file_descr,
                                            &mut rng,
                                            &output_format,
                                            &global_language,
                                            force_global_language,
                                            &runtime_descriptor_pool,
                                        )?;
                                        message.set_field(&field_descr, Value::Message(nested_msg));
                                    }
                                } else {
                                    // Handle primitive types
                                    let mut fake_field_value: Option<DataType> = None;
                                    if is_list_field {
                                        let mut repeated_values = Vec::new();
                                        let num_values = rng.random_range(min_count..=max_count);
                                        for _ in 0..num_values {
                                            let DataType::Protobuf(proto_value) =
                                                get_fake_data_output_value(
                                                    data_type,
                                                    language,
                                                    &output_format,
                                                    field_kind,
                                                );
                                            repeated_values.push(proto_value);
                                        }
                                        fake_field_value =
                                            Some(DataType::Protobuf(Value::List(repeated_values)));
                                    } else if field_cardinality == Cardinality::Required {
                                        fake_field_value = Some(get_fake_data_output_value(
                                            data_type,
                                            language,
                                            &output_format,
                                            field_kind,
                                        ));
                                    } else if field_cardinality == Cardinality::Optional {
                                        if rng.random_bool(0.6) || min_count > 0 {
                                            fake_field_value = Some(get_fake_data_output_value(
                                                data_type,
                                                language,
                                                &output_format,
                                                field_kind,
                                            ));
                                        }
                                    }

                                    if let Some(DataType::Protobuf(value)) = fake_field_value {
                                        message.set_field(&field_descr, value);
                                    }
                                }
                            }
                        }
                    }
                    match output_format {
                        _ => {
                            all_messages.push(message);
                        }
                    }
                }
            } // end of message iteration
            let mut generated_file_content: Vec<u8> = Vec::new();
            let mut generated_file = protobuf::plugin::code_generator_response::File::new();
            generated_file.set_name(output_name);
            match output_format {
                DesiredOutputFormat::Protobuf => {
                    for next_message in all_messages {
                        let msg_bytes = next_message.encode_to_vec();
                        generated_file_content.extend_from_slice(&msg_bytes);
                    }

                    if output_encoding == OutputEncoding::Binary
                        || output_encoding == OutputEncoding::Both
                    {
                        fs::write(&binary_name, &generated_file_content)
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                        log::info!("Writing binary to the following path: {}", binary_name);
                    }

                    if output_encoding == OutputEncoding::Base64
                        || output_encoding == OutputEncoding::Both
                    {
                        let file_content_string =
                            general_purpose::STANDARD.encode(&generated_file_content);
                        generated_file.set_content(file_content_string);
                        response.file.push(generated_file);
                    }
                }
            }
        }
    }


    // Encode the CodeGeneratorResponse and write to stdout
    let mut output_buffer = Vec::new();
    response.write_to_vec(&mut output_buffer)?; // Use write_to_vec() for rust-protobuf
    io::stdout().write_all(&output_buffer)?;

    Ok(())
}

/// Recursively generates a DynamicMessage for a given message descriptor,
/// populating its fields with fake data based on options.
fn generate_fake_message_field(
    message_descr: &MessageDescriptor,
    file_descr: &FileDescriptorProto, // Need to pass this down for protobuf options
    rng: &mut impl Rng,
    output_format: &DesiredOutputFormat,
    global_language: &SupportedLanguage,
    force_global_language: bool,
    runtime_descriptor_pool: &prost_reflect::DescriptorPool, // Need this for nested message descriptors
) -> io::Result<DynamicMessage> {
    let mut message = DynamicMessage::new(message_descr.clone());

    for field_descr in message_descr.fields() {
        let field_name = field_descr.name();
        let field_cardinality = field_descr.cardinality();
        let field_kind = &field_descr.kind();
        let is_list_field = field_descr.is_list();

        let mut fake_field_value: Option<DataType> = None;

        // Find the corresponding protobuf FieldDescriptorProto to get custom options
        // This is necessary because prost-reflect's FieldDescriptor doesn't expose custom options directly.
        let message_proto = find_message_proto(file_descr, message_descr.name())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("MessageProto for '{}' not found.", message_descr.name()),
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

        if let Some(pb_options) = field_proto.options.as_ref() {
            if let Some(fake_data_option) = fake_data.get(pb_options) {
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

                if let prost_reflect::Kind::Message(nested_message_descr) = field_kind.clone() {
                    // RECURSIVE CALL FOR NESTED MESSAGE
                    if is_list_field {
                        let mut nested_messages = Vec::new();
                        let num_values = rng.random_range(min_count..=max_count);
                        for _ in 0..num_values {
                            let nested_msg = generate_fake_message_field(
                                &nested_message_descr,
                                file_descr,
                                rng,
                                output_format,
                                global_language,
                                force_global_language,
                                runtime_descriptor_pool,
                            )?;
                            nested_messages.push(nested_msg);
                        }
                        message.set_field(&field_descr, Value::List(nested_messages.into_iter().map(Value::Message).collect()));
                    } else {
                        let nested_msg = generate_fake_message_field(
                            &nested_message_descr,
                            file_descr,
                            rng,
                            output_format,
                            global_language,
                            force_global_language,
                            runtime_descriptor_pool,
                        )?;
                        message.set_field(&field_descr, Value::Message(nested_msg));
                    }
                } else {
                    // Existing logic for primitive types
                    if is_list_field {
                        // Generate multiple values for list fields
                        let mut repeated_values = Vec::new();
                        let num_values = rng.random_range(min_count..=max_count);
                        for _ in 0..num_values {
                            let fake_value = get_fake_data_output_value(
                                data_type,
                                language,
                                output_format,
                                field_kind,
                            );
                            let DataType::Protobuf(proto_value) = fake_value;
                            repeated_values.push(proto_value);
                        }
                        fake_field_value = Some(DataType::Protobuf(Value::List(repeated_values)));
                    } else if field_cardinality == Cardinality::Required {
                        fake_field_value = Some(get_fake_data_output_value(
                            data_type,
                            language,
                            output_format,
                            field_kind,
                        ));
                    } else if field_cardinality == Cardinality::Optional {
                        let should_generate_value = rng.random_bool(0.6);
                        if should_generate_value || min_count > 0 {
                            fake_field_value = Some(get_fake_data_output_value(
                                data_type,
                                language,
                                output_format,
                                field_kind,
                            ));
                        }
                    }
                }
            } else {
                log::debug!(
                    "  Field '{}' has no custom FakeDataFieldOption, skipping.",
                    field_name
                );
            }
        } else {
            log::debug!("  Field '{}' has no options on it, skipping.", field_name);
        }

        // Only set the field if it's a primitive type and fake_field_value was generated
        if !matches!(field_kind, prost_reflect::Kind::Message(_)) {
            if let Some(DataType::Protobuf(fake_value)) = fake_field_value {
                message.set_field(&field_descr, fake_value);
            }
        }
    }
    Ok(message)
}

