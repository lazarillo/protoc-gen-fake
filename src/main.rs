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
use std::io::{self, Read, Write};

use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use prost_types::{FieldOptions, FileDescriptorProto, UninterpretedOption};

#[allow(clippy::derive_partial_eq_without_eq)]
pub mod gen_fake {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/prost_generated/gen_fake.rs"
    ));
}

use gen_fake::FakeDataFieldOption;

fn main() -> io::Result<()> {
    env_logger::init(); // RUST_LOG=debug or RUST_LOG=trace to see output

    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    let request = CodeGeneratorRequest::decode(&*buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let files_to_generate: HashSet<&String> = request.file_to_generate.iter().collect();

    let mut response = CodeGeneratorResponse::default();

    for file_descriptor in request.proto_file.into_iter() {
        if let Some(file_name) = &file_descriptor.name {
            if files_to_generate.contains(file_name) {
                log::info!("Processing primary file: {}", file_name);

                for message_type in &file_descriptor.message_type {
                    log::info!(
                        "  Message: {}",
                        message_type.name.as_deref().unwrap_or("[Unnamed Message]")
                    );

                    for field in &message_type.field {
                        log::info!(
                            "    Field: {}",
                            field.name.as_deref().unwrap_or("[Unnamed Field]")
                        );

                        if let Some(options) = &field.options {
                            log::trace!(
                                "    FieldOptions for '{}': {:?}",
                                field.name.as_deref().unwrap_or(""),
                                options
                            );

                            let mut found_custom_option = false;
                            for uninterpreted_option in &options.uninterpreted_option {
                                log::trace!(
                                    "      UninterpretedOption found: {:?}",
                                    uninterpreted_option
                                );

                                if uninterpreted_option.name.len() == 2
                                    && uninterpreted_option.name[0].name_part == "gen_fake"
                                    && uninterpreted_option.name[1].name_part == "field"
                                {
                                    log::debug!(
                                        "      Identified potential custom option 'gen_fake.field'!"
                                    );

                                    // Let's try decoding from `string_value` first as `Vec<u8>`.
                                    if let Some(value_bytes_vec) =
                                        &uninterpreted_option.string_value
                                    {
                                        log::trace!(
                                            "        string_value present, length: {}",
                                            value_bytes_vec.len()
                                        );
                                        log::trace!(
                                            "        string_value (hex): {:x?}",
                                            value_bytes_vec
                                        );

                                        match FakeDataFieldOption::decode(value_bytes_vec.as_ref())
                                        {
                                            Ok(decoded_option) => {
                                                log::info!(
                                                    "      SUCCESS: Manually decoded custom FakeDataFieldOption for field '{}': data_type = {}",
                                                    field.name.as_deref().unwrap_or(""),
                                                    decoded_option.data_type
                                                );
                                                found_custom_option = true;
                                                break;
                                            }
                                            Err(e) => {
                                                log::error!(
                                                    "      ERROR: Decoding FakeDataFieldOption from string_value failed for field '{}': {}",
                                                    field.name.as_deref().unwrap_or(""),
                                                    e
                                                );
                                            }
                                        }
                                    } else {
                                        log::warn!(
                                            "      Custom option 'gen_fake.field' found by name, but string_value (serialized message) is MISSING. Option: {:?}",
                                            uninterpreted_option
                                        );
                                    }
                                }
                            }

                            if !found_custom_option {
                                log::debug!(
                                    "      No matching FakeDataFieldOption found or successfully decoded in uninterpreted_option for field '{}'",
                                    field.name.as_deref().unwrap_or("")
                                );
                            }
                        }
                    }
                }

                response
                    .file
                    .push(prost_types::compiler::code_generator_response::File {
                        name: Some(format!("{}.txt", file_name)),
                        content: Some(format!("// Generated content for {}", file_name)),
                        ..Default::default()
                    });
            } else {
                log::debug!("Skipping dependency/non-primary file: {}", file_name);
            }
        }
    }

    let mut output_buffer = Vec::new();
    response.encode(&mut output_buffer)?;
    io::stdout().write_all(&output_buffer)?;

    Ok(())
}
