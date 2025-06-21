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

use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use prost_types::{FieldOptions, FileDescriptorProto, UninterpretedOption};

// Import ExtensionDescriptor in addition to others
use once_cell::sync::Lazy;
use prost_reflect::{
    DescriptorPool, ExtensionDescriptor, FieldDescriptor, FileDescriptor, MessageDescriptor, Value,
};
use std::borrow::Cow; // Import Cow to handle the return type of get_extension

// Include the prost-generated code for your custom message.
pub mod generated_files_mod;
use generated_files_mod::FakeDataFieldOption;

// Initialize a static DescriptorPool from the binary descriptor set generated by build.rs.
static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    // This `env!` macro reads the environment variable set by `build.rs`.
    // It's evaluated at compile time, ensuring the path to the embedded bytes is correct.
    let descriptor_set_bytes = include_bytes!(env!("DESCRIPTOR_SET_BIN_PATH")); // This macro embeds the file's bytes directly into the binary.
    // HIGHLIGHT END.
    DescriptorPool::decode(descriptor_set_bytes.as_ref()).expect("Failed to decode descriptor set")
});

fn iterate_over_request(request: &CodeGeneratorRequest, key_files: &HashSet<&String>) {
    for file in request.proto_file.iter() {
        if let Some(filename) = file.name.as_ref() {
            if key_files.contains(filename) {
                log::info!("Processing primary file: {}", filename);
            }
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::init();

    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    let request = CodeGeneratorRequest::decode(&*buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let key_files: HashSet<&String> = request.file_to_generate.iter().collect();

    let mut response = CodeGeneratorResponse::default();

    // Pre-fetch the ExtensionDescriptor for our custom option, as it's needed repeatedly.
    // Assign to an owned `ExtensionDescriptor`, as `get_extension_by_name` returns `Option<ExtensionDescriptor>`.
    let fake_data_extension: ExtensionDescriptor = DESCRIPTOR_POOL
    // let fake_data_extension: ExtensionDescriptor = (&*DESCRIPTOR_POOL)
        .get_extension_by_name("gen_fake.field")
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Extension 'gen_fake.field' not found in descriptor pool. Ensure fake_field.proto is processed by build.rs correctly.")
        })?;

    for file_descriptor in request.proto_file.into_iter() {
        if let Some(file_name) = &file_descriptor.name {
            if key_files.contains(file_name) {
                log::info!("Processing primary file: {}", file_name);

                let file_descriptor_reflect =
                    DESCRIPTOR_POOL.get_file_by_name(file_name).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("File {} not found in descriptor pool", file_name),
                        )
                    })?;

                for message_type in &file_descriptor.message_type {
                    log::info!(
                        "  Message: {}",
                        message_type.name.as_deref().unwrap_or("[Unnamed Message]")
                    );

                    let message_name_str = message_type.name.as_deref().unwrap_or("");
                    let message_descriptor = file_descriptor_reflect
                        .messages()
                        .find(|msg| msg.name() == message_name_str)
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::NotFound,
                                format!(
                                    "Message {} not found in file descriptor for {}",
                                    message_name_str, file_name
                                ),
                            )
                        })?;

                    for field in &message_type.field {
                        log::info!(
                            "    Field: {}",
                            field.name.as_deref().unwrap_or("[Unnamed Field]")
                        );

                        let field_descriptor = message_descriptor
                            .get_field_by_name(field.name.as_deref().unwrap_or(""))
                            .ok_or_else(|| {
                                io::Error::new(
                                    io::ErrorKind::NotFound,
                                    format!(
                                        "Field {} not found in message {}",
                                        field.name.as_deref().unwrap_or(""),
                                        message_type.name.as_deref().unwrap_or("")
                                    ),
                                )
                            })?;

                        // CORRECTED: Bind the result of `field_descriptor.options()` to a local variable
                        // to extend its lifetime.
                        let field_options_reflect = field_descriptor.options();
                        let option_value_cow: Cow<'_, Value> =
                            field_options_reflect.get_extension(&fake_data_extension);

                        // Use `match` or `if let` on `option_value_cow.as_ref()` which yields `&Value`.
                        if let Value::Message(dynamic_option_message) = option_value_cow.as_ref() {
                            let serialized_option = dynamic_option_message.encode_to_vec();
                            match FakeDataFieldOption::decode(serialized_option.as_slice()) {
                                Ok(fake_data_option) => {
                                    log::info!(
                                        "      SUCCESS: Found custom FakeDataFieldOption on field '{}': data_type = {}",
                                        field.name.as_deref().unwrap_or(""),
                                        fake_data_option.data_type
                                    );
                                }
                                Err(e) => {
                                    log::error!(
                                        "      ERROR: Failed to decode FakeDataFieldOption from reflected data for field '{}': {}",
                                        field.name.as_deref().unwrap_or(""),
                                        e
                                    );
                                }
                            }
                        } else {
                            // This branch means the option was not set, or was set to a non-message type.
                            log::debug!(
                                "      No custom option 'gen_fake.field' found or it was not a Message type for field '{}'. Value: {:?}",
                                field.name.as_deref().unwrap_or(""),
                                option_value_cow
                            );
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
