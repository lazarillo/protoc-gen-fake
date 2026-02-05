//! # (Protoc) Gen Fake
//!
//! `protoc-gen-fake` is a custom plugin for `protoc` that uses annotation on the proto file schema
//! to generate a file with fake data well-aligned with the expected data types of the fields.

use base64::{Engine as _, engine::general_purpose};
use prost::Message;
use prost_reflect::{
    Cardinality, DescriptorPool, DynamicMessage, MessageDescriptor, ReflectMessage, Value,
};

use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use rand::Rng;
use rand::seq::IndexedRandom;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

pub mod utils;
use utils::{
    DataType, DesiredOutputFormat, OutputEncoding, SupportedLanguage, choose_language,
    get_fake_data_output_value, get_key_files, get_runtime_descriptor_pool,
    parse_request_parameters,
};

pub mod fake_data;

fn main() -> io::Result<()> {
    // Initialize logging
    env_logger::init();

    // Handle command line arguments
    for arg in std::env::args() {
        match arg.as_str() {
            "--version" | "-V" => {
                println!("protoc-gen-fake {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--help" | "-h" => {
                println!("protoc-gen-fake: Protocol Buffer Fake Data Generator");
                println!(
                    "Usage: protoc --plugin=protoc-gen-fake=<path_to_plugin> --fake_out=. --fake_opt=<options> -I <proto_path> <proto_file>"
                );
                return Ok(());
            }
            _ => {}
        }
    }

    // Read CodeGeneratorRequest from stdin
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    // Decode with prost
    let request = CodeGeneratorRequest::decode(buffer.as_slice())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Decode with DynamicMessage (for Options preservation)
    let bootstrap_bytes = include_bytes!(env!("BOOTSTRAP_DESCRIPTOR_SET_BIN_PATH"));
    let bootstrap_pool = DescriptorPool::decode(bootstrap_bytes.as_ref())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let request_desc = bootstrap_pool
        .get_message_by_name("google.protobuf.compiler.CodeGeneratorRequest")
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "CodeGeneratorRequest descriptor not found in bootstrap pool",
            )
        })?;
    let request_dyn = DynamicMessage::decode(request_desc, buffer.as_slice())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Build Options Map: FullName -> Options DynamicMessage
    let mut options_map: HashMap<String, DynamicMessage> = HashMap::new();
    build_options_map(&request_dyn, &mut options_map);

    // Parse parameters
    let (output_format, output_path, global_language, force_global_language, output_encoding) =
        parse_request_parameters(&request);

    // Get key files
    let key_files = get_key_files(&request);
    log::debug!("{} key file(s) to generate fake data over", key_files.len());

    // Build runtime descriptor pool
    let runtime_descriptor_pool = get_runtime_descriptor_pool(&request);

    // Prepare response
    let mut response = CodeGeneratorResponse::default();
    let mut rng = rand::rng(); // Use thread_rng directly if acceptable, or explicit Rng

    // Main loop
    for filename in key_files.iter() {
        if let Some(_) = request
            .proto_file
            .iter()
            .find(|f| f.name.as_ref() == Some(filename))
        {
            log::info!("Processing file: {}", filename);

            let output_file_path = Path::new(filename);
            let file_stem = output_file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let output_name = format!("{}.b64", file_stem);

            // Determine binary output path (logic preserved from original)
            let binary_name: String;
            let user_supplied_output_path_str = output_path.to_string_lossy();

            if user_supplied_output_path_str == "." {
                binary_name = Path::new(file_stem)
                    .with_extension("bin")
                    .to_string_lossy()
                    .to_string();
            } else if user_supplied_output_path_str.ends_with(".bin") {
                binary_name = user_supplied_output_path_str.to_string();
            } else {
                binary_name = Path::new(user_supplied_output_path_str.as_ref())
                    .join(file_stem)
                    .with_extension("bin")
                    .to_string_lossy()
                    .to_string();
            }

            let mut all_messages: Vec<DynamicMessage> = Vec::new();

            let runtime_file_descriptor = runtime_descriptor_pool
                .get_file_by_name(filename)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("File '{}' not found in pool", filename),
                    )
                })?;

            let mut message_processed = false;

            // Loop through messages in the file
            for message_descr in runtime_file_descriptor.messages() {
                let message_name = message_descr.name();
                // Check for (gen_fake.fake_msg).include = true
                // We need to look up the extension in the pool to check it.
                // The extension is defined in `gen_fake/fake_field.proto`, extension number 1490 for MessageOptions.

                let is_included =
                    is_message_included(&message_descr, &runtime_descriptor_pool, &options_map);

                if is_included {
                    if message_processed {
                        log::warn!(
                            "Skipping message '{}' because another message was already processed.",
                            message_name
                        );
                        continue;
                    }
                    message_processed = true;
                    log::debug!("Generating fake data for message: {}", message_name);

                    let message = generate_fake_message_field(
                        &message_descr,
                        &mut rng,
                        &output_format,
                        &global_language,
                        force_global_language,
                        &runtime_descriptor_pool,
                        &options_map,
                    )?;
                    all_messages.push(message);
                }
            }

            // Generate output file content
            let mut generated_file_content: Vec<u8> = Vec::new();
            let mut generated_file =
                prost_types::compiler::code_generator_response::File::default();
            generated_file.name = Some(output_name);

            // Encode messages
            for next_message in all_messages {
                let msg_bytes = next_message.encode_to_vec();
                generated_file_content.extend_from_slice(&msg_bytes);
            }

            // Write binary if requested
            if output_encoding == OutputEncoding::Binary || output_encoding == OutputEncoding::Both
            {
                if let Some(parent) = Path::new(&binary_name).parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&binary_name, &generated_file_content)?;
                log::info!("Wrote binary output to: {}", binary_name);
            }

            // Add to response if Base64 requested
            if output_encoding == OutputEncoding::Base64 || output_encoding == OutputEncoding::Both
            {
                let file_content_string = general_purpose::STANDARD.encode(&generated_file_content);
                generated_file.content = Some(file_content_string);
                response.file.push(generated_file);
            }
        }
    }

    // Write response to stdout
    let mut output_buffer = Vec::new();
    response
        .encode(&mut output_buffer)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    io::stdout().write_all(&output_buffer)?;

    Ok(())
}

