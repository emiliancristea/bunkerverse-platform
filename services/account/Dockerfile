# Multi-stage Dockerfile for Account Service
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
COPY services/account/ ./
# Copy rust toolchain if it exists
COPY rust-toolchain.tom[l] ./ 

# Build the service in release mode
RUN cargo build --release --bin account-service

# Stage 2: Runtime (minimal distroless image for security)
FROM gcr.io/distroless/cc-debian12:nonroot

# Copy the binary from builder
COPY --from=builder --chown=nonroot:nonroot \
    /usr/src/app/target/release/account-service /usr/local/bin/account-service

# Set non-root user (distroless nonroot has UID 65532)
USER nonroot

# Expose the service port
EXPOSE 8085

# Health check endpoint
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/account-service", "--health-check"]

# Run the service
ENTRYPOINT ["/usr/local/bin/account-service"]
