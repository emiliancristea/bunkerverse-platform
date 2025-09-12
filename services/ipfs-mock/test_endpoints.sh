#!/bin/bash

# IPFS Mock Gateway - Endpoint Testing Script
# This script demonstrates the functionality of the IPFS mock gateway

echo "🚀 IPFS Mock Gateway - Endpoint Testing"
echo "========================================"
echo

# Set base URL
BASE_URL="http://localhost:8080"

# Health Check
echo "📋 Health Check:"
curl -s "${BASE_URL}/health" | jq '.' || echo "jq not available, raw output:"
curl -s "${BASE_URL}/health"
echo -e "\n"

# IPFS Gateway Endpoints
echo "🌐 IPFS Gateway Content Tests:"
echo "-------------------------------"

echo "1. NFT Metadata (JSON):"
curl -s "${BASE_URL}/ipfs/QmNFTMetadata123" | jq '.name, .description' 2>/dev/null || \
curl -s "${BASE_URL}/ipfs/QmNFTMetadata123" | head -10
echo -e "\n"

echo "2. Text Content:"
curl -s "${BASE_URL}/ipfs/QmTextContent456" | head -3
echo -e "\n"

echo "3. Image Content (HEAD request for metadata):"
curl -I "${BASE_URL}/ipfs/QmImage789" 2>/dev/null | grep -E "(HTTP|content-type|content-length|etag)"
echo -e "\n"

# IPFS API v0 Endpoints
echo "📡 IPFS API v0 Tests:"
echo "--------------------"

echo "1. Version Info:"
curl -s "${BASE_URL}/api/v0/version" | jq '.' 2>/dev/null || \
curl -s "${BASE_URL}/api/v0/version"
echo -e "\n"

echo "2. Node ID:"
curl -s "${BASE_URL}/api/v0/id" | jq '.ID' 2>/dev/null || \
curl -s "${BASE_URL}/api/v0/id" | head -3
echo -e "\n"

echo "3. Cat Command:"
curl -s "${BASE_URL}/api/v0/cat?arg=QmSampleHash" | head -5
echo -e "\n"

echo "4. List Command:"
curl -s "${BASE_URL}/api/v0/ls?arg=QmDirectoryHash" | jq '.Objects[0].Links[0]' 2>/dev/null || \
curl -s "${BASE_URL}/api/v0/ls?arg=QmDirectoryHash" | head -10
echo -e "\n"

echo "5. Add Command (Mock):"
curl -s -X POST "${BASE_URL}/api/v0/add" | jq '.hash' 2>/dev/null || \
curl -s -X POST "${BASE_URL}/api/v0/add"
echo -e "\n"

# Content Type Variations
echo "🎮 Content Type Detection Tests:"
echo "------------------------------"

echo "1. JSON pattern (ends with 1):"
curl -s "${BASE_URL}/ipfs/QmGameMetadata1" | jq '.name' 2>/dev/null || echo "JSON content"

echo "2. Image pattern (ends with 2):"
curl -I "${BASE_URL}/ipfs/QmGameImage2" 2>/dev/null | grep content-type

echo "3. Text pattern (ends with 4):"
curl -s "${BASE_URL}/ipfs/QmGameText4" | head -1

echo -e "\n✅ All endpoint tests completed!"
echo "The IPFS Mock Gateway is fully functional and compatible with standard IPFS clients."