// Helper to check for the `fake_msg` option on a message
fn is_message_included(
    message_descr: &MessageDescriptor,
    pool: &DescriptorPool,
    options_map: &HashMap<String, DynamicMessage>,
) -> bool {
    let full_name = message_descr.full_name();
    log::debug!("Checking inclusion for message: {}", full_name);

    if let Some(options_msg) = options_map.get(full_name) {
        log::debug!("Found options for message: {}", full_name);

        // fake_msg extension ID is 1490
        // We need the ExtensionDescriptor from the pool to look it up in the DynamicMessage
        if let Some(ext_file) = pool
            .get_file_by_name("gen_fake/fake_field.proto")
            .or_else(|| pool.get_file_by_name("proto/gen_fake/fake_field.proto"))
        {
            if let Some(ext_desc) = ext_file.extensions().find(|e| e.number() == 1490) {
                if options_msg.has_extension(&ext_desc) {
                    let val_cow = options_msg.get_extension(&ext_desc);
                    if let Value::Message(msg_opt) = val_cow.as_ref() {
                        // Check "include" field (field 1)
                        if let Some(include_field) =
                            msg_opt.descriptor().get_field_by_name("include")
                        {
                            if let Value::Bool(b) = msg_opt.get_field(&include_field).as_ref() {
                                return *b;
                            }
                        }
                    }
                }
            }
        }
    } else {
        log::debug!("No options found in map for message: {}", full_name);
    }
    false
}

