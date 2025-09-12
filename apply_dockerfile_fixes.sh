#!/bin/bash

# Apply Dockerfile fixes to all services
services=("marketplace" "indexer" "identity" "ai_data" "account" "mission" "payment" "social" "ipfs-mock")

for service in "${services[@]}"; do
    dockerfile="infra/dockerfiles/$service/Dockerfile"
    
    if [ -f "$dockerfile" ]; then
        echo "Fixing $dockerfile..."
        
        # Create a backup
        cp "$dockerfile" "$dockerfile.backup"
        
        # Create the common workspace setup section
        cat > temp_workspace_setup.txt << 'EOF'
# Copy workspace files for dependency resolution
COPY Cargo-docker.toml ./Cargo.toml
COPY Cargo.lock ./
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
EOF
        
        # Extract the binary name for this service
        if [ "$service" = "ipfs-mock" ]; then
            binary_name="ipfs-mock"
        else
            binary_name="$service-service"
        fi
        
        # Use sed to replace the old patterns with new ones
        sed -i 's|# Copy dependency files first for better layer caching|# PLACEHOLDER_WORKSPACE_SETUP|g' "$dockerfile"
        sed -i "/# PLACEHOLDER_WORKSPACE_SETUP/r temp_workspace_setup.txt" "$dockerfile"
        sed -i '/# PLACEHOLDER_WORKSPACE_SETUP/d' "$dockerfile"
        
        # Remove old copy and mkdir lines
        sed -i '/^COPY services\/.*\/Cargo\.toml services\/.*\/Cargo\.lock \.\/$/d' "$dockerfile"
        sed -i '/^# Create src directory with placeholder/d' "$dockerfile"
        sed -i '/^RUN mkdir src && echo "fn main() {}" > src\/main\.rs$/d' "$dockerfile"
        sed -i '/^# Pre-build dependencies (this layer will be cached)$/d' "$dockerfile"
        sed -i '/^RUN cargo build --release && rm -rf src target\/release\/deps\/.*$/d' "$dockerfile"
        sed -i '/^# Copy source code$/d' "$dockerfile"
        sed -i '/^COPY services\/.*\/src \.\/src\/$/d' "$dockerfile"
        
        # Update the build commands
        sed -i "s|RUN cargo build --release --target x86_64-unknown-linux-musl|RUN cargo build --release --bin $binary_name --target x86_64-unknown-linux-musl|g" "$dockerfile"
        sed -i "s|# Copy actual source code|# Copy actual source code|g" "$dockerfile"
        sed -i "s|COPY services/$service/src ./services/$service/src/|COPY services/$service/src ./services/$service/src/|g" "$dockerfile"
        
        # Clean up temporary file
        rm temp_workspace_setup.txt
        
        echo "✅ Fixed $dockerfile"
    else
        echo "❌ $dockerfile not found"
    fi
done

echo "All Dockerfile fixes applied!"