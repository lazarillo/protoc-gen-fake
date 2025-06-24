use protobuf::Message; // Import Message trait for parsing and serialization
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write}; // Import locales for fake data generation

// Corrected: CodeGeneratorRequest and CodeGeneratorResponse are in `protobuf::plugin`.
use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse};

// Common descriptor types are usually available directly from `protobuf::descriptor`.
use protobuf::descriptor::FileDescriptorProto;

#[path = "./gen/mod.rs"]
pub mod generated_protos;

use crate::generated_protos::fake_field::FakeDataFieldOption;
use crate::generated_protos::fake_field::exts::fake_data;

pub mod fake_data; // Import your fake data generation logic
use crate::fake_data::{FakeData, get_fake_data};

fn main() -> io::Result<()> {
    // Initialize logging for better debugging output
    env_logger::init(); // RUST_LOG=info, debug, or trace for more detail

    // Read the CodeGeneratorRequest from stdin
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    // Log raw stdin buffer at TRACE level (only shows with RUST_LOG=trace)
    log::trace!("Raw stdin buffer (hex): {:x?}", buffer);

    // Decode the request using protobuf::Message::parse_from_bytes
    let request = CodeGeneratorRequest::parse_from_bytes(&buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

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
    let key_files: HashSet<&String> = request.file_to_generate.iter().collect();

    let mut response = CodeGeneratorResponse::new(); // Use .new() for rust-protobuf messages

    // Iterate through the key file(s) to use for generating fake data
    // This is the main entry point for processing the request.
    for &filename in key_files.iter() {
        let file_descr = request
            .proto_file
            .iter()
            .find(|f| f.name.as_deref() == Some(filename))
            .unwrap_or_default();
        log::info!(
            "Processing file of interest: {}",
            file_descr.name.as_deref().unwrap_or_default()
        );
        for message_descr in file_descr.message_type.iter() {
            log::debug!(
                " Message: {}",
                message_descr.name.as_deref().unwrap_or_default()
            );
            for field_descr in message_descr.field.iter() {
                log::debug!(
                    "  Field: {}",
                    field_descr.name.as_deref().unwrap_or_default()
                );
                // Check if the field has options. In rust-protobuf 3.x, `options` is an Option<FieldOptions>.
                if let Some(options) = field_descr.options.as_ref() {
                    // Use `get` on the extension accessor directly, passing the `FieldOptions` reference.
                    if let Some(fake_data_option) = fake_data.get(options) {
                        let data_type = fake_data_option.data_type.as_str();
                        let language = fake_data_option.language.as_str();
                        if let Some(fake_value) = get_fake_data(data_type, language) {
                            log::info!(
                                "  Field '{}' - fake data type '{}' in '{}':  '{}'",
                                field_descr.name.as_deref().unwrap_or_default(),
                                data_type,
                                language,
                                fake_value
                            )
                        } else {
                            log::info!(
                                "  Field '{}' - requested fake data of type '{}' in '{}', but failed to generate it",
                                field_descr.name.as_deref().unwrap_or_default(),
                                data_type,
                                language,
                            )
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
