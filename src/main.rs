// use fake::Fake;
// use fake::faker::internet::en::SafeEmail;
// use fake::faker::name::en::{FirstName, LastName};
// use fake::locales::{EN, FR_FR, PT_BR};
// use protobuf::descriptor::{DescriptorProto, FieldDescriptorProto, FileDescriptorProto};
// use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse};
// // use protobuf::reflect::FileDescriptor;
// use protobuf::{Message, MessageDyn, ext};
// use std::any::Any;
// use std::collections::HashSet;
// use std::io::{BufReader, Read, Write};

// mod generated_files_mod;
// use generated_files_mod::fake_field_generated::FakeDataFieldOption;

// use crate::fake_data::get_fake_data;

// mod fake_data;

// // mod gen_fake;
// // use gen::fake_field::gen_fake::field;

// const CODE_GENERATOR_FEATURE_PROTO3_OPTIONAL: u64 = 1;
// const EXTENSION_NAME: &str = "gen_fake.field";
// const FIELD_EXTENSION_NUMBER: u32 = 1491;

// fn generate_fake_data(message: &DescriptorProto) -> String {
//     let mut fake_data = String::new();
//     // fake_data.push_str(&format!("Name: {:?}     ", message.name()));
//     // fake_data.push_str(&format!("Options: {:?}     ", message.options));
//     // fake_data.push_str(&format!("String: {:?}     ", message.to_string()));

//     for field in &message.field {
//         if let Some(options) = field.options.as_ref() {
//             // In protobuf extensions, we need to access them through the extension methods
//             // The exact method name depends on how the extension is defined in the generated code

//             // Try to get the extension from the options
//             // This accesses the extension using the generated code
//             // fake_data.push_str(&format!("Options: {:?}                   ", options));
//             // fake_data.push_str(&format!(
//             //     "\nOptions for field {}: {:?}                ",
//             //     field.name.as_ref().map_or("<unnamed>", |s| s),
//             //     options
//             // ));

//             // options.special_fields().unknown_fields().find_field(1491);
//             // fake_data.push_str(&format!(
//             //     "\nSpecial fields: {:?}",
//             //     options
//             //         .special_fields()
//             //         .unknown_fields()
//             //         .write_to_bytes()
//             //         .iter()
//             //         .collect::<Vec<_>>()
//             // ));
//             // .join(", ");

//             // for (i, opt) in options.special_fields().unknown_fields().iter().enumerate() {
//             //     fake_data.push_str(&format!("\n  MyOption {}: {:?} and {:?}", i, opt.0, opt.1));
//             // }

//             for (opt) in options.special_fields().unknown_fields().iter() {
//                 if opt.0 == FIELD_EXTENSION_NUMBER {
//                     fake_data.push_str(&format!("\n  Found extension {:?} : {:?}", opt.0, opt.1));

//                     match &opt.1 {
//                         protobuf::UnknownValueRef::LengthDelimited(bytes) => {
//                             // Skip field encoding bytes to get the actual value
//                             if bytes.len() > 2 {
//                                 let value_bytes = &bytes[2..];
//                                 let data_type = String::from_utf8_lossy(value_bytes).to_string();
//                                 fake_data.push_str(&format!(
//                                     " - data_type: {} ({:?})",
//                                     data_type,
//                                     data_type.type_id()
//                                 ));
//                                 fake_data.push_str(&format!(" - raw bytes: {:?}", value_bytes));
//                             }
//                         }
//                         _ => {
//                             fake_data.push_str(" - Not in expected format");
//                         }
//                     }
//                 }
//             }

//             //     // Print all name parts to see the full path
//             //     for name_part in &opt.name {
//             //         if let Some(name) = &name_part.name_part {
//             //             fake_data.push_str(&format!("{} / ", name));
//             //         } else {
//             //             fake_data.push_str("<unnamed> / ");
//             //         }
//             //     }

//             //     // Print option values
//             //     if let Some(value) = &opt.string_value {
//             //         fake_data.push_str(&format!(" = \"{}\"", String::from_utf8_lossy(value)));
//             //     } else if opt.positive_int_value != Some(0) {
//             //         fake_data.push_str(&format!(" = {:?}", opt.positive_int_value));
//             //     }
//             // }

