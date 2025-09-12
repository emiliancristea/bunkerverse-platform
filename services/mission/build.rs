fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("src/generated")
        .compile(
            &[
                "../../schemas/proto/bunkerverse/services/v1/mission_service.proto",
                "../../schemas/proto/bunkerverse/services/v1/health_service.proto",
            ],
            &["../../schemas/proto"],
        )?;
    Ok(())
}
