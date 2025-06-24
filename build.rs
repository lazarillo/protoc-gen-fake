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

    // Define the Cargo's output directory for generated Rust files.
    // This is where `protobuf-codegen` will initially place its output.
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
        .inputs(&[format!("{}/gen_fake/fake_field.proto", proto_dir)])
        .includes(&[proto_dir])
        .out_dir(&out_dir_cargo) // Generate to Cargo's OUT_DIR initially
        .run_from_script();

    println!(
        "cargo:warning=Rust protobuf code generation complete in OUT_DIR. Now copying and cleaning to src/gen/."
    );

    // Post-process and copy the generated `fake_field.rs` from OUT_DIR to src/gen/
    filter_attributes(
        &out_dir_cargo.join("fake_field.rs"),
        &src_gen_dir.join("fake_field.rs"),
    )?;

    // Post-process and copy the generated `mod.rs` file from OUT_DIR to src/gen/
    filter_attributes(&out_dir_cargo.join("mod.rs"), &src_gen_dir.join("mod.rs"))?;

    Ok(())
}
