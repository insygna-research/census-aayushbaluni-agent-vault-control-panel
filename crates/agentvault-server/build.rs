use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../proto");

    println!("cargo:rerun-if-changed={}", proto_root.display());

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                proto_root.join("agentvault/common/v1/common.proto"),
                proto_root.join("agentvault/policy/v1/policy.proto"),
                proto_root.join("agentvault/run/v1/run.proto"),
                proto_root.join("agentvault/approval/v1/approval.proto"),
                proto_root.join("agentvault/incident/v1/incident.proto"),
                proto_root.join("agentvault/learner/v1/learner.proto"),
            ],
            &[proto_root, PathBuf::from(include_dir())],
        )?;

    Ok(())
}

/// Well-known types (google/protobuf/*.proto) ship with the system protoc (`protobuf-compiler` +
/// `libprotobuf-dev` in the Dockerfile). `PROTOC_INCLUDE` overrides the location.
fn include_dir() -> String {
    std::env::var("PROTOC_INCLUDE").unwrap_or_else(|_| "/usr/include".to_string())
}
