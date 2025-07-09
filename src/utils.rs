//! Utility functions for for parsing protoc info and generating fake data.

use core::fmt;
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

/// Allow for representation of either JSON or Protobuf output in a single object.
pub enum DataOutputType {
    Json(JsonValue),
    Protobuf(Value),
}

/// Whether the output format is Protobuf or JSON.
#[derive(Debug, PartialEq)]
pub enum DesiredOutputFormat {
    Protobuf,
    Json,
}

impl fmt::Display for DesiredOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DesiredOutputFormat::Protobuf => write!(f, "Protobuf"),
            DesiredOutputFormat::Json => write!(f, "JSON"),
        }
    }
}

/// Parse the request from protoc to extract and return output format and output path.
pub fn parse_request_parameters(request: &CodeGeneratorRequest) -> (DesiredOutputFormat, PathBuf) {
    let mut output_format = DesiredOutputFormat::Protobuf; // Default output format
    let mut output_path = PathBuf::from("."); // Default output path
    if let Some(params) = request.parameter.as_ref() {
        for param in params.split(',') {
            let key_val = param.split('=').collect::<Vec<&str>>();
            if key_val.len() == 2 {
                match key_val[0].to_lowercase().as_str() {
                    // Check if the parameter is 'format'
                    key if key.starts_with("form") => match key_val[1].to_lowercase().as_str() {
                        val if val.starts_with("proto") => {
                            output_format = DesiredOutputFormat::Protobuf;
                            log::info!(
                                "Parameter '{}' found, output format set to: {}",
                                params,
                                output_format
                            );
                        }
                        val if val.starts_with("json") => {
                            output_format = DesiredOutputFormat::Json;
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

/// Simple extractor for the file names to generate from the request.
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
    output_format: &DesiredOutputFormat,
    field_kind: &ProstFieldKind,
) -> DataOutputType {
    let possible_value = get_fake_data(data_type, language);
    log::debug!(
        "get_fake_data_output_value: data_type='{}', language='{}', possible_value={:?}",
        data_type,
        language,
        possible_value
    );
    match output_format {
        DesiredOutputFormat::Json => {
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

#[cfg(test)]
mod utils_tests {
    use super::*; // Import everything from the outer scope

    /// Helper function to create a mock CodeGeneratorRequest for testing.
    fn create_mock_request(
        parameter: Option<&str>,
        files_to_generate: &[&str],
    ) -> CodeGeneratorRequest {
        let mut request = CodeGeneratorRequest::new();
        if let Some(param_str) = parameter {
            request.set_parameter(param_str.to_string());
        }
        // Corrected: Directly assign to the `file_to_generate` field, which is a Vec<String>.
        request.file_to_generate = files_to_generate.iter().map(|&s| s.to_string()).collect();
        request
    }

    /// Test `parse_request_parameters` with default (no) parameters.
    #[test]
    fn test_parse_request_parameters_default() {
        let request = create_mock_request(None, &[]);
        let (format, path) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Protobuf);
        assert_eq!(path, PathBuf::from("."));
    }

    /// Test `parse_request_parameters` with JSON format.
    #[test]
    fn test_parse_request_parameters_json_format() {
        let request = create_mock_request(Some("format=json"), &[]);
        let (format, path) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Json);
        assert_eq!(path, PathBuf::from("."));
    }

    /// Test `parse_request_parameters` with Protobuf format.
    #[test]
    fn test_parse_request_parameters_protobuf_format() {
        let request = create_mock_request(Some("format=protobuf"), &[]);
        let (format, path) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Protobuf);
        assert_eq!(path, PathBuf::from("."));
    }

    /// Test `parse_request_parameters` with a custom output path.
    #[test]
    fn test_parse_request_parameters_output_path() {
        let request = create_mock_request(Some("output_path=./my_output"), &[]);
        let (format, path) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Protobuf); // Default format
        assert_eq!(path, PathBuf::from("./my_output"));
    }

    /// Test `parse_request_parameters` with both format and output path.
    #[test]
    fn test_parse_request_parameters_all_options() {
        let request = create_mock_request(Some("format=json,output_path=/tmp/out"), &[]);
        let (format, path) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Json);
        assert_eq!(path, PathBuf::from("/tmp/out"));
    }

    /// Test `parse_request_parameters` with unrecognized parameters.
    #[test]
    fn test_parse_request_parameters_unrecognized() {
        // Unrecognized parameters should be ignored, and defaults should apply
        let request = create_mock_request(Some("unknown=value,format=json"), &[]);
        let (format, path) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Json);
        assert_eq!(path, PathBuf::from("."));
    }

    /// Test `get_key_files`.
    #[test]
    fn test_get_key_files() {
        let files = vec!["user.proto", "address.proto"];
        let request = create_mock_request(None, &files);
        let key_files = get_key_files(&request);
        let expected_files: HashSet<String> = files.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(key_files, expected_files);
    }

    /// Test `get_fake_data_output_value` for JSON output.
    #[test]
    fn test_get_fake_data_output_value_json() {
        let output = get_fake_data_output_value(
            "FirstName",
            "en",
            &DesiredOutputFormat::Json,
            &ProstFieldKind::String,
        );
        match output {
            DataOutputType::Json(value) => {
                assert!(value.is_string());
                assert!(!value.as_str().unwrap().is_empty());
            }
            _ => panic!("Expected Json output"),
        }
    }

    /// Test `get_fake_data_output_value` for Protobuf output.
    #[test]
    fn test_get_fake_data_output_value_protobuf() {
        let output = get_fake_data_output_value(
            "Age",
            "en",
            &DesiredOutputFormat::Protobuf,
            &ProstFieldKind::Int32,
        );
        match output {
            DataOutputType::Protobuf(value) => {
                // Removed assert!(value.is_i32()); as it doesn't exist
                let age = value.as_i32().unwrap(); // Directly unwrap the Option<i32>
                assert!(age >= 8 && age <= 90);
            }
            _ => panic!("Expected Protobuf output"),
        }
    }

    /// Test `get_fake_data_output_value` with a list type for JSON.
    #[test]
    fn test_get_fake_data_output_value_json_list() {
        let output = get_fake_data_output_value(
            "Words",
            "en",
            &DesiredOutputFormat::Json,
            &ProstFieldKind::String,
        );
        match output {
            DataOutputType::Json(value) => {
                log::info!("Mike says, output value: {:?}", value);
                assert!(value.is_array());
                let arr = value.as_array().unwrap();
                assert!(!arr.is_empty());
                assert!(arr.len() <= 10);
                for item in arr {
                    assert!(item.is_string());
                    assert!(!item.as_str().unwrap().is_empty());
                }
            }
            _ => panic!("Expected Json Array output"),
        }
    }

    /// Test `get_fake_data_output_value` with a list type for Protobuf.
    #[test]
    fn test_get_fake_data_output_value_protobuf_list() {
        let output = get_fake_data_output_value(
            "Words",
            "en",
            &DesiredOutputFormat::Protobuf,
            &ProstFieldKind::String,
        );
        match output {
            DataOutputType::Protobuf(value) => {
                assert!(value.as_list().is_some());
                let list = value.as_list().unwrap();
                assert!(!list.is_empty());
                assert!(list.len() <= 10);
                for item in list {
                    assert!(item.as_str().is_some());
                    assert!(!item.as_str().unwrap().is_empty());
                }
            }
            _ => panic!("Expected Protobuf List output"),
        }
    }
}
