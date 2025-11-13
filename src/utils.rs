//! Utility functions for for parsing protoc info and generating fake data.

use core::fmt;
use prost::Message as _;
use prost_reflect::MessageDescriptor;
use prost_reflect::{
    DescriptorPool, DynamicMessage, FieldDescriptor, Kind as ProstFieldKind, Value,
};
use prost_types::{
    FieldDescriptorProto as ProstFieldDescriptor, FileDescriptorProto as ProstFileDescriptor,
    FileDescriptorSet,
};
use protobuf::descriptor::FieldDescriptorProto;
use protobuf::plugin::CodeGeneratorRequest;
use protobuf::{descriptor::FileDescriptorProto, Message as _};
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;

use crate::fake_data::get_fake_data;
use crate::generated_proto::FakeDataFieldOption;

/// Allow for representation of either JSON or Protobuf output in a single object.
#[derive(Debug)]
pub enum DataType {
    Json(JsonValue),
    Protobuf(Value),
}

pub enum DataMsg {
    JsonMsg(JsonMap<String, JsonValue>),
    ProtoMsg(DynamicMessage),
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

#[derive(PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum SupportedLanguage {
    AR_SA,   // Arabic (Saudi Arabia)
    DE_DE,   // German (Germany)
    Default, // Allow for default, which means it can be overridden by the field
    EN,      // English
    FR_FR,   // French (France)
    IT_IT,   // Italian (Italy)
    JA_JP,   // Japanese (Japan)
    PT_BR,   // Portuguese (Brazil)
    PT_PT,   // Portuguese (Portugal)
    ZH_CN,   // Simplified Chinese (China)
    ZH_TW,   // Traditional Chinese (Taiwan)
}

impl Default for SupportedLanguage {
    fn default() -> Self {
        SupportedLanguage::Default
    }
}

impl fmt::Display for SupportedLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupportedLanguage::AR_SA => write!(f, "Arabic (Saudi Arabia)"),
            SupportedLanguage::DE_DE => write!(f, "German (Germany)"),
            SupportedLanguage::Default => write!(f, "Default (no overriding language)"),
            SupportedLanguage::EN => write!(f, "English"),
            SupportedLanguage::FR_FR => write!(f, "French (France)"),
            SupportedLanguage::IT_IT => write!(f, "Italian (Italy)"),
            SupportedLanguage::JA_JP => write!(f, "Japanese (Japan)"),
            SupportedLanguage::PT_BR => write!(f, "Portuguese (Brazil)"),
            SupportedLanguage::PT_PT => write!(f, "Portuguese (Portugal)"),
            SupportedLanguage::ZH_CN => write!(f, "Simplified Chinese (China)"),
            SupportedLanguage::ZH_TW => write!(f, "Traditional Chinese (Taiwan)"),
        }
    }
}

// Define a custom error type for when conversion fails
#[derive(Debug, PartialEq)]
pub enum ParseLanguageError {
    InvalidLanguage,
}

impl fmt::Display for ParseLanguageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseLanguageError::InvalidLanguage => {
                write!(f, "Language not yet supported or invalid")
            }
        }
    }
}

// Implement the standard FromStr trait for SupportedLanguage
impl FromStr for SupportedLanguage {
    type Err = ParseLanguageError; // Specify custom error
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "ar_sa" | "ar" | "arabic" => Ok(SupportedLanguage::AR_SA),
            "de_de" | "de" | "german" => Ok(SupportedLanguage::DE_DE),
            "en" | "english" => Ok(SupportedLanguage::EN),
            "fr_fr" | "fr" | "french" => Ok(SupportedLanguage::FR_FR),
            "it_it" | "it" | "italian" => Ok(SupportedLanguage::IT_IT),
            "ja_jp" | "ja" | "japanese" => Ok(SupportedLanguage::JA_JP),
            "pt_br" | "pt" | "brazilian_portuguese" => Ok(SupportedLanguage::PT_BR),
            "pt_pt" | "portuguese" => Ok(SupportedLanguage::PT_PT),
            "zh_cn" | "zh" | "simplified_chinese" | "chinese" => Ok(SupportedLanguage::ZH_CN),
            "zh_tw" | "traditional_chinese" | "taiwanese" => Ok(SupportedLanguage::ZH_TW),
            _ => Err(ParseLanguageError::InvalidLanguage),
        }
    }
}
// To use `Debug` for `println!("{:?}", ...)`
impl std::fmt::Debug for SupportedLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedLanguage::AR_SA => write!(f, "SupportedLanguage::AR_SA"),
            SupportedLanguage::DE_DE => write!(f, "SupportedLanguage::DE_DE"),
            SupportedLanguage::Default => write!(f, "SupportedLanguage::Default"),
            SupportedLanguage::EN => write!(f, "SupportedLanguage::EN"),
            SupportedLanguage::FR_FR => write!(f, "SupportedLanguage::FR_FR"),
            SupportedLanguage::IT_IT => write!(f, "SupportedLanguage::IT_IT"),
            SupportedLanguage::JA_JP => write!(f, "SupportedLanguage::JA_JP"),
            SupportedLanguage::PT_BR => write!(f, "SupportedLanguage::PT_BR"),
            SupportedLanguage::PT_PT => write!(f, "SupportedLanguage::PT_PT"),
            SupportedLanguage::ZH_CN => write!(f, "SupportedLanguage::ZH_CN"),
            SupportedLanguage::ZH_TW => write!(f, "SupportedLanguage::ZH_TW"),
        }
    }
}

