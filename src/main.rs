use once_cell::sync::Lazy;
use prost::Message;
use prost_reflect::{
    DescriptorPool, DynamicMessage, ExtensionDescriptor, FieldDescriptor, FileDescriptor,
    Kind as ProstFieldKind, MessageDescriptor, Value,
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
use serde::de; // Import Message trait for parsing and serialization
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
                    let field_kind = field_descr.kind();
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
                            if let Some(fake_value) = get_fake_data(data_type, language) {
                                log::info!(
                                    "  Field '{}' - fake data type '{}' in '{}':  '{}'",
                                    field_name,
                                    data_type,
                                    language,
                                    fake_value
                                )
                            } else {
                                log::info!(
                                    "  Field '{}' - requested fake data of type '{}' in '{}', but failed to generate it",
                                    field_name,
                                    data_type,
                                    language,
                                )
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
