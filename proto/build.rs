use std::path::PathBuf;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("REBUILD_PROTO").is_err() {
        return Ok(());
    }

    let out_dir = PathBuf::from("./src/gen");
    fs::create_dir_all(&out_dir)?;

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .compile_protos(
            &[
                "../opentelemetry-proto/opentelemetry/proto/collector/profiles/v1development/profiles_service.proto",
            ],
            &["../opentelemetry-proto"],
        )?;

    Ok(())
}
