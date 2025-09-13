fn main() {
    // Note: feedback service doesn't have a protobuf definition yet
    // This is a placeholder for when it's added
    println!(
        "cargo:warning=Feedback service protobuf definition not found, skipping gRPC generation"
    );
}