//             // let has_extension = options.uninterpreted_option.iter().any(|option| {
//             //     fake_data.push_str(&format!("\nChecking option: {:?}     ", option));
//             //     option.name.iter().any(|name_part| {
//             //         name_part
//             //             .name_part
//             //             .as_ref()
//             //             .map_or(false, |name| name.starts_with(EXTENSION_NAME))
//             //     })
//             // });

//             // if has_extension {
//             //     let field_name = field.name.as_ref().unwrap();
//             //     fake_data.push_str(&format!(
//             //         "\nFound field with extension: {:?}     ",
//             //         field_name
//             //     ));

//             //     for opt in &options.uninterpreted_option {
//             //         // Check if the option name matches the extension name
//             //         let extension_matches = opt.name.iter().any(|name_part| {
//             //             name_part
//             //                 .name_part
//             //                 .as_ref()
//             //                 .map_or(false, |name| name == EXTENSION_NAME)
//             //         });
//             //         if extension_matches {
//             //             // Extract the value from the option
//             //             if let Some(value) = &opt.string_value {
//             //                 let data_type = String::from_utf8_lossy(value);
//             //                 fake_data.push_str(&format!("(data_type: {})     ", data_type));
//             //             }
//             //         }
//             //     }
//             // }
//         }
//     }

//     // fake_data.push_str(&format!(
//     //     "Field Details: {:?}     ",
//     //     message
//     //         .field
//     //         .iter()
//     //         .map(|fld| {
//     //             format!(
//     //                 "{:?}: {:?} - {:?} - {:?}",
//     //                 fld.name.as_ref().unwrap(),
//     //                 fld.options.as_ref().map_or("None".to_string(), |opt| {
//     //                     let options_str: Vec<String> = opt
//     //                         .uninterpreted_option
//     //                         .iter()
//     //                         .map(|option| format!("{:?}", option))
//     //                         .collect();
//     //                     if options_str.is_empty() {
//     //                         "Empty".to_string()
//     //                     } else {
//     //                         options_str.join(", ")
//     //                     }
//     //                 }),
//     //                 fld.label.unwrap(),
//     //                 fld.type_(),
//     //             )
//     //         })
//     //         .collect::<Vec<_>>()
//     // ));
//     // fake_data.push_str(&format!("Nested Types: {:?}     ", message.nested_type));
//     // fake_data.push_str(&format!("Enum Types: {:?}     ", message.enum_type));
//     // fake_data.push_str(&format!("Fields: {:?}     ", message.field));
//     // fake_data.push_str("                                  ");
//     // fake_data.push_str(Faker.fake::<String>().as_str());
//     // fake_data.push_str(&format!("Fake: {:?}   ", Name().fake::<String>().as_str()));

//     // for field in &message.field {
//     //     match field.r#type {
//     //         1 => fake_data.push_str(&format!("{}: {}\n", field.name, Word().fake::<String>())),
//     //         2 => fake_data.push_str(&format!("{}: {}\n", field.name, Word().fake::<String>())),
//     //         3 => fake_data.push_str(&format!("{}: {}\n", field.name, Word().fake::<String>())),
//     //         _ => fake_data.push_str(&format!("{}: <unsupported type>\n", field.name)),
//     //     }
//     // }
//     fake_data
// }

// // pub fn generate_py_init_files(
// //     file_descriptor: &FileDescriptorProto,
// //     opts: &Option<py_package::PyPackageOptions>,
// // ) -> impl Iterator<Item = File> {
// //     // Creates an iterator with 0 or 1 items based on whether `opts` is `Some` or `None`.
// //     opts
// //         // Using as_ref to convert the Option into an Option<&PyPackageOptions>.
// //         // This allows us to avoid moving the value out of the Option.
// //         .as_ref()
// //         .into_iter()
// //         .filter(|opt| opt.enable)
// //         // Using flat map to iterate over the option and generate an iterator of `File` objects.
// //         .flat_map(|opt| create_init_files(opt, file_descriptor))

// // }

