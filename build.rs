use prost_build::Config;
use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    // Base directory for proto files
    let proto_dir = "proto";

    // Tell Cargo that if any proto files change, to rerun this build script.
    println!(
        "cargo:rerun-if-changed={}/gen_fake/fake_field.proto",
        proto_dir
    );
    println!("cargo:rerun-if-changed={}/examples/user.proto", proto_dir);

    // Get the CARGO_MANIFEST_DIR environment variable.
    let manifest_dir_str = env::var("CARGO_MANIFEST_DIR").map_err(|e| Box::<dyn Error>::from(e))?;

    // Define the output directory for generated Rust files.
    let out_dir = PathBuf::from(manifest_dir_str)
        .join("src")
        .join("prost_generated");

    // Create the output directory if it doesn't exist.
    std::fs::create_dir_all(&out_dir)?;

    println!(
        "cargo:warning=Generating Rust code from protos into {:?}",
        out_dir
    );

    Config::new()
        .out_dir(&out_dir)
        // Only compile your custom proto files here.
        // Google's well-known types (like descriptor.proto) are provided by `prost-types`.
        .compile_protos(
            &[format!("{}/gen_fake/fake_field.proto", proto_dir)],
            &[proto_dir], // The include path tells prost where to find imported .proto files
        )?;

    println!("cargo:warning=Prost code generation complete");

    Ok(())
}
