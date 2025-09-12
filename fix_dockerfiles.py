#!/usr/bin/env python3
"""
Script to fix all Dockerfiles to work with Rust workspace structure
"""

import os
import re
from pathlib import Path

def fix_dockerfile(dockerfile_path, service_name):
    """Fix a single Dockerfile for workspace builds"""
    print(f"Fixing {dockerfile_path} for service: {service_name}")
    
    with open(dockerfile_path, 'r') as f:
        content = f.read()
    
    # Pattern 1: Fix the dependency copy section
    old_copy_pattern = rf"# Copy dependency files first for better layer caching\nCOPY services/{service_name}/Cargo\.toml services/{service_name}/Cargo\.lock \./\n\n# Create src directory with placeholder main\.rs to enable dependency pre-build\nRUN mkdir src && echo \"fn main\(\) \{\}\" > src/main\.rs\n\n# Pre-build dependencies \(this layer will be cached\)\nRUN cargo build --release && rm -rf src target/release/deps/{service_name}_service\*"
    
    new_copy_pattern = f"""# Copy workspace files for dependency resolution
COPY Cargo.toml Cargo.lock ./
COPY services/{service_name}/Cargo.toml ./services/{service_name}/

# Create placeholder workspace structure for dependency pre-build
RUN mkdir -p services/{service_name}/src && echo "fn main() {{}}" > services/{service_name}/src/main.rs

# Pre-build dependencies (this layer will be cached)
RUN cargo build --release --bin {service_name}-service"""
    
    # Pattern 2: Fix the source copy
    old_src_pattern = f"# Copy source code\nCOPY services/{service_name}/src ./src/"
    new_src_pattern = f"# Copy actual source code\nCOPY services/{service_name}/src ./services/{service_name}/src/"
    
    # Pattern 3: Fix the final build command
    old_build_pattern = "# Build the application binary\nRUN cargo build --release --target x86_64-unknown-linux-musl"
    new_build_pattern = f"# Build the application binary\nRUN cargo build --release --bin {service_name}-service --target x86_64-unknown-linux-musl"
    
    # Apply fixes
    content = re.sub(old_copy_pattern, new_copy_pattern, content, flags=re.MULTILINE)
    content = re.sub(re.escape(old_src_pattern), new_src_pattern, content)
    content = re.sub(re.escape(old_build_pattern), new_build_pattern, content)
    
    # Write back
    with open(dockerfile_path, 'w') as f:
        f.write(content)
    
    print(f"✅ Fixed {dockerfile_path}")

def main():
    base_dir = Path("D:/code-dev/main/bunkercorporation/bunkerverse-platform")
    dockerfiles_dir = base_dir / "infra" / "dockerfiles"
    
    # Services to fix (excluding ipfs-mock which might be different)
    services = [
        "marketplace", "indexer", "identity", "ai_data", 
        "account", "mission", "payment", "social"
        # feedback already fixed manually
    ]
    
    for service in services:
        dockerfile_path = dockerfiles_dir / service / "Dockerfile"
        if dockerfile_path.exists():
            fix_dockerfile(dockerfile_path, service)
        else:
            print(f"❌ Dockerfile not found: {dockerfile_path}")
    
    print("\n🎉 All Dockerfiles fixed!")

if __name__ == "__main__":
    main()