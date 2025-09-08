# Dockerfile for Player Account Service
FROM rust:1.80-slim as builder

WORKDIR /usr/src/app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace first for dependency resolution
COPY Cargo.toml Cargo.lock ./
COPY services/player_account/Cargo.toml ./services/player_account/
COPY libs/ ./libs/

# Copy source code
COPY services/player_account/src ./services/player_account/src

# Build the service
RUN cargo build --release --package player-account-service

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /usr/src/app/target/release/player-account-service /usr/local/bin/player-account-service

EXPOSE 3002

CMD ["player-account-service"]