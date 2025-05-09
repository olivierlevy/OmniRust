// build.rs
use prost_build; // Explicitly import prost_build

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the path to your .proto files
    let proto_dir = "src/networking/grpc/protos";
    let proto_files = &[
        &format!("{}/common.proto", proto_dir),
        &format!("{}/greeter.proto", proto_dir),
        // Add more .proto files here as needed
    ];

    // Create the directory if it doesn't exist, to avoid build errors if no protos are defined yet.
    std::fs::create_dir_all(proto_dir)?;
    
    // Create dummy proto files if they don't exist, to allow the build to pass initially.
    if !std::path::Path::new(proto_files[0]).exists() {
        std::fs::write(proto_files[0], 
r#"syntax = "proto3";

package common;

message Empty {}

message StringMessage {
    string value = 1;
}
"#)?;
    }
    if !std::path::Path::new(proto_files[1]).exists() {
        std::fs::write(proto_files[1], 
r#"syntax = "proto3";

package greeter;

import "common.proto";

service Greeter {
  rpc SayHello (HelloRequest) returns (HelloReply);
  rpc SayHelloAgain (HelloRequest) returns (HelloReply);
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
}
"#)?;
    }

    // Create a new prost_build::Config instance.
    let prost_config = prost_build::Config::new();
    
    // Example customization for prost_config (optional):
    // prost_config.btree_map(&["."]); // To use BTreeMap for all map fields
    // prost_config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/networking/grpc/generated")
        // Pass the configured prost_build::Config to tonic_build
        .compile_protos_with_config(
            prost_config,
            proto_files, // List of .proto files to compile
            &[proto_dir], // Include path for .proto files
        )?;

    println!("cargo:rerun-if-changed=build.rs");
    for proto_file in proto_files {
        println!("cargo:rerun-if-changed={}", proto_file);
    }
    
    Ok(())
}
