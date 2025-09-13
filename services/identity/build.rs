fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &[
                "../../schemas/proto/bunkerverse/core/v1/types.proto",
                "../../schemas/proto/bunkerverse/core/v1/enums.proto",
                "../../schemas/proto/bunkerverse/core/v1/events.proto",
                "../../schemas/proto/bunkerverse/services/v1/health_service.proto",
                "../../schemas/proto/bunkerverse/services/v1/identity_service.proto",
            ],
            &["../../schemas/proto"],
        )?;
    Ok(())
}