fn generate_fake_message_field(
    message_descr: &MessageDescriptor,
    rng: &mut impl Rng,
    output_format: &DesiredOutputFormat,
    global_language: &SupportedLanguage,
    force_global_language: bool,
    runtime_descriptor_pool: &DescriptorPool,
    options_map: &HashMap<String, DynamicMessage>,
) -> io::Result<DynamicMessage> {
    let mut message = DynamicMessage::new(message_descr.clone());

    // 0. Pre-calculate active Oneof fields
    let mut active_oneof_fields = HashSet::new();
    for oneof in message_descr.oneofs() {
        // Collect all fields in this oneof
        let fields: Vec<prost_reflect::FieldDescriptor> = oneof.fields().collect();
        // Pick one randomly
        if let Some(chosen) = fields.choose(rng) {
            active_oneof_fields.insert(chosen.full_name().to_string());
        }
    }

    for field_descr in message_descr.fields() {
        // Check Oneof constraint
        if let Some(_oneof) = field_descr.containing_oneof() {
            let fname = field_descr.full_name().to_string();
            if !active_oneof_fields.contains(&fname) {
                // Skip this field as it wasn't selected for the oneof group
                continue;
            }
        } else {
            // log::debug!("Field {} is not in a oneof", field_descr.full_name());
        }

        let field_kind = field_descr.kind();
        let is_list_field = field_descr.is_list() || field_descr.is_map();
        let cardinality = field_descr.cardinality();

        let mut fake_field_value: Option<DataType> = None;

        // 1. Check for `fake_data` option (ID 1491)
        let mut fake_opt_data: Option<(String, SupportedLanguage, i32, i32)> = None;

        let field_full_name = field_descr.full_name().to_string();

        // Use options_map for lookup
        if let Some(field_options) = options_map.get(&field_full_name) {
            // ... existing lookup ...
            if let Some(ext_file) = runtime_descriptor_pool
                .get_file_by_name("gen_fake/fake_field.proto")
                .or_else(|| {
                    runtime_descriptor_pool.get_file_by_name("proto/gen_fake/fake_field.proto")
                })
            {
                if let Some(ext_desc) = ext_file.extensions().find(|e| e.number() == 1491) {
                    if field_options.has_extension(&ext_desc) {
                        let val_cow = field_options.get_extension(&ext_desc);
                        if let Value::Message(fake_data_msg) = val_cow.as_ref() {
                            let type_str = fake_data_msg
                                .descriptor()
                                .get_field_by_name("data_type")
                                .map(|f| {
                                    fake_data_msg
                                        .get_field(&f)
                                        .as_ref()
                                        .as_str()
                                        .unwrap_or("")
                                        .to_string()
                                })
                                .unwrap_or_default();

                            let lang_str = fake_data_msg
                                .descriptor()
                                .get_field_by_name("language")
                                .map(|f| {
                                    fake_data_msg
                                        .get_field(&f)
                                        .as_ref()
                                        .as_str()
                                        .unwrap_or("")
                                        .to_string()
                                })
                                .unwrap_or_default();
                            let lang = SupportedLanguage::from_str(&lang_str)
                                .unwrap_or(SupportedLanguage::Default);

                            let min_c = fake_data_msg
                                .descriptor()
                                .get_field_by_name("min_count")
                                .map(|f| fake_data_msg.get_field(&f).as_ref().as_i32().unwrap_or(1))
                                .unwrap_or(1);

                            let max_c = fake_data_msg
                                .descriptor()
                                .get_field_by_name("max_count")
                                .map(|f| fake_data_msg.get_field(&f).as_ref().as_i32().unwrap_or(5))
                                .unwrap_or(5);

                            log::debug!(
                                "Field {}: Parsed options min_count={}, max_count={}",
                                field_full_name,
                                min_c,
                                max_c
                            );

                            fake_opt_data = Some((type_str, lang, min_c, max_c));
                        }
                    }
                }
            }
        }

        // Helper: Default fake data for Map Entries if explicit options are missing
        if fake_opt_data.is_none() && message_descr.is_map_entry() {
            let type_str = match field_kind {
                prost_reflect::Kind::String => "UUID", // Use UUID for unique string keys
                prost_reflect::Kind::Int32
                | prost_reflect::Kind::Sint32
                | prost_reflect::Kind::Sfixed32 => "Integer",
                prost_reflect::Kind::Int64
                | prost_reflect::Kind::Sint64
                | prost_reflect::Kind::Sfixed64 => "Integer", // FakeData::Integer handles casting ideally, or we need Int64? Integer is i32 in fake_data.rs, might truncate.
                prost_reflect::Kind::Uint32 | prost_reflect::Kind::Fixed32 => "WholeNumber",
                _ => "Word", // Fallback
            };
            fake_opt_data = Some((type_str.to_string(), SupportedLanguage::Default, 1, 1));
        }

        if let Some((data_type, field_lang, min_c, max_c)) = fake_opt_data {
            let language = choose_language(&field_lang, global_language, force_global_language);
            let min_count = max(min_c, 0);
            let max_count = max(max_c, max(min_count, 1));

            match field_kind {
                prost_reflect::Kind::Message(ref nested_msg_descr) => {
                    // Recursion
                    if field_descr.is_map() {
                        let mut map_val = HashMap::new();
                        let count = rng.random_range(min_count..=max_count);
                        // Map Entry has Key (1) and Value (2)
                        // But we need to verify we can access them. MapEntry is a Message.
                        if let (Some(key_field), Some(val_field)) =
                            (nested_msg_descr.get_field(1), nested_msg_descr.get_field(2))
                        {
                            for _ in 0..count {
                                let entry_msg = generate_fake_message_field(
                                    &nested_msg_descr,
                                    rng,
                                    output_format,
                                    global_language,
                                    force_global_language,
                                    runtime_descriptor_pool,
                                    options_map,
                                )?;

                                let key_val = entry_msg.get_field(&key_field);
                                let val_val = entry_msg.get_field(&val_field);

                                // Map keys must be converted to MapKey
                                // We handle basic types.
                                let map_key_opt = match key_val.as_ref() {
                                    Value::Bool(b) => Some(prost_reflect::MapKey::Bool(*b)),
                                    Value::I32(i) => Some(prost_reflect::MapKey::I32(*i)),
                                    Value::I64(i) => Some(prost_reflect::MapKey::I64(*i)),
                                    Value::U32(i) => Some(prost_reflect::MapKey::U32(*i)),
                                    Value::U64(i) => Some(prost_reflect::MapKey::U64(*i)),
                                    Value::String(s) => {
                                        Some(prost_reflect::MapKey::String(s.clone()))
                                    }
                                    _ => None,
                                };

                                if let Some(map_key) = map_key_opt {
                                    map_val.insert(map_key, val_val.as_ref().clone());
                                }
                            }
                        }
                        message.set_field(&field_descr, Value::Map(map_val));
                    } else if is_list_field {
                        let mut nested_list = Vec::new();
                        let count = rng.random_range(min_count..=max_count);
                        for _ in 0..count {
                            let msg = generate_fake_message_field(
                                &nested_msg_descr,
                                rng,
                                output_format,
                                global_language,
                                force_global_language,
                                runtime_descriptor_pool,
                                options_map,
                            )?;
                            nested_list.push(Value::Message(msg));
                        }
                        message.set_field(&field_descr, Value::List(nested_list));
                    } else {
                        let msg = generate_fake_message_field(
                            &nested_msg_descr,
                            rng,
                            output_format,
                            global_language,
                            force_global_language,
                            runtime_descriptor_pool,
                            options_map,
                        )?;
                        message.set_field(&field_descr, Value::Message(msg));
                    }
                }
                _ => {
                    // Primitive
                    if is_list_field {
                        let mut val_list = Vec::new();
                        let count = rng.random_range(min_count..=max_count);
                        for _ in 0..count {
                            let val = get_fake_data_output_value(
                                &data_type,
                                &language,
                                output_format,
                                &field_kind,
                            );
                            let DataType::Protobuf(v) = val;
                            val_list.push(v);
                        }
                        fake_field_value = Some(DataType::Protobuf(Value::List(val_list)));
                    } else if cardinality == Cardinality::Required {
                        fake_field_value = Some(get_fake_data_output_value(
                            &data_type,
                            &language,
                            output_format,
                            &field_kind,
                        ));
                    } else if cardinality == Cardinality::Optional {
                        // 60% chance or forced if min_count > 0
                        if min_count > 0 || rng.random_bool(0.6) {
                            fake_field_value = Some(get_fake_data_output_value(
                                &data_type,
                                &language,
                                output_format,
                                &field_kind,
                            ));
                        }
                    }
                }
            }
        }

        // Apply if primitive and set
        // Fix for partial move: use a reference to field_kind in the match guard
        if !matches!(field_kind, prost_reflect::Kind::Message(_)) {
            if let Some(val) = fake_field_value {
                let DataType::Protobuf(v) = val;
                message.set_field(&field_descr, v);
            }
        }
    }

    Ok(message)
}

