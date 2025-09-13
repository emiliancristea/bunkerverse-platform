fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Protobuf build configuration for social service
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &[
                "../../schemas/proto/bunkerverse/services/v1/social_service.proto",
                "../../schemas/proto/bunkerverse/services/v1/health_service.proto",
                "../../schemas/proto/bunkerverse/core/v1/types.proto",
                "../../schemas/proto/bunkerverse/core/v1/enums.proto",
                "../../schemas/proto/bunkerverse/core/v1/events.proto",
                "../../schemas/proto/bunkerverse/core/v1/transactions.proto",
            ],
            &["../../schemas/proto"],
        )?;

    println!(
        "cargo:rerun-if-changed=../../schemas/proto/bunkerverse/services/v1/social_service.proto"
    );
    println!(
        "cargo:rerun-if-changed=../../schemas/proto/bunkerverse/services/v1/health_service.proto"
    );
    println!("cargo:rerun-if-changed=../../schemas/proto/bunkerverse/core/v1/types.proto");
    println!("cargo:rerun-if-changed=../../schemas/proto/bunkerverse/core/v1/enums.proto");
    println!("cargo:rerun-if-changed=../../schemas/proto/bunkerverse/core/v1/events.proto");
    println!("cargo:rerun-if-changed=../../schemas/proto/bunkerverse/core/v1/transactions.proto");

    Ok(())
}
