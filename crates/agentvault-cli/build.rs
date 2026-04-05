use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto_root = manifest_dir.join("../../proto");
    println!("cargo:rerun-if-changed={}", proto_root.display());

    std::env::set_var("PROTOC", protobuf_src::protoc());
    let include = protobuf_src::include();

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile_protos(
            &[
                proto_root.join("agentvault/common/v1/common.proto"),
                proto_root.join("agentvault/policy/v1/policy.proto"),
                proto_root.join("agentvault/run/v1/run.proto"),
                proto_root.join("agentvault/incident/v1/incident.proto"),
                proto_root.join("agentvault/approval/v1/approval.proto"),
                proto_root.join("agentvault/learner/v1/learner.proto"),
            ],
            &[proto_root, include],
        )?;

    Ok(())
}
