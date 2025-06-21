use prost_build::Config;
use std::env;
use std::error::Error;
use std::io::Result as IoResult;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let proto_dir = "proto"; // Root of your .proto files
    let out_dir = PathBuf::from(env::var("OUT_DIR")?); // Cargo's standard output directory

    // Tell Cargo to re-run this script if any of your custom proto files change.
    println!(
        "cargo:rerun-if-changed={}/gen_fake/fake_field.proto",
        proto_dir
    );
    println!("cargo:rerun-if-changed={}/examples/user.proto", proto_dir);
    // You should still have descriptor.proto and plugin.proto in your proto/google/protobuf/ directories
    // for protoc to find them, even if prost-types provides the Rust structs.
    println!(
        "cargo:rerun-if-changed={}/google/protobuf/descriptor.proto",
        proto_dir
    );
    println!(
        "cargo:rerun-if-changed={}/google/protobuf/compiler/plugin.proto",
        proto_dir
    );

    // --- PROST-BUILD FOR RUST STRUCTS ---
    // This part generates your `src/prost_generated/gen_fake.rs`
    let prost_generated_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("src")
        .join("gen");
    std::fs::create_dir_all(&prost_generated_dir)?; // Ensure directory exists

    println!(
        "cargo:warning=Generating Rust code from protos into {:?}",
        prost_generated_dir
    );

    Config::new()
        .out_dir(&prost_generated_dir)
        // Compile ONLY your custom .proto files.
        // prost-types handles Google's well-known types.
        .compile_protos(
            &[format!("{}/gen_fake/fake_field.proto", proto_dir)],
            &[proto_dir], // Include path for protoc to find imported protos
        )?;

    println!("cargo:warning=Prost code generation complete");

    // --- PROST-REFLECT: GENERATE FILE DESCRIPTOR SET ---
    // This generates a binary FileDescriptorSet that prost-reflect can load at runtime
    // to perform reflection, including accessing custom options.
    let descriptor_file_name = "file_descriptor_set.bin";
    let descriptor_full_path_buf = out_dir.join(descriptor_file_name); // This is a PathBuf

    // Convert PathBuf to String, which explicitly implements Into<PathBuf>.
    // This is the most robust way to ensure the trait bound is satisfied in tricky cases.
    let descriptor_full_path_str = descriptor_full_path_buf
        .to_str()
        .ok_or_else(|| "Path contains invalid Unicode")?
        .to_owned();

    println!(
        "cargo:warning=Generating FileDescriptorSet for prost-reflect into {:?}",
        descriptor_full_path_str
    );

    let mut prost_reflect_config = Config::new();
    prost_reflect_config
        // Pass the String directly, without `Some()`.
        .file_descriptor_set_path(descriptor_full_path_str)
        // Compile all protos that you need reflection for, including Google's,
        // because prost-reflect needs their schema definitions.
        .compile_protos(
            &[
                format!("{}/gen_fake/fake_field.proto", proto_dir),
                format!("{}/examples/user.proto", proto_dir), // User.proto for its options
                format!("{}/google/protobuf/descriptor.proto", proto_dir), // For FieldOptions definition
            ],
            &[proto_dir], // Include path for protoc to find imported protos
        )?;

    println!("cargo:warning=Prost-reflect descriptor set generation complete");

    Ok(())
}
