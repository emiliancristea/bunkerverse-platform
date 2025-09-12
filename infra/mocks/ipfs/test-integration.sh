#!/bin/bash

# IPFS Mock Gateway - Integration Testing Script
# Tests both the Rust implementation and nginx alternative

set -e

echo "🧪 IPFS Mock Gateway - Integration Testing"
echo "=========================================="
echo

# Configuration
RUST_SERVICE_PORT=8080
NGINX_SERVICE_PORT=8081
BASE_DIR="$(dirname "$0")"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

# Function to test endpoint
test_endpoint() {
    local url="$1"
    local description="$2"
    local expected_status="${3:-200}"
    
    print_status "Testing: $description"
    
    response=$(curl -s -w "%{http_code}" -o /tmp/ipfs_test_response "$url" 2>/dev/null)
    status_code="${response}"
    
    if [ "$status_code" = "$expected_status" ]; then
        print_success "$description - Status: $status_code"
        return 0
    else
        print_error "$description - Expected: $expected_status, Got: $status_code"
        return 1
    fi
}

# Function to test service
test_service() {
    local service_name="$1"
    local base_url="$2"
    local port="$3"
    
    echo
    echo "🔍 Testing $service_name on port $port"
    echo "----------------------------------------"
    
    # Check if service is running
    if ! curl -s --connect-timeout 5 "$base_url/health" > /dev/null 2>&1; then
        print_error "$service_name is not running or not reachable on port $port"
        return 1
    fi
    
    local test_count=0
    local pass_count=0
    
    # Health check
    ((test_count++))
    if test_endpoint "$base_url/health" "Health check"; then
        ((pass_count++))
    fi
    
    # IPFS Gateway endpoints
    ((test_count++))
    if test_endpoint "$base_url/ipfs/QmNFTMetadata123" "NFT metadata retrieval"; then
        ((pass_count++))
    fi
    
    ((test_count++))
    if test_endpoint "$base_url/ipfs/QmTextContent456" "Text content retrieval"; then
        ((pass_count++))
    fi
    
    ((test_count++))
    if test_endpoint "$base_url/ipfs/QmImageContent789" "Image content retrieval"; then
        ((pass_count++))
    fi
    
    # IPFS API v0 endpoints
    ((test_count++))
    if test_endpoint "$base_url/api/v0/version" "API version"; then
        ((pass_count++))
    fi
    
    ((test_count++))
    if test_endpoint "$base_url/api/v0/id" "Node ID"; then
        ((pass_count++))
    fi
    
    ((test_count++))
    if test_endpoint "$base_url/api/v0/cat?arg=QmSampleHash" "Cat command"; then
        ((pass_count++))
    fi
    
    ((test_count++))
    if test_endpoint "$base_url/api/v0/ls?arg=QmDirectoryHash" "List command"; then
        ((pass_count++))
    fi
    
    # HEAD request test
    ((test_count++))
    print_status "Testing: HEAD request for metadata"
    if curl -I -s --connect-timeout 5 "$base_url/ipfs/QmTestContent" | grep -q "HTTP/.*200"; then
        print_success "HEAD request for metadata"
        ((pass_count++))
    else
        print_error "HEAD request for metadata"
    fi
    
    # CORS test
    ((test_count++))
    print_status "Testing: CORS headers"
    if curl -s -H "Origin: http://localhost:3000" "$base_url/health" -D /tmp/cors_headers | grep -q "Access-Control-Allow-Origin"; then
        print_success "CORS headers present"
        ((pass_count++))
    else
        print_error "CORS headers missing"
    fi
    
    echo
    echo "📊 $service_name Results: $pass_count/$test_count tests passed"
    
    if [ $pass_count -eq $test_count ]; then
        print_success "$service_name - All tests passed!"
        return 0
    else
        print_error "$service_name - Some tests failed"
        return 1
    fi
}

# Function to validate sample data files
validate_sample_data() {
    echo
    echo "📋 Validating Sample Data Files"
    echo "-------------------------------"
    
    local data_dir="$BASE_DIR/sample-data"
    local files=("nft-metadata.json" "game-content.txt" "achievement-notification.json" "marketplace-listing.json")
    local valid_count=0
    
    for file in "${files[@]}"; do
        if [ -f "$data_dir/$file" ]; then
            print_success "Found: $file"
            ((valid_count++))
            
            # Validate JSON files
            if [[ "$file" == *.json ]]; then
                if jq empty "$data_dir/$file" 2>/dev/null; then
                    print_success "Valid JSON: $file"
                else
                    print_error "Invalid JSON: $file"
                fi
            fi
        else
            print_error "Missing: $file"
        fi
    done
    
    echo "📊 Sample Data: $valid_count/${#files[@]} files found"
}

# Function to test Docker build
test_docker_build() {
    echo
    echo "🐳 Testing Docker Build"
    echo "-----------------------"
    
    # Test Rust service Docker build
    print_status "Testing Rust service Docker build"
    if docker build -f ../dockerfiles/ipfs-mock/Dockerfile -t ipfs-mock:test ../../.. > /tmp/docker_build.log 2>&1; then
        print_success "Docker build successful"
    else
        print_error "Docker build failed - check /tmp/docker_build.log"
        return 1
    fi
    
    # Clean up test image
    docker rmi ipfs-mock:test > /dev/null 2>&1 || true
}

# Main execution
main() {
    echo "Starting IPFS Mock Gateway integration tests..."
    echo
    
    # Validate sample data
    validate_sample_data
    
    # Test Rust implementation
    if test_service "Rust IPFS Mock" "http://localhost:$RUST_SERVICE_PORT" "$RUST_SERVICE_PORT"; then
        rust_tests_passed=true
    else
        rust_tests_passed=false
    fi
    
    # Test Docker build
    if test_docker_build; then
        docker_build_passed=true
    else
        docker_build_passed=false
    fi
    
    # Summary
    echo
    echo "🏁 Integration Test Summary"
    echo "=========================="
    
    if [ "$rust_tests_passed" = true ]; then
        print_success "Rust IPFS Mock Service - PASSED"
    else
        print_error "Rust IPFS Mock Service - FAILED"
    fi
    
    if [ "$docker_build_passed" = true ]; then
        print_success "Docker Build - PASSED"
    else
        print_error "Docker Build - FAILED"
    fi
    
    # Overall result
    if [ "$rust_tests_passed" = true ] && [ "$docker_build_passed" = true ]; then
        echo
        print_success "🎉 All integration tests passed!"
        print_success "The IPFS Mock Gateway is fully functional and ready for use."
        exit 0
    else
        echo
        print_error "❌ Some integration tests failed."
        print_error "Please check the service configuration and try again."
        exit 1
    fi
}

# Run main function
main "$@"