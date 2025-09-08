# Dockerfile for AI Data Service
FROM rust:1.80-slim as builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    clang \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace for dependency resolution
COPY Cargo.toml Cargo.lock ./
COPY services/ai_data/Cargo.toml ./services/ai_data/
COPY libs/ ./libs/

# Copy source code  
COPY services/ai_data/src ./services/ai_data/src

# Build the service (this will build the NAR wrapper with mock FFI)
RUN cargo build --release --package ai-data-service

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /usr/src/app/target/release/ai-data-service /usr/local/bin/ai-data-service

# Set environment variables for the NAR engine
ENV MODEL_PATH="models/gemma-3-1b.gguf"
ENV NAR_THREADS="4"

EXPOSE 3004

CMD ["ai-data-service"]