/// Parse the request from protoc to extract and return output format and output path.
pub fn parse_request_parameters(
    request: &CodeGeneratorRequest,
) -> (DesiredOutputFormat, PathBuf, SupportedLanguage, bool) {
    let mut output_format = DesiredOutputFormat::Protobuf; // Default output format
    let mut output_path = PathBuf::from("."); // Default output path
                                              // No overriding default language, let the fields decide language
    let mut language = SupportedLanguage::Default;
    let mut force_global_language = false; // Default to not forcing global language override
    if let Some(params) = request.parameter.as_ref() {
        for param in params.split(',') {
            let key_val = param.split('=').collect::<Vec<&str>>();
            if key_val.len() == 2 {
                match key_val[0].to_lowercase().as_str() {
                    // Check if the parameter is 'format'
                    key if key.starts_with("form") => match key_val[1].to_lowercase().as_str() {
                        val if val.starts_with("proto") | val.starts_with("bin") => {
                            output_format = DesiredOutputFormat::Protobuf;
                            log::debug!(
                                "Parameter '{}' found, output format set to: {}",
                                param,
                                output_format
                            );
                        }
                        val if val.starts_with("json") => {
                            output_format = DesiredOutputFormat::Json;
                            log::debug!(
                                "Parameter '{}' found, output format set to: {}",
                                param,
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
                    key if key.starts_with("forc") => {
                        force_global_language = true;
                        log::debug!(
                            "Parameter '{}' found, forcing global language override set to: {}",
                            param,
                            force_global_language
                        );
                    }
                    key if key.starts_with("out") => {
                        output_path = PathBuf::from(key_val[1]);
                        log::debug!(
                            "Parameter '{}' found, output path set to: {}",
                            param,
                            output_path.to_str().unwrap_or("unknown")
                        );
                    }
                    key if key.starts_with("lang") => {
                        let found_language = SupportedLanguage::from_str(key_val[1]);
                        language = match found_language {
                            Ok(lang) => lang,
                            Err(err) => {
                                log::warn!(
                                    "Provided language '{}': {}, defaulting to no overriding language",
                                    key_val[1],
                                    err
                                );
                                SupportedLanguage::Default
                            }
                        };
                        log::debug!("Parameter '{}' found, language set to: {}", param, language);
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
        log::debug!(
            "No parameters provided, using default output format, language, and path: '{}', '{}' and '{}'",
            output_format,
            language,
            output_path.to_str().unwrap_or("unknown")
        );
    }
    // (output_format.to_string(), output_path)
    (output_format, output_path, language, force_global_language)
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
            ProstFileDescriptor::decode(pb_bytes.as_ref())
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
        "Runtime descriptor pool built using the following {} files:\n{}",
        runtime_descriptor_pool.files().len(),
        runtime_descriptor_pool
            .files()
            .map(|f| f.name().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
    runtime_descriptor_pool
}

pub fn choose_language(
    field_language: &SupportedLanguage,
    global_language: &SupportedLanguage,
    force_global_language: bool,
) -> SupportedLanguage {
    if force_global_language {
        log::debug!(
            "Forcing language override to '{}'",
            global_language.to_string()
        );
        global_language.clone()
    } else if field_language != &SupportedLanguage::Default {
        log::debug!(
            "Using field-level language '{}', ignoring global language",
            field_language.to_string()
        );
        field_language.clone()
    } else {
        log::debug!(
            "Using language '{}', no field-level language specified",
            global_language.to_string()
        );
        global_language.clone()
    }
}

pub fn get_fake_data_cardinality(
    field_descr: &FieldDescriptor,
    fake_data_option: &FakeDataFieldOption,
    min_count: &mut HashMap<String, Option<u32>>,
    max_count: &mut HashMap<String, Option<u32>>,
) -> () {
    let new_min_count: Option<u32> = match fake_data_option.min_count.try_into() {
        Ok(0) => None,
        Ok(val) => Some(val),
        Err(_) => None,
    };
    let new_max_count: Option<u32> = match fake_data_option.max_count.try_into() {
        Ok(0) => None,
        Ok(val) => Some(val),
        Err(_) => None,
    };
    match field_descr.kind() {
        // ProstFieldKind::Message(msg_descr) => {
        //     // For a message field, we want to generate fake data based upon the attributes
        //     // *within* the message, so we need to place cardinality on the message fields
        //     msg_descr.fields().into_iter().for_each(|msg_field| {
        //         min_count
        //             .entry(msg_field.full_name().to_string())
        //             .and_modify(|new_min| {
        //                 if let Some(val) = new_min {
        //                     if let Some(v) = new_min_count {
        //                         *new_min = Some(*val * v);
        //                     }
        //                 } else {
        //                     *new_min = new_min_count;
        //                 }
        //             })
        //             .or_insert(new_min_count);
        //         max_count
        //             .entry(msg_field.full_name().to_string())
        //             .and_modify(|new_max| {
        //                 if let Some(val) = new_max {
        //                     if let Some(v) = new_max_count {
        //                         *new_max = Some(*val * v);
        //                     }
        //                 } else {
        //                     *new_max = new_max_count;
        //                 }
        //             })
        //             .or_insert(new_max_count);
        //     });
        // }
        _ => {
            min_count
                .entry(field_descr.full_name().to_string())
                .insert_entry(new_min_count);
            max_count
                .entry(field_descr.full_name().to_string())
                .insert_entry(new_max_count);
        }
    }
}

pub fn iterate_message(
    msg_descr: &MessageDescriptor,    // from prost_reflect::descriptor
    file_descr: &FileDescriptorProto, // from protobuf::descriptor
    output_format: &DesiredOutputFormat,
) -> DataMsg {
    let mut output_msg = match output_format {
        DesiredOutputFormat::Json => DataMsg::JsonMsg(JsonMap::<String, JsonValue>::new()),
        _ => DataMsg::ProtoMsg(DynamicMessage::new(msg_descr.clone())),
    };
    for field_descr in msg_descr.fields() {
        let field_msg = match field_descr.kind() {
            ProstFieldKind::Message(inner_msg) => {
                iterate_message(&inner_msg, file_descr, output_format)
            }
            _ => DataMsg::JsonMsg(JsonMap::new()),
        };
    }
    output_msg
}

pub fn get_field_descriptor<'a>(
    file_descr: &'a FileDescriptorProto,
    msg_name: &'a str,
    field_name: &'a str,
) -> &'a FieldDescriptorProto {
    let msg_proto = file_descr
        .message_type
        .iter()
        .find(|msg| msg.name.as_deref() == Some(msg_name))
        .expect(format!("Protobuf DescriptorProto for '{}' not found", msg_name).as_str());
    msg_proto
        .field
        .iter()
        .find(|fld| fld.name.as_deref() == Some(field_name))
        .expect(
            format!(
                "Protobuf FieldDescriptorProto for '{}' not found",
                field_name
            )
            .as_str(),
        )
}

pub fn get_fake_data_output_value(
    data_type: &str,
    language: &SupportedLanguage,
    output_format: &DesiredOutputFormat,
    field_kind: &ProstFieldKind,
) -> DataType {
    let possible_value = get_fake_data(data_type, language);
    match output_format {
        DesiredOutputFormat::Json => {
            // Note: this match is only needed for better logging information.
            match &possible_value {
                Some(fake_val) => {
                    log::info!(
                        "    Fake data type '{}' in '{}': '{}'",
                        data_type,
                        language,
                        &fake_val.to_string()
                    );
                }
                None => {
                    log::warn!(
                        "    No fake data found for type '{}' in '{}'",
                        data_type,
                        language
                    );
                }
            }
            DataType::Json(possible_value.unwrap_or_default().into_json_value())
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
                    log::warn!(
                        "    No fake data found for type '{}' in '{}'",
                        data_type,
                        language
                    );
                }
            }
            DataType::Protobuf(
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

    use env_logger;
    use once_cell::sync::Lazy; // Import Lazy for one-time initialization // Import env_logger

    // Initialize the logger once for all tests in this file's `utils_tests` module.
    // This static variable ensures the initialization happens only once
    // when this test module is loaded.
    static INIT_LOGGER_UTILS: Lazy<()> = Lazy::new(|| {
        // Set `is_test(true)` to ensure log messages are captured by Cargo's test harness
        // and `try_init().ok()` to prevent panicking if already initialized (e.g., by another test crate).
        env_logger::builder().is_test(true).try_init().ok();
    });

    // This `init` test function will be run by Cargo first within this module.
    // It forces the initialization of the logger.
    #[test]
    fn init_logger() {
        Lazy::force(&INIT_LOGGER_UTILS);
    }

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
        let (format, path, language, force_language) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Protobuf);
        assert_eq!(path, PathBuf::from("."));
        assert_eq!(language, SupportedLanguage::Default);
        assert!(!force_language);
    }

    /// Test `parse_request_parameters` with JSON format.
    #[test]
    fn test_parse_request_parameters_json_format() {
        let request = create_mock_request(Some("format=json"), &[]);
        let (format, path, language, force_language) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Json);
        assert_eq!(path, PathBuf::from("."));
        assert_eq!(language, SupportedLanguage::Default);
        assert!(!force_language);
    }

    /// Test `parse_request_parameters` with Protobuf format.
    #[test]
    fn test_parse_request_parameters_protobuf_format() {
        let request = create_mock_request(Some("format=protobuf"), &[]);
        let (format, path, language, force_language) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Protobuf);
        assert_eq!(path, PathBuf::from("."));
        assert_eq!(language, SupportedLanguage::Default);
        assert!(!force_language);
    }

    /// Test `parse_request_parameters` with a custom output path.
    #[test]
    fn test_parse_request_parameters_output_path() {
        let request = create_mock_request(Some("output_path=./my_output"), &[]);
        let (format, path, language, force_language) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Protobuf); // Default format
        assert_eq!(path, PathBuf::from("./my_output"));
        assert_eq!(language, SupportedLanguage::Default);
        assert!(!force_language);
    }

    /// Test `parse_request_parameters` with both format and output path.
    #[test]
    fn test_parse_request_parameters_all_options() {
        let request = create_mock_request(Some("format=json,output_path=/tmp/out"), &[]);
        let (format, path, language, force_language) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Json);
        assert_eq!(path, PathBuf::from("/tmp/out"));
        assert_eq!(language, SupportedLanguage::Default);
        assert!(!force_language);
    }

    /// Test `parse_request_parameters` with unrecognized parameters.
    #[test]
    fn test_parse_request_parameters_unrecognized() {
        // Unrecognized parameters should be ignored, and defaults should apply
        let request = create_mock_request(Some("unknown=value,format=json"), &[]);
        let (format, path, language, force_language) = parse_request_parameters(&request);
        assert_eq!(format, DesiredOutputFormat::Json);
        assert_eq!(path, PathBuf::from("."));
        assert_eq!(language, SupportedLanguage::Default);
        assert!(!force_language);
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
            &SupportedLanguage::Default,
            &DesiredOutputFormat::Json,
            &ProstFieldKind::String,
        );
        match output {
            DataType::Json(value) => {
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
            &SupportedLanguage::Default,
            &DesiredOutputFormat::Protobuf,
            &ProstFieldKind::Int32,
        );
        match output {
            DataType::Protobuf(value) => {
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
            &SupportedLanguage::Default,
            &DesiredOutputFormat::Json,
            &ProstFieldKind::String,
        );
        match output {
            DataType::Json(value) => {
                assert!(value.is_array());
                let arr = value.as_array().unwrap();
                // assert!(!arr.is_empty());
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
            &SupportedLanguage::Default,
            &DesiredOutputFormat::Protobuf,
            &ProstFieldKind::String,
        );
        match output {
            DataType::Protobuf(value) => {
                assert!(value.as_list().is_some());
                let list = value.as_list().unwrap();
                // assert!(!list.is_empty());
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
