# Dockerfile for Marketplace Service
FROM rust:1.80-slim as builder

WORKDIR /usr/src/app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace first  
COPY Cargo.toml Cargo.lock ./
COPY services/marketplace/Cargo.toml ./services/marketplace/

# Create a minimal main.rs for the build context
RUN mkdir -p services/marketplace/src && \
    echo 'fn main() { println!("Marketplace service starting..."); }' > services/marketplace/src/main.rs

# Build the service
RUN cargo build --release --package marketplace-service

# Runtime stage
FROM debian:bookworm-slim  

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /usr/src/app/target/release/marketplace-service /usr/local/bin/marketplace-service

EXPOSE 3005

CMD ["marketplace-service"]