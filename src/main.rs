use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::{FirstName, LastName};
use protobuf::descriptor::{DescriptorProto, FieldDescriptorProto, FileDescriptorProto};
use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse};
// use protobuf::reflect::FileDescriptor;
use protobuf::{Message, MessageDyn, ext};
use std::any::Any;
use std::io::{BufReader, Read, Write};

mod generated_files_mod;
use generated_files_mod::fake_field_generated::FakeDataFieldOption;

mod fake_data;

// mod gen_fake;
// use gen::fake_field::gen_fake::field;

const CODE_GENERATOR_FEATURE_PROTO3_OPTIONAL: u64 = 1;
const EXTENSION_NAME: &str = "gen_fake.field";
const FIELD_EXTENSION_NUMBER: u32 = 1491;

fn generate_fake_data(message: &DescriptorProto) -> String {
    let mut fake_data = String::new();
    // fake_data.push_str(&format!("Name: {:?}     ", message.name()));
    // fake_data.push_str(&format!("Options: {:?}     ", message.options));
    // fake_data.push_str(&format!("String: {:?}     ", message.to_string()));

    for field in &message.field {
        if let Some(options) = field.options.as_ref() {
            // In protobuf extensions, we need to access them through the extension methods
            // The exact method name depends on how the extension is defined in the generated code

            // Try to get the extension from the options
            // This accesses the extension using the generated code
            // fake_data.push_str(&format!("Options: {:?}                   ", options));
            // fake_data.push_str(&format!(
            //     "\nOptions for field {}: {:?}                ",
            //     field.name.as_ref().map_or("<unnamed>", |s| s),
            //     options
            // ));

            // options.special_fields().unknown_fields().find_field(1491);
            // fake_data.push_str(&format!(
            //     "\nSpecial fields: {:?}",
            //     options
            //         .special_fields()
            //         .unknown_fields()
            //         .write_to_bytes()
            //         .iter()
            //         .collect::<Vec<_>>()
            // ));
            // .join(", ");

            // for (i, opt) in options.special_fields().unknown_fields().iter().enumerate() {
            //     fake_data.push_str(&format!("\n  MyOption {}: {:?} and {:?}", i, opt.0, opt.1));
            // }

            for (opt) in options.special_fields().unknown_fields().iter() {
                if opt.0 == FIELD_EXTENSION_NUMBER {
                    fake_data.push_str(&format!("\n  Found extension {:?} : {:?}", opt.0, opt.1));

                    match &opt.1 {
                        protobuf::UnknownValueRef::LengthDelimited(bytes) => {
                            // Skip field encoding bytes to get the actual value
                            if bytes.len() > 2 {
                                let value_bytes = &bytes[2..];
                                let data_type = String::from_utf8_lossy(value_bytes).to_string();
                                fake_data.push_str(&format!(
                                    " - data_type: {} ({:?})",
                                    data_type,
                                    data_type.type_id()
                                ));
                                fake_data.push_str(&format!(" - raw bytes: {:?}", value_bytes));
                            }
                        }
                        _ => {
                            fake_data.push_str(" - Not in expected format");
                        }
                    }
                }
            }

            //     // Print all name parts to see the full path
            //     for name_part in &opt.name {
            //         if let Some(name) = &name_part.name_part {
            //             fake_data.push_str(&format!("{} / ", name));
            //         } else {
            //             fake_data.push_str("<unnamed> / ");
            //         }
            //     }

            //     // Print option values
            //     if let Some(value) = &opt.string_value {
            //         fake_data.push_str(&format!(" = \"{}\"", String::from_utf8_lossy(value)));
            //     } else if opt.positive_int_value != Some(0) {
            //         fake_data.push_str(&format!(" = {:?}", opt.positive_int_value));
            //     }
            // }

            // let has_extension = options.uninterpreted_option.iter().any(|option| {
            //     fake_data.push_str(&format!("\nChecking option: {:?}     ", option));
            //     option.name.iter().any(|name_part| {
            //         name_part
            //             .name_part
            //             .as_ref()
            //             .map_or(false, |name| name.starts_with(EXTENSION_NAME))
            //     })
            // });

            // if has_extension {
            //     let field_name = field.name.as_ref().unwrap();
            //     fake_data.push_str(&format!(
            //         "\nFound field with extension: {:?}     ",
            //         field_name
            //     ));

            //     for opt in &options.uninterpreted_option {
            //         // Check if the option name matches the extension name
            //         let extension_matches = opt.name.iter().any(|name_part| {
            //             name_part
            //                 .name_part
            //                 .as_ref()
            //                 .map_or(false, |name| name == EXTENSION_NAME)
            //         });
            //         if extension_matches {
            //             // Extract the value from the option
            //             if let Some(value) = &opt.string_value {
            //                 let data_type = String::from_utf8_lossy(value);
            //                 fake_data.push_str(&format!("(data_type: {})     ", data_type));
            //             }
            //         }
            //     }
            // }
        }
    }

    // fake_data.push_str(&format!(
    //     "Field Details: {:?}     ",
    //     message
    //         .field
    //         .iter()
    //         .map(|fld| {
    //             format!(
    //                 "{:?}: {:?} - {:?} - {:?}",
    //                 fld.name.as_ref().unwrap(),
    //                 fld.options.as_ref().map_or("None".to_string(), |opt| {
    //                     let options_str: Vec<String> = opt
    //                         .uninterpreted_option
    //                         .iter()
    //                         .map(|option| format!("{:?}", option))
    //                         .collect();
    //                     if options_str.is_empty() {
    //                         "Empty".to_string()
    //                     } else {
    //                         options_str.join(", ")
    //                     }
    //                 }),
    //                 fld.label.unwrap(),
    //                 fld.type_(),
    //             )
    //         })
    //         .collect::<Vec<_>>()
    // ));
    // fake_data.push_str(&format!("Nested Types: {:?}     ", message.nested_type));
    // fake_data.push_str(&format!("Enum Types: {:?}     ", message.enum_type));
    // fake_data.push_str(&format!("Fields: {:?}     ", message.field));
    // fake_data.push_str("                                  ");
    // fake_data.push_str(Faker.fake::<String>().as_str());
    // fake_data.push_str(&format!("Fake: {:?}   ", Name().fake::<String>().as_str()));

    // for field in &message.field {
    //     match field.r#type {
    //         1 => fake_data.push_str(&format!("{}: {}\n", field.name, Word().fake::<String>())),
    //         2 => fake_data.push_str(&format!("{}: {}\n", field.name, Word().fake::<String>())),
    //         3 => fake_data.push_str(&format!("{}: {}\n", field.name, Word().fake::<String>())),
    //         _ => fake_data.push_str(&format!("{}: <unsupported type>\n", field.name)),
    //     }
    // }
    fake_data
}

