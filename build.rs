// build.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This build script is now minimal as gRPC/protobuf compilation
    // is handled outside of this script (e.g., pre-generated code).

    // You might still want this if you make changes to build.rs itself.
    println!("cargo:rerun-if-changed=build.rs");
    
    // If you have other build-time tasks unrelated to gRPC, add them here.

    Ok(())
}
