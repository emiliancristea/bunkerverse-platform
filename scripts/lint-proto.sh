#!/bin/bash
# Protocol Buffer Linting Script
# Runs comprehensive validation and linting for all .proto files

set -euo pipefail

echo "🔍 Running Protocol Buffer static analysis..."

# Check if buf is installed
if ! command -v buf &> /dev/null; then
    echo "❌ buf CLI not found. Installing..."
    curl -sSL "https://github.com/bufbuild/buf/releases/latest/download/buf-$(uname -s)-$(uname -m)" \
        -o "/usr/local/bin/buf" && chmod +x "/usr/local/bin/buf"
fi

# Validate buf.yaml configuration
echo "📋 Validating buf configuration..."
buf config validate

# Run linting
echo "🧹 Linting Protocol Buffer files..."
buf lint

# Check for breaking changes (if previous version exists)
if [ -f "buf.lock" ]; then
    echo "🔄 Checking for breaking changes..."
    buf breaking --against buf.lock
fi

# Generate code from protobuf schemas
echo "🔧 Generating code from Protocol Buffer schemas..."
buf generate

# Validate generated code compiles
echo "🔬 Validating generated Rust code compiles..."
cd libs/common-rust && cargo check

echo "✅ Protocol Buffer validation completed successfully!"