fn iter_proto(protos: Vec<&FileDescriptorProto>) -> Vec<String> {
    // let mut messages = Vec::new();

    let messages: Vec<String> = protos
        .iter()
        .flat_map(|proto| {
            proto
                .message_type
                .iter()
                .map(|message| generate_fake_data(message))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    messages

    // let mut response = CodeGeneratorResponse::new();
    // response.set_supported_features(CODE_GENERATOR_FEATURE_PROTO3_OPTIONAL);

    // let mut file = protobuf::plugin::code_generator_response::File::new();
    // file.set_name("fake_file.sql".to_string());
    // file.set_content("CREATE TABLE fake_table (id INT);".to_string());
    // response.file.push(file);

    // response
}

fn main() {
    env_logger::init();
    log::error!("Starting the protobuf code generator...");
    let mut request = CodeGeneratorRequest::new();
    request
        .merge_from_bytes(
            BufReader::new(std::io::stdin())
                .bytes()
                .filter_map(Result::ok)
                .collect::<Vec<u8>>()
                .as_slice(),
        )
        .expect("Failed to parse proto file.");
    let result = iter_proto(request.proto_file.iter().collect());

    std::io::stdout()
        .write_all(&result.join("\n").as_bytes())
        .unwrap();
}

//     request
//         .merge_from_bytes(
//             BufReader::new(std::io::stdin())
//                 .bytes()
//                 .filter_map(Result::ok)
//                 .collect::<Vec<u8>>()
//                 .as_slice(),
//         )
//         .unwrap();

//     let should_remove_enum_field_prefix = should_remove_enum_field_prefix(&request.parameter);

//     let enums = iter_proto(request, &should_remove_enum_field_prefix);

//     let mut response = CodeGeneratorResponse::new();

//     response.set_supported_features(CODE_GENERATOR_FEATURE_PROTO3_OPTIONAL);
//     for (file_name, mapping) in enums {
//         let mut file = protobuf::plugin::code_generator_response::File::new();
//         log::info!("File name: {}", file_name);
//         log::info!("Mapping: {}", mapping);
//         file.set_name(format!("{}.sql", file_name));
//         file.set_content(mapping);
//         response.file.push(file);
//     }

//     let output = response.write_to_bytes().unwrap();
//     std::io::stdout().write_all(&output).unwrap();
// }
