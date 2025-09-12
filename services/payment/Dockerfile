# Multi-stage Dockerfile for Service
# Optimized for security and minimal size per Task 0.4 requirements

# Stage 1: Builder
FROM rust:1.80.0-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy service files
COPY services/payment/ ./
# Copy rust toolchain if it exists
COPY rust-toolchain.tom[l] ./ 

# Build the service in release mode
RUN cargo build --release --bin payment-service

# Stage 2: Runtime (minimal distroless image for security)
FROM gcr.io/distroless/cc-debian12:nonroot

# Copy the binary from builder
COPY --from=builder --chown=nonroot:nonroot \
    /usr/src/app/target/release/payment-service /usr/local/bin/payment-service

# Set non-root user (distroless nonroot has UID 65532)
USER nonroot

# Expose the service port
EXPOSE 8080

# Note: Health checks disabled for distroless (no shell)
# Health checks should be configured in orchestration layer (k8s/docker-compose)

# Run the service
ENTRYPOINT ["/usr/local/bin/payment-service"]
