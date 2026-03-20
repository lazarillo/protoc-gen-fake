use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let proto_dir = "proto"; // Root of your .proto files
    let out_dir_cargo = PathBuf::from(env::var("OUT_DIR")?); // Cargo's standard output directory

    // --- RERUN-IF-CHANGED RULES ---
    println!(
        "cargo:rerun-if-changed={}/gen_fake/fake_field.proto",
        proto_dir
    );
    println!(
        "cargo:rerun-if-changed={}/google/protobuf/descriptor.proto",
        proto_dir
    );
    // plugin.proto is standard, usually doesn't change, but good to keep if we modify it (we don't here)
    println!(
        "cargo:rerun-if-changed={}/google/protobuf/compiler/plugin.proto",
        proto_dir
    );

    // --- PROST-BUILD FOR PROST TYPES AND FILE DESCRIPTOR SET ---
    // This generates:
    // 1. Rust structs compatible with `prost` (e.g., for `FakeDataFieldOption`) into `src/gen_prost`.
    // 2. A binary `FileDescriptorSet` (`file_descriptor_set.bin`) for `prost-reflect`'s runtime use.

    let prost_gen_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("src")
        .join("gen_prost");
    std::fs::create_dir_all(&prost_gen_dir)?;

    let descriptor_file_name = "file_descriptor_set.bin";
    let descriptor_full_path_buf = out_dir_cargo.join(descriptor_file_name);

    let bootstrap_descriptor_file_name = "bootstrap_descriptors.bin";
    let bootstrap_descriptor_path_buf = out_dir_cargo.join(bootstrap_descriptor_file_name);

    let mut prost_config = prost_build::Config::new();
    prost_config
        .out_dir(&prost_gen_dir)
        .file_descriptor_set_path(&descriptor_full_path_buf)
        .compile_protos(
            &[
                format!("{}/gen_fake/fake_field.proto", proto_dir),
                format!("{}/google/protobuf/descriptor.proto", proto_dir),
            ],
            &[proto_dir],
        )?;

    // Separate config for bootstrap to avoid overwriting or mixing if needed,
    // though we could theoretically combine them.
    // But we need plugin.proto specifically for the Request parsing.
    let mut bootstrap_config = prost_build::Config::new();
    bootstrap_config
        .out_dir(&prost_gen_dir) // Reuse gen dir? Or temp? Doesn't matter as we don't compile rust code from it necessarily.
        .file_descriptor_set_path(&bootstrap_descriptor_path_buf)
        .compile_protos(
            &[
                format!("{}/google/protobuf/compiler/plugin.proto", proto_dir),
                format!("{}/gen_fake/fake_field.proto", proto_dir),
                format!("{}/google/protobuf/descriptor.proto", proto_dir),
            ],
            &[proto_dir],
        )?;

    // Tell Cargo to set an environment variable pointing to the generated descriptor set.
    // This path will be used by `include_bytes!` in `main.rs` for the STATIC pool.
    println!(
        "cargo:rustc-env=DESCRIPTOR_SET_BIN_PATH={}",
        descriptor_full_path_buf.display()
    );
    println!(
        "cargo:rustc-env=BOOTSTRAP_DESCRIPTOR_SET_BIN_PATH={}",
        bootstrap_descriptor_path_buf.display()
    );

    Ok(())
}
