use protobuf_codegen::Customize;
use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Base directory for proto files
    let proto_dir = "proto";

    // let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new("src/gen");
    std::fs::create_dir_all(dest_path).unwrap();

    // Tell cargo to rerun this script if any proto files change
    println!("cargo:rerun-if-changed={}", proto_dir);

    // // Get the output directory from Cargo
    // let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // First generate into OUT_DIR (temporary location)
    let out_dir = env::var("OUT_DIR").unwrap();
    println!(
        "cargo:warning=Generating Rust code from protos into {}",
        out_dir
    );

    // Create a new Codegen instance
    protobuf_codegen::Codegen::new()
        // .pure()
        .protoc()
        .include(proto_dir)
        .input(&format!("{}/gen_fake/fake_field.proto", proto_dir))
        // .inputs(&[
        //     format!("{}/gen_fake/fake_field.proto", proto_dir),
        //     format!("{}/examples/user.proto", proto_dir),
        // ])
        // .includes(&[proto_dir])
        // .out_dir(&out_dir)
        // .customize(Customize::default())
        // .cargo_out_dir("gen")
        .out_dir(&out_dir) // Output to OUT_DIR first
        .run_from_script();

    // Copy the generated file to src/gen
    let generated_file = Path::new(&out_dir).join("fake_field.rs");
    if generated_file.exists() {
        let target_file = dest_path.join("fake_field.rs");
        fs::copy(&generated_file, &target_file)?;
        println!("cargo:warning=Copied generated file to {:?}", target_file);
    } else {
        println!(
            "cargo:warning=Generated file not found at {:?}",
            generated_file
        );
    }

    // Print completion message for debugging
    println!("cargo:warning=Proto code generation complete");

    Ok(())
}