// //     response.set_supported_features(CODE_GENERATOR_RESPONSE_FEATURE_PROTO3_OPTIONAL);
// //     let opts: Vec<(&FileDescriptorProto, Option<py_package::PyPackageOptions>)> = request
// //         .proto_file
// //         .iter()
// //         .map(|file| {
// //             let opts = py_package::exts::py_package_opts.get(&file.options);
// //             if let Some(opt) = &opts {
// //                 log::error!("Found py_package options in file: {}", file.name());
// //                 log::error!("Options: {:?}", opt);
// //             };
// //             (file, opts)
// //         })
// //         .collect();

// fn iter_proto(protos: Vec<&FileDescriptorProto>) -> Vec<String> {
//     // let mut messages = Vec::new();

//     let messages: Vec<String> = protos
//         .iter()
//         .flat_map(|proto| {
//             proto
//                 .message_type
//                 .iter()
//                 .map(|message| generate_fake_data(message))
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();
//     messages

//     // let mut response = CodeGeneratorResponse::new();
//     // response.set_supported_features(CODE_GENERATOR_FEATURE_PROTO3_OPTIONAL);

//     // let mut file = protobuf::plugin::code_generator_response::File::new();
//     // file.set_name("fake_file.sql".to_string());
//     // file.set_content("CREATE TABLE fake_table (id INT);".to_string());
//     // response.file.push(file);

//     // response
// }

// // fn main() {
// //     // Read message from stdin
// //     let mut reader = BufReader::new(io::stdin());
// //     let mut incoming_request = Vec::new();
// //     reader.read_to_end(&mut incoming_request).unwrap();

// //     // Parse as a request
// //     let req = CodeGeneratorRequest::parse_from_bytes(&incoming_request).unwrap();

// //     // Generate the content for each output file
// //     let mut response = CodeGeneratorResponse::new();
// //     for proto_file in req.proto_file.iter() {
// //         let mut output = String::new();
// //         output.push_str(&format!("// from file: {:?}\n", &proto_file.name));
// //         output.push_str(&format!("// package: {:?}\n", &proto_file.package));
// //         for message in proto_file.message_type.iter() {
// //             output.push_str(&format!("\nmessage: {:?}\n", &message.name));
// //             for field in message.field.iter() {
// //                 output.push_str(&format!(
// //                     "- {:?} {:?} {:?}\n",
// //                     field.type_,
// //                     field.type_name,
// //                     field.name(),
// //                 ));
// //             }
// //         }

// //         // Add it to the response
// //         let mut output_file = code_generator_response::File::new();
// //         output_file.content = Some(output);
// //         output_file.name = Some(format!("{:?}/out.txt", &proto_file.name.as_ref().unwrap()));
// //         response.file.push(output_file);
// //     }

// //     // Serialize the response to binary message and return it
// //     let out_bytes: Vec<u8> = response.write_to_bytes().unwrap();
// //     io::stdout().write_all(&out_bytes).unwrap();
// // }

