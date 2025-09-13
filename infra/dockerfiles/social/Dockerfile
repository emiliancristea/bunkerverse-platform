# Multi-stage Docker build for SOCIAL Service Smart Stub
# Security-hardened with minimal attack surface and non-root execution

#################################################
# Build Stage
#################################################
FROM rust:1.80-alpine AS builder

# Install build dependencies
RUN apk add --no-cache     musl-dev     pkgconfig     openssl-dev     openssl-libs-static

# Create app user for build
RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid 10001 appuser

# Set working directory
WORKDIR /build

# Copy workspace files for dependency resolution
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./
COPY schemas/ ./schemas/
COPY contracts/Cargo.toml ./contracts/Cargo.toml
COPY libs/common-rust/Cargo.toml ./libs/common-rust/Cargo.toml
COPY libs/nar-rust-wrapper-for-llama-cpp/Cargo.toml ./libs/nar-rust-wrapper-for-llama-cpp/Cargo.toml
COPY client/rust-app-logic/Cargo.toml ./client/rust-app-logic/Cargo.toml
COPY services/identity/Cargo.toml ./services/identity/Cargo.toml
COPY services/account/Cargo.toml ./services/account/Cargo.toml
COPY services/marketplace/Cargo.toml ./services/marketplace/Cargo.toml
COPY services/payment/Cargo.toml ./services/payment/Cargo.toml
COPY services/mission/Cargo.toml ./services/mission/Cargo.toml
COPY services/social/Cargo.toml ./services/social/Cargo.toml
COPY services/ai_data/Cargo.toml ./services/ai_data/Cargo.toml
COPY services/feedback/Cargo.toml ./services/feedback/Cargo.toml
COPY services/indexer/Cargo.toml ./services/indexer/Cargo.toml
COPY services/ipfs-mock/Cargo.toml ./services/ipfs-mock/Cargo.toml

# Create placeholder workspace structure for dependency pre-build for all services
RUN mkdir -p services/identity/src && echo "fn main() {}" > services/identity/src/main.rs
RUN mkdir -p services/account/src && echo "fn main() {}" > services/account/src/main.rs
RUN mkdir -p services/marketplace/src && echo "fn main() {}" > services/marketplace/src/main.rs
RUN mkdir -p services/payment/src && echo "fn main() {}" > services/payment/src/main.rs
RUN mkdir -p services/mission/src && echo "fn main() {}" > services/mission/src/main.rs
RUN mkdir -p services/social/src && echo "fn main() {}" > services/social/src/main.rs
RUN mkdir -p services/ai_data/src && echo "fn main() {}" > services/ai_data/src/main.rs
RUN mkdir -p services/feedback/src && echo "fn main() {}" > services/feedback/src/main.rs
RUN mkdir -p services/indexer/src && echo "fn main() {}" > services/indexer/src/main.rs
RUN mkdir -p services/ipfs-mock/src && echo "fn main() {}" > services/ipfs-mock/src/main.rs
RUN mkdir -p contracts/src && echo "fn main() {}" > contracts/src/lib.rs
RUN mkdir -p libs/common-rust/src && echo "fn main() {}" > libs/common-rust/src/lib.rs
RUN mkdir -p libs/nar-rust-wrapper-for-llama-cpp/src && echo "fn main() {}" > libs/nar-rust-wrapper-for-llama-cpp/src/lib.rs
RUN mkdir -p client/rust-app-logic/src && echo "fn main() {}" > client/rust-app-logic/src/lib.rs

# Pre-build dependencies (this layer will be cached)
RUN cargo build --release --bin social-service

# Copy actual source code
COPY services/social/src ./services/social/src/

# Build the application binary
RUN cargo build --release --bin social-service --target x86_64-unknown-linux-musl

#################################################
# Runtime Stage - Distroless for minimal attack surface
#################################################
FROM gcr.io/distroless/static-debian12:nonroot

# Metadata labels for security and compliance
LABEL org.opencontainers.image.title="SOCIAL Service Smart Stub"       org.opencontainers.image.description="Security-hardened smart stub for social service"       org.opencontainers.image.version="1.0.0"       org.opencontainers.image.vendor="Bunkerverse Platform"       org.opencontainers.image.licenses="MIT"       org.opencontainers.image.source="https://github.com/emiliancristea/bunkerverse-platform"       security.scan-policy="required"       security.non-root="true"       security.read-only-root-filesystem="true"

# Copy the binary from builder stage
COPY --from=builder --chown=65532:65532     /build/target/x86_64-unknown-linux-musl/release/social-service     /usr/local/bin/social-service

# Set secure defaults
ENV RUST_LOG=info     RUST_BACKTRACE=1     ENABLE_CRYPTO=false

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3     CMD ["/usr/local/bin/social-service", "--health-check"] || exit 1

# Use non-root user (distroless nonroot user - UID 65532)
USER 65532:65532

# Expose service port
EXPOSE 8089

# Run the binary
ENTRYPOINT ["/usr/local/bin/social-service"]
