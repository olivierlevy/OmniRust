// build.rs
use std::path::PathBuf;
use std::fs;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")
        .map_err(|e| format!("Failed to get CARGO_MANIFEST_DIR: {}", e))?);
    
    let proto_base_dir = manifest_dir.join("src").join("networking").join("grpc").join("protos");
    let out_dir_path = manifest_dir.join("src").join("networking").join("grpc").join("generated");

    // Ensure both proto input and generated output directories exist
    fs::create_dir_all(&proto_base_dir)
        .map_err(|e| format!("Failed to create proto base directory {:?}: {}", proto_base_dir, e))?;
    fs::create_dir_all(&out_dir_path)
        .map_err(|e| format!("Failed to create generated output directory {:?}: {}", out_dir_path, e))?;


    let common_proto_path = proto_base_dir.join("common.proto");
    let greeter_proto_path = proto_base_dir.join("greeter.proto");

    if !common_proto_path.exists() {
        fs::write(&common_proto_path, 
r#"syntax = "proto3";
package common;
message Empty {}
message StringMessage { string value = 1; }
"#).map_err(|e| format!("Failed to write common.proto: {}", e))?;
    }

    if !greeter_proto_path.exists() {
        fs::write(&greeter_proto_path, 
r#"syntax = "proto3";
package greeter;
import "common.proto";
service Greeter {
  rpc SayHello (HelloRequest) returns (HelloReply);
  rpc SayHelloAgain (HelloRequest) returns (HelloReply);
}
message HelloRequest { string name = 1; }
message HelloReply { string message = 1; }
"#).map_err(|e| format!("Failed to write greeter.proto: {}", e))?;
    }

    // Convert PathBufs to &str for tonic_build
    // It's crucial these paths are valid UTF-8 and exist when compile is called.
    let common_proto_str = common_proto_path.to_str()
        .ok_or_else(|| format!("Path is not valid UTF-8: {:?}", common_proto_path))?;
    let greeter_proto_str = greeter_proto_path.to_str()
        .ok_or_else(|| format!("Path is not valid UTF-8: {:?}", greeter_proto_path))?;
    let proto_base_dir_str = proto_base_dir.to_str()
        .ok_or_else(|| format!("Path is not valid UTF-8: {:?}", proto_base_dir))?;

    let proto_files_to_compile = &[
        common_proto_str,
        greeter_proto_str,
    ];
    
    let include_dirs = &[
        proto_base_dir_str,
    ];
    
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir_path) // Pass as &Path or &str
        .compile(
            proto_files_to_compile,
            include_dirs,
        )
        .map_err(|e| format!("tonic_build::compile failed: {}", e))?;

    println!("cargo:rerun-if-changed=build.rs");
    for proto_file_str in proto_files_to_compile {
        println!("cargo:rerun-if-changed={}", proto_file_str);
    }

    // Copy config.example.toml as config.toml to the output directory where the executable will be.
    let profile = env::var("PROFILE")?; // "debug" or "release"
    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.join("target"));
    let exe_out_dir = target_dir.join(profile);

    let example_config_src = manifest_dir.join("config.example.toml");
    // The destination file will be named config.toml
    let final_config_dest = exe_out_dir.join("config.toml"); 

    if example_config_src.exists() {
        // Ensure the destination directory exists
        if !exe_out_dir.exists() {
            fs::create_dir_all(&exe_out_dir)
                .map_err(|e| format!("Failed to create executable output directory {:?}: {}", exe_out_dir, e))?;
        }
        // Copy config.example.toml to target/{profile}/config.toml
        fs::copy(&example_config_src, &final_config_dest)
            .map_err(|e| format!("Failed to copy config.example.toml to {:?}: {}", final_config_dest, e))?;
        println!("cargo:rerun-if-changed=config.example.toml"); 
    } else {
        println!("cargo:warning=config.example.toml not found at project root, not creating a default config.toml in target directory.");
    }
    
    Ok(())
}