// fn main() {
//     env_logger::init();
//     log::error!("Starting the protobuf code generator...\n");
//     // let email_fr = get_fake_data("SafeEmail", FR_FR);
//     // log::error!("Fake French safe email: {}", email_fr);
//     // let name_en = get_fake_data("FirstName", EN);
//     // log::error!("Fake English first name: {}", name_en);
//     // let name_pt = get_fake_data("LastName", PT_BR);
//     // log::error!("Fake Portuguese last name: {}", name_pt);
//     let mut request = CodeGeneratorRequest::new();
//     request
//         .merge_from_bytes(
//             BufReader::new(std::io::stdin())
//                 .bytes()
//                 .filter_map(Result::ok)
//                 .collect::<Vec<u8>>()
//                 .as_slice(),
//         )
//         .expect("Failed to parse proto file.");
//     let files_of_interest: HashSet<&String> = request.file_to_generate.iter().collect();
//     let mut result = CodeGeneratorResponse::new();
//     for proto_file in request.proto_file.iter() {
//         if let Some(file_name) = &proto_file.name {
//             log::error!("Processing proto file: {}", file_name);
//             log::error!(
//                 "    (From package: {}",
//                 &proto_file.package.as_deref().unwrap_or_default()
//             );
//             log::error!("    with options: {:?}", &proto_file.options);
//             if files_of_interest.contains(file_name) {
//                 log::error!("{} is a key file, looking at messages within...", file_name);
//                 for message in proto_file.message_type.iter() {
//                     log::error!(
//                         "Processing message: {:?}",
//                         &message.name.as_deref().unwrap_or("<unnamed>")
//                     );
//                     for field in message.field.iter() {
//                         log::error!(
//                             "Field: {:?} (type: {:?}, type_name: {:?},\noptions: {:?},\nspecial_fields: {:?})\n",
//                             &field.name,
//                             &field.type_,
//                             &field.type_name,
//                             &field.options,
//                             &field.special_fields
//                         );
//                         for option in field.options.iter() {
//                             log::error!(
//                                 "Within the option: special fields {:?}, special fields {:?}, uninterpreted option {:?}, **unknown fields** {:?}, <<<specific field>>>: {:?}\n",
//                                 option.special_fields,
//                                 option.uninterpreted_option,
//                                 option.unknown_fields(),
//                                 option.special_fields(),
//                                 option.special_fields().unknown_fields()
//                             );
//                         }
//                         for unknown in field.options.unknown_fields().iter() {
//                             log::error!(
//                                 "Unknown field: {:?} with key {:?} and value: {:?}\n",
//                                 unknown,
//                                 unknown.0,
//                                 unknown.1
//                             );
//                         }
//                         log::error!(
//                             "With the special fields for field {}: {:?}\n",
//                             field.name.as_ref().unwrap_or(&"<unnamed>".to_string()),
//                             field.special_fields
//                         );
//                         log::error!("\n");
//                     }
//                 }
//             }
//             log::error!("Done.\n");
//         }
//     }

//     //     let fake_data = iter_proto(vec![proto_file]);
//     //     log::error!("Generated fake data: {:?}", fake_data);
//     //     for data in fake_data {
//     //         result.file.push({
//     //             let mut file = protobuf::plugin::code_generator_response::File::new();
//     //             file.set_name(format!("{}.sql", proto_file.name));
//     //             file.set_content(data);
//     //             file
//     //         });
//     //     }
//     // }

//     // let result = iter_proto(request.proto_file.iter().collect());

//     // std::io::stdout()
//     //     .write_all(&result.join("\n").as_bytes())
//     //     .unwrap();
// }

// //     request
// //         .merge_from_bytes(
// //             BufReader::new(std::io::stdin())
// //                 .bytes()
// //                 .filter_map(Result::ok)
// //                 .collect::<Vec<u8>>()
// //                 .as_slice(),
// //         )
// //         .unwrap();

// //     let should_remove_enum_field_prefix = should_remove_enum_field_prefix(&request.parameter);

// //     let enums = iter_proto(request, &should_remove_enum_field_prefix);

// //     let mut response = CodeGeneratorResponse::new();

// //     response.set_supported_features(CODE_GENERATOR_FEATURE_PROTO3_OPTIONAL);
// //     for (file_name, mapping) in enums {
// //         let mut file = protobuf::plugin::code_generator_response::File::new();
// //         log::error!("File name: {}", file_name);
// //         log::error!("Mapping: {}", mapping);
// //         file.set_name(format!("{}.sql", file_name));
// //         file.set_content(mapping);
// //         response.file.push(file);
// //     }

// //     let output = response.write_to_bytes().unwrap();
// //     std::io::stdout().write_all(&output).unwrap();
// // }

use prost::Message;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};

// IMPORT FROM `prost_types` for Google's well-known Protobuf types.
// This crate provides pre-generated Rust types for descriptor.proto and plugin.proto,
// removing the need to compile them directly in your build.rs.
use prost_types::{
    FieldOptions,        // Options for a field, which you are extending
    FileDescriptorProto, // Describes a .proto file
    compiler::{
        CodeGeneratorRequest, // The main input message from protoc
        CodeGeneratorResponse,
    }, // The main output message to protoc
    field_options::CType, // An enum related to FieldOptions, re-exported if needed
};

// Include the prost-generated code for your custom options.
// This path corresponds to the `out_dir` and package structure defined in your build.rs.
// The `gen_fake` module comes from `package gen_fake;` in your .proto file,
// and `options` is a sub-module likely created by prost for extensions.
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod gen_fake {
    pub mod options {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/prost_generated/gen_fake.rs"
        ));
    }
}

