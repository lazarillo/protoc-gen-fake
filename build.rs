use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

// Helper function to filter out inner attributes and inner doc comments
fn filter_attributes(input: &Path, output: &Path) -> io::Result<()> {
    let content = fs::read_to_string(input)?;
    let mut filtered_lines = Vec::new();

    for line in content.lines() {
        // Filter out lines that start with `#!` (inner attributes)
        // or `//!` (inner doc comments).
        // This is a common heuristic for generated Protobuf files.
        if !line.trim().starts_with("#!") && !line.trim().starts_with("//!") {
            filtered_lines.push(line);
        }
    }

    fs::write(output, filtered_lines.join("\n"))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Base directory for proto files
    let proto_dir = "proto";

    // Tell Cargo to rerun this build script if any of the proto files change.
    println!(
        "cargo:rerun-if-changed={}/gen_fake/fake_field.proto",
        proto_dir
    );
    println!("cargo:rerun-if-changed={}/examples/user.proto", proto_dir);
    println!(
        "cargo:rerun-if-changed={}/google/protobuf/descriptor.proto",
        proto_dir
    );
    println!(
        "cargo:rerun-if-changed={}/google/protobuf/compiler/plugin.proto",
        proto_dir
    );

    // Define the Cargo's output directory for generated Rust files.
    let out_dir_cargo = PathBuf::from(env::var("OUT_DIR")?);
    println!(
        "cargo:warning=protobuf-codegen will output to Cargo's OUT_DIR: {:?}",
        out_dir_cargo
    );

    // Define the *target* directory within your `src/` where you want the generated files to end up.
    let src_gen_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("src")
        .join("gen");
    fs::create_dir_all(&src_gen_dir)?; // Ensure the `src/gen` directory exists

    println!(
        "cargo:warning=Generated .rs files will be copied to your src/gen/ directory: {:?}",
        src_gen_dir
    );

    // Run protobuf_codegen to generate the .rs files into Cargo's OUT_DIR
    protobuf_codegen::Codegen::new()
        .inputs(&[
            format!("{}/gen_fake/fake_field.proto", proto_dir),
            format!("{}/examples/user.proto", proto_dir),
            format!("{}/google/protobuf/descriptor.proto", proto_dir),
            format!("{}/google/protobuf/compiler/plugin.proto", proto_dir),
        ])
        .includes(&[proto_dir])
        .out_dir(&out_dir_cargo) // Generate to Cargo's OUT_DIR initially
        .customize(protobuf_codegen::Customize::default().gen_mod_rs(false)) // Avoids gen_mod_rs
        .run_from_script();

    println!(
        "cargo:warning=Rust protobuf code generation complete in OUT_DIR. Now copying and cleaning to src/gen/."
    );

    // IMPORTANT: Post-process and copy the generated files from OUT_DIR to src/gen/
    // protobuf-codegen names output files based on the .proto file name.
    // So, `fake_field.proto` will become `fake_field.rs`.
    let generated_fake_field_rs_in_out_dir = out_dir_cargo.join("fake_field.rs");
    let target_fake_field_rs_in_src_gen = src_gen_dir.join("fake_field.rs");

    // Filter attributes and copy the cleaned file
    filter_attributes(
        &generated_fake_field_rs_in_out_dir,
        &target_fake_field_rs_in_src_gen,
    )?;

    Ok(())
}
