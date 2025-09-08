# Dockerfile for Indexer Service
FROM rust:1.80-slim as builder

WORKDIR /usr/src/app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace first
COPY Cargo.toml Cargo.lock ./
COPY services/indexer/Cargo.toml ./services/indexer/

# Create a minimal main.rs for the build context
RUN mkdir -p services/indexer/src && \
    echo 'fn main() { println!("Indexer service starting..."); }' > services/indexer/src/main.rs

# Build the service
RUN cargo build --release --package indexer-service

# Runtime stage  
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /usr/src/app/target/release/indexer-service /usr/local/bin/indexer-service

EXPOSE 3003

CMD ["indexer-service"]