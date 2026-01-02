use prost_build::Config as ProstConfig; // Alias to avoid name collision
use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        std::env::set_var("PROTOC", protobuf_src::protoc());
    }

    let proto_dir = "proto"; // Root of your .proto files
    let out_dir_cargo = PathBuf::from(env::var("OUT_DIR")?); // Cargo's standard output directory

    // --- RERUN-IF-CHANGED RULES ---
    // These ensure Cargo rebuilds if the proto files change.
    println!(
        "cargo:rerun-if-changed={}/gen_fake/fake_field.proto",
        proto_dir
    );
    println!(
        "cargo:rerun-if-changed={}/google/protobuf/descriptor.proto",
        proto_dir
    );
    println!(
        "cargo:rerun-if-changed={}/google/protobuf/compiler/plugin.proto",
        proto_dir
    );

    // --- PROTOBUF-CODEGEN FOR RUST-PROTOBUF TYPES ---
    // This generates `fake_field.rs` for `rust-protobuf`'s use (for custom option parsing).
    let protobuf_gen_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("src")
        .join("gen_protobuf");
    std::fs::create_dir_all(&protobuf_gen_dir)?;
    println!(
        "cargo:warning=Generating rust-protobuf code into {:?}",
        protobuf_gen_dir
    );
    protobuf_codegen::Codegen::new()
        .inputs(&[format!("{}/gen_fake/fake_field.proto", proto_dir)])
        .includes(&[proto_dir])
        .out_dir(&protobuf_gen_dir)
        .customize(protobuf_codegen::Customize::default().gen_mod_rs(false))
        .run_from_script();
    println!("cargo:warning=rust-protobuf code generation complete");

    // --- PROST-BUILD FOR PROST TYPES (NOT USED) AND FILE DESCRIPTOR SET ---
    // This generates:
    // 1. Rust structs compatible with `prost` (e.g., for `FakeDataFieldOption`) into `src/gen_prost`.
    //    (Note: These are technically generated but we rely on `rust-protobuf`'s `FakeDataFieldOption` for parsing.)
    // 2. A binary `FileDescriptorSet` (`file_descriptor_set.bin`) for `prost-reflect`'s runtime use.
    //    This FileDescriptorSet *only* contains the schema for your custom option and google.protobuf.descriptor,
    //    NOT user-defined protos like `user.proto`.
    let prost_gen_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("src")
        .join("gen_prost");
    std::fs::create_dir_all(&prost_gen_dir)?;
    println!(
        "cargo:warning=Generating Prost code into {:?}",
        prost_gen_dir
    );

    let descriptor_file_name = "file_descriptor_set.bin";
    let descriptor_full_path_buf = out_dir_cargo.join(descriptor_file_name);
    let descriptor_full_path_str = descriptor_full_path_buf
        .to_str()
        .ok_or_else(|| "Path contains invalid Unicode")?
        .to_owned();

    let mut prost_config = ProstConfig::new();
    prost_config
        .out_dir(&prost_gen_dir)
        // Generate the FileDescriptorSet.
        // CRUCIAL: Only include `fake_field.proto` and `descriptor.proto` here.
        // user.proto is NOT included, as its schema will be provided at runtime via CodeGeneratorRequest.
        .file_descriptor_set_path(&descriptor_full_path_str)
        .compile_protos(
            &[
                format!("{}/gen_fake/fake_field.proto", proto_dir),
                format!("{}/google/protobuf/descriptor.proto", proto_dir),
            ],
            &[proto_dir],
        )?;

    println!("cargo:warning=Prost code generation complete");
    println!("cargo:warning=Prost-reflect descriptor set generation complete");

    // Tell Cargo to set an environment variable pointing to the generated descriptor set.
    // This path will be used by `include_bytes!` in `main.rs` for the STATIC pool.
    println!(
        "cargo:rustc-env=DESCRIPTOR_SET_BIN_PATH={}",
        descriptor_full_path_buf.display()
    );

    Ok(())
}