fn build_options_map(request: &DynamicMessage, map: &mut HashMap<String, DynamicMessage>) {
    log::debug!("Entring build_options_map");
    if let Some(field) = request.descriptor().get_field_by_name("proto_file") {
        if let Value::List(files) = request.get_field(&field).as_ref() {
            log::debug!("build_options_map: Found {} files in request", files.len());
            for file_val in files {
                if let Value::Message(file_msg) = file_val {
                    let name = file_msg
                        .get_field_by_name("name")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    log::debug!("build_options_map: Processing file {}", name);
                    // Get package
                    let mut package = String::new();
                    if let Some(pkg_field) = file_msg.descriptor().get_field_by_name("package") {
                        if let Value::String(s) = file_msg.get_field(&pkg_field).as_ref() {
                            package = s.clone();
                        }
                    }

                    // Process messages
                    if let Some(msg_type_field) =
                        file_msg.descriptor().get_field_by_name("message_type")
                    {
                        if let Value::List(msgs) = file_msg.get_field(&msg_type_field).as_ref() {
                            for msg_val in msgs {
                                if let Value::Message(msg_msg) = msg_val {
                                    process_message_options(msg_msg, &package, map);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn process_message_options(
    msg_msg: &DynamicMessage,
    scope: &str,
    map: &mut HashMap<String, DynamicMessage>,
) {
    // Determine full name
    let name_field = msg_msg.descriptor().get_field_by_name("name").unwrap();
    let name_val = msg_msg.get_field(&name_field);
    let name = name_val.as_str().unwrap_or("").to_string();

    let full_name = if scope.is_empty() {
        name.clone()
    } else {
        format!("{}.{}", scope, name)
    };

    // Store options
    if let Some(options_field) = msg_msg.descriptor().get_field_by_name("options") {
        if msg_msg.has_field(&options_field) {
            let options_val = msg_msg.get_field(&options_field);
            if let Value::Message(options_msg) = options_val.as_ref() {
                map.insert(full_name.clone(), options_msg.clone());
                log::debug!("Stored options for message: {}", full_name);
            }
        }
    }

    // Process nested messages
    if let Some(nested_type_field) = msg_msg.descriptor().get_field_by_name("nested_type") {
        if let Value::List(nested_msgs) = msg_msg.get_field(&nested_type_field).as_ref() {
            for nested_val in nested_msgs {
                if let Value::Message(nested_msg) = nested_val {
                    process_message_options(nested_msg, &full_name, map);
                }
            }
        }
    }

    // Process fields
    if let Some(field_field) = msg_msg.descriptor().get_field_by_name("field") {
        if let Value::List(fields) = msg_msg.get_field(&field_field).as_ref() {
            for field_val in fields {
                if let Value::Message(field_msg) = field_val {
                    // Field name
                    let fname_field = field_msg.descriptor().get_field_by_name("name").unwrap();
                    let fname = field_msg
                        .get_field(&fname_field)
                        .as_str()
                        .unwrap_or("")
                        .to_string();
                    let field_full_name = format!("{}.{}", full_name, fname);

                    // Field options
                    if let Some(fopt_field) = field_msg.descriptor().get_field_by_name("options") {
                        if field_msg.has_field(&fopt_field) {
                            let fopt_val = field_msg.get_field(&fopt_field);
                            if let Value::Message(fopt_msg) = fopt_val.as_ref() {
                                map.insert(field_full_name.clone(), fopt_msg.clone());
                                log::debug!("Stored options for field: {}", field_full_name);
                            }
                        }
                    }
                }
            }
        }
    }
}
