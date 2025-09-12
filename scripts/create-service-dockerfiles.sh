#!/bin/bash
# Create Dockerfiles for all backend services

services=(
    "marketplace:8081"
    "identity:8082"
    "player_account:8083"
    "ai_data:8084"
    "account:8085"
    "feedback:8086"
    "mission:8087"
    "payment:8088"
    "social:8089"
)

for service_info in "${services[@]}"; do
    IFS=':' read -r service port <<< "$service_info"
    
    if [ "$service" == "indexer" ]; then
        continue  # Already created
    fi
    
    cat > "services/$service/Dockerfile" << EOF
# Multi-stage Dockerfile for ${service^} Service
# Optimized for security and minimal size per Task 0.4 requirements

# Stage 1: Builder
FROM rust:1.80.0-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \\
    pkg-config \\
    libssl-dev \\
    protobuf-compiler \\
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy workspace files for better caching
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY libs/ ./libs/
COPY schemas/ ./schemas/
COPY contracts/ ./contracts/
COPY services/ ./services/

# Build the service in release mode
RUN cargo build --release --bin ${service}-service

# Stage 2: Runtime (minimal distroless image for security)
FROM gcr.io/distroless/cc-debian12:nonroot

# Copy the binary from builder
COPY --from=builder --chown=nonroot:nonroot \\
    /usr/src/app/target/release/${service}-service /usr/local/bin/${service}-service

# Set non-root user (distroless nonroot has UID 65532)
USER nonroot

# Expose the service port
EXPOSE $port

# Health check endpoint
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \\
    CMD ["/usr/local/bin/${service}-service", "--health-check"]

# Run the service
ENTRYPOINT ["/usr/local/bin/${service}-service"]
EOF
    
    echo "Created Dockerfile for $service service on port $port"
done

echo "All Dockerfiles created successfully!"