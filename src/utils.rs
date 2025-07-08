use prost::Message as _;
use prost_reflect::{DescriptorPool, Kind as ProstFieldKind, Value};
use prost_types::{FileDescriptorProto, FileDescriptorSet};
use protobuf::Message as _;
use protobuf::plugin::CodeGeneratorRequest;
use serde_json::{Value as JsonValue, to_value as to_json_value};
use std::collections::HashSet;
use std::io;
use std::path::PathBuf;

use crate::fake_data::get_fake_data;

pub enum DataOutputType {
    Json(JsonValue),
    Protobuf(Value),
}

pub fn parse_request_parameters(request: &CodeGeneratorRequest) -> (&str, PathBuf) {
    let mut output_format = "protobuf_binary"; // Default output format
    let mut output_path = PathBuf::from("."); // Default output path
    if let Some(params) = request.parameter.as_ref() {
        for param in params.split(',') {
            let key_val = param.split('=').collect::<Vec<&str>>();
            if key_val.len() == 2 {
                match key_val[0].to_lowercase().as_str() {
                    // Check if the parameter is 'format'
                    key if key.starts_with("form") => match key_val[1].to_lowercase().as_str() {
                        val if val.starts_with("proto") => {
                            output_format = "protobuf_binary";
                            log::info!(
                                "Parameter '{}' found, output format set to: {}",
                                params,
                                output_format
                            );
                        }
                        val if val.starts_with("json") => {
                            output_format = "json";
                            log::info!(
                                "Parameter '{}' found, output format set to: {}",
                                params,
                                output_format
                            );
                        }
                        _ => {
                            log::warn!(
                                "Unrecognized output format '{}', defaulting to '{}'",
                                key_val[1],
                                output_format
                            );
                        }
                    },
                    key if key.starts_with("out") => {
                        output_path = PathBuf::from(key_val[1]);
                        log::info!(
                            "Parameter '{}' found, output path set to: {}",
                            params,
                            output_path.to_str().unwrap_or("unknown")
                        );
                    }
                    _ => {
                        log::warn!(
                            "Unrecognized parameter '{}', expected 'format=<value>' or 'output_path=<value>'",
                            param
                        );
                    }
                }
            } else {
                log::warn!(
                    "Unrecognized parameter '{}', expected 'format=<value>' or 'output_path=<value>'",
                    param
                );
            }
        }
    } else {
        log::info!(
            "No parameters provided, using default output format and path: '{}' and '{}'",
            output_format,
            output_path.to_str().unwrap_or("unknown")
        );
    }
    // (output_format.to_string(), output_path)
    (output_format, output_path)
}

pub fn get_key_files(request: &CodeGeneratorRequest) -> HashSet<String> {
    request
        .file_to_generate
        .iter()
        .map(|file_str| file_str.clone())
        .collect()
}

pub fn get_runtime_descriptor_pool(request: &CodeGeneratorRequest) -> DescriptorPool {
    log::debug!("Building runtime descriptor pool, including user-provided files");
    let mut runtime_file_descriptor_set = FileDescriptorSet::default();
    runtime_file_descriptor_set.file = request
        .proto_file
        .iter()
        .map(|pb_fd| {
            let pb_bytes = pb_fd
                .write_to_bytes()
                .expect("Failed to serialize protobuf::descriptor::FileDescriptorProto");
            FileDescriptorProto::decode(pb_bytes.as_ref())
                .expect("Failed to decode prost_types::FileDescriptorProto")
        })
        .collect();

    let runtime_descriptor_pool =
        DescriptorPool::from_file_descriptor_set(runtime_file_descriptor_set)
            .map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to build runtime DescriptorPool: {}", err),
                )
            })
            .expect("Failed to build runtime DescriptorPool");
    log::debug!(
        "Runtime descriptor pool built successfully with the following {} files: {}",
        runtime_descriptor_pool.files().len(),
        runtime_descriptor_pool
            .files()
            .map(|f| f.name().to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    runtime_descriptor_pool
}

pub fn get_fake_data_output_value(
    data_type: &str,
    language: &str,
    output_format: &str,
    field_kind: &ProstFieldKind,
) -> DataOutputType {
    let possible_value = get_fake_data(data_type, language);
    match output_format {
        "json" => {
            // Note: this match is only needed for better logging information.
            match &possible_value {
                Some(fake_val) => {
                    log::info!(
                        "    Fake data type '{}' in '{}':  '{}'",
                        data_type,
                        language,
                        &fake_val.to_string()
                    );
                }
                None => {
                    log::info!(
                        "    No fake data found for type '{}' in '{}'",
                        data_type,
                        language
                    );
                }
            }
            DataOutputType::Json(
                to_json_value(&possible_value.unwrap_or_default()).unwrap_or_default(),
            )
        }
        _ => {
            // Note: this match is only needed for better logging information.
            match &possible_value {
                Some(fake_val) => {
                    log::info!(
                        "    Fake data type '{}' in '{}':  '{}'",
                        data_type,
                        language,
                        &fake_val.to_string()
                    );
                }
                None => {
                    log::info!(
                        "    No fake data found for type '{}' in '{}'",
                        data_type,
                        language
                    );
                }
            }
            DataOutputType::Protobuf(
                possible_value
                    .unwrap_or_default()
                    .into_prost_reflect_value(field_kind),
            )
        }
    }
}
