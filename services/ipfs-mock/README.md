# IPFS Mock Gateway

A comprehensive mock implementation of the IPFS (InterPlanetary File System) gateway for development and testing purposes. This service simulates the behavior of a real IPFS gateway, providing realistic responses for IPFS content requests and API calls.

## Features

### 🌐 IPFS Gateway Endpoints
- **Content Retrieval**: `GET /ipfs/{cid}` - Retrieve content by CID
- **Metadata Headers**: `HEAD /ipfs/{cid}` - Get content metadata without body
- **Proper CORS**: Cross-origin resource sharing enabled for browser access
- **Content Types**: Automatic content type detection based on CID patterns

### 📡 IPFS API v0 Simulation
- `GET|POST /api/v0/cat` - Display content of IPFS objects
- `GET|POST /api/v0/ls` - List links from IPFS objects
- `POST /api/v0/add` - Add content to IPFS (mock response)
- `GET|POST /api/v0/id` - Show IPFS node identity
- `GET|POST /api/v0/version` - Show IPFS version information

### 🎮 Smart Content Generation
- **NFT Metadata**: JSON metadata for gaming NFTs (weapons, armor, utilities)
- **Images**: Mock PNG generation with CID-based colors
- **Text Content**: Gaming-themed text content and mission briefings
- **Binary Data**: Deterministic binary content generation

### 🏥 Health & Monitoring
- **Health Endpoint**: `GET /health` - Service health status
- **Structured Logging**: JSON-formatted logs compatible with ELK stack
- **Docker Health Checks**: Built-in container health monitoring

## Content Type Detection

The mock gateway automatically determines content type based on CID patterns:

- **JSON/Metadata**: CIDs containing "json", "meta" or ending with 1,3
- **Images**: CIDs containing "image", "png", "jpg" or ending with 2,6  
- **Text**: CIDs containing "txt", "text" or ending with 4,8
- **Default**: JSON NFT metadata for gaming assets

## Mock Data Examples

### NFT Metadata Response
```json
{
  "name": "Epic Battle Axe",
  "description": "A legendary weapon forged in the depths of Mount Bunker...",
  "image": "ipfs://QmExampleImageHash123456789",
  "attributes": [
    {
      "trait_type": "Weapon Type",
      "value": "Axe"
    },
    {
      "trait_type": "Rarity", 
      "value": "Epic"
    }
  ],
  "tokenId": 1,
  "generatedAt": "2024-01-01T12:00:00Z"
}
```

### IPFS API Response
```bash
curl http://localhost:8080/api/v0/version
{
  "Version": "0.14.0-mock",
  "Commit": "mock-commit-hash",
  "Service": "IPFS Mock Gateway"
}
```

## Configuration

Environment variables:
- `RUST_LOG`: Logging level (default: info)
- `IPFS_GATEWAY_PORT`: Service port (default: 8080)
- `IPFS_SERVICE_NAME`: Service name for logging

## Development Usage

### Running Standalone
```bash
cd services/ipfs-mock
cargo run
```

### Docker Compose
The service is automatically included in the platform's docker-compose.yml:
```bash
docker-compose up ipfs-mock
```

### Testing Endpoints
```bash
# Get NFT metadata
curl http://localhost:8080/ipfs/QmNftMetadata123

# Get image content  
curl http://localhost:8080/ipfs/QmImageContent456

# API calls
curl http://localhost:8080/api/v0/cat?arg=QmSomeHash
curl -X POST http://localhost:8080/api/v0/add
```

## Architecture

- **Language**: Rust with Tokio async runtime
- **Web Framework**: Axum for high-performance HTTP server
- **Content Generation**: Deterministic content based on CID hashing
- **Security**: Runs as non-root user in distroless container
- **Observability**: Structured JSON logging with tracing

## Integration with Bunkerverse Platform

This IPFS mock integrates seamlessly with the Bunkerverse gaming platform:

- **NFT Storage**: Simulates IPFS storage for game asset metadata
- **Content Delivery**: Provides consistent responses for testing
- **API Compatibility**: Compatible with standard IPFS client libraries
- **Development Speed**: No need for real IPFS infrastructure during development

## Security Features

- Non-root container execution (UID 65532)
- Distroless base image for minimal attack surface
- Security scanning labels for compliance
- Read-only root filesystem
- Proper CORS configuration
- Input validation and sanitization