// Import your specific custom option message and its extension accessor function.
use gen_fake::options::{FakeDataFieldOption, field_exts};
use gen_fake::

use gen_fake::exts; // Import the extension accessor function

fn main() -> io::Result<()> {
    // Initialize logging for better debugging output.
    // Ensure you have `log` and `env_logger` in your Cargo.toml dependencies.
    env_logger::init();

    // Read the CodeGeneratorRequest binary data from stdin.
    // This is how `protoc` communicates the .proto file information to your plugin.
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    // Decode the binary buffer into a strongly-typed CodeGeneratorRequest Rust struct.
    // `prost::Message::decode` handles the deserialization.
    let request = CodeGeneratorRequest::decode(&*buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Create a HashSet for efficient lookup of the names of files that need to be generated.
    // `request.file_to_generate` contains the explicit file paths passed to `protoc`.
    let files_to_generate: HashSet<&String> = request.file_to_generate.iter().collect();

    // Initialize an empty CodeGeneratorResponse, which your plugin will populate
    // with generated files and return to `protoc`.
    let mut response = CodeGeneratorResponse::default();

    // Iterate through all FileDescriptorProto objects provided by `protoc`.
    // `request.proto_file` contains descriptors for all input files, including dependencies.
    for file_descriptor in request.proto_file.into_iter() {
        // Check if the file has a name (it should).
        if let Some(file_name) = &file_descriptor.name {
            // Only process files that were explicitly requested for code generation.
            if files_to_generate.contains(file_name) {
                log::info!("Processing primary file: {}", file_name);

                // Iterate through each message type defined in the current .proto file.
                for message_type in &file_descriptor.message_type {
                    log::info!(
                        "  Message: {}",
                        message_type.name.as_deref().unwrap_or("[Unnamed Message]")
                    );

                    // Iterate through each field within the current message type.
                    for field in &message_type.field {
                        log::info!(
                            "    Field: {}",
                            field.name.as_deref().unwrap_or("[Unnamed Field]")
                        );

                        // Check if the field has any options associated with it.
                        if let Some(options) = &field.options {
                            // Attempt to get your custom extension.
                            // `options.get_extension(field_exts::field)` uses the generated
                            // accessor function (`field_exts::field`) to retrieve an
                            // `Option<&FakeDataFieldOption>`.
                            if let Some(fake_data_option) = options.get_extension(field_exts::field)
                            {
                                log::info!(
                                    "      Found custom FakeDataFieldOption on field '{}': data_type = {}",
                                    field.name.as_deref().unwrap_or(""),
                                    fake_data_option.data_type // Access the `data_type` field of your custom option
                                );
                            } else {
                                log::debug!(
                                    "      No FakeDataFieldOption found on field '{}'",
                                    field.name.as_deref().unwrap_or("")
                                );
                            }

                            // This loop checks for any truly "unknown" fields.
                            // If your custom option is correctly defined and accessed via `get_extension`,
                            // it should NOT appear in this `unknown_fields` iterator.
                            for unknown in options.unknown_fields().iter() {
                                log::warn!(
                                    "      Unexpected Unknown field (key: {}): wire_type={:?}, bytes={:?}",
                                    unknown.0,                 // The field tag number
                                    unknown.1.wire_type(), // The wire type (e.g., LengthDelimited)
                                    unknown.1.encode_to_vec() // The raw bytes of the unknown field's value
                                );
                            }
                        }
                    }
                }

                // Add a dummy generated file to the response for demonstration.
                // In a real plugin, you would generate actual code based on the .proto content.
                response
                    .file
                    .push(prost_types::code_generator_response::File {
                        name: Some(format!("{}.txt", file_name)), // Name of the generated file
                        content: Some(format!("// Generated content for {}", file_name)), // Content of the generated file
                        ..Default::default() // Use default for other fields (like insertion_point)
                    });
            } else {
                log::debug!("Skipping dependency/non-primary file: {}", file_name);
            }
        }
    }

    // Encode the CodeGeneratorResponse back into binary format and write it to stdout.
    // This is how your plugin sends its output back to `protoc`.
    let mut output_buffer = Vec::new();
    response.encode(&mut output_buffer)?;
    io::stdout().write_all(&output_buffer)?;

    Ok(())
}
