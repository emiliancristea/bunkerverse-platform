# IPFS Mock Gateway Implementation

This directory contains a complete IPFS Gateway Mock Server implementation for the Bunkerverse Platform, providing realistic IPFS gateway behavior for development and testing purposes.

## 🏗️ Architecture Overview

### Primary Implementation: Rust-based HTTP Server
- **Location**: `/services/ipfs-mock/`
- **Technology**: Rust with Axum web framework
- **Features**: Complete IPFS gateway simulation with smart content generation
- **Container**: Security-hardened distroless container

### Alternative Implementation: Nginx-based Static Server
- **Location**: `/infra/mocks/ipfs/`
- **Technology**: Nginx with static file serving
- **Use Case**: Simple static content serving for basic testing

## 📂 Directory Structure

```
infra/mocks/ipfs/
├── sample-data/                 # Sample mock content files
│   ├── nft-metadata.json       # NFT metadata examples
│   ├── game-content.txt        # Game text content
│   ├── achievement-notification.json
│   └── marketplace-listing.json
├── html/                       # Web interface files
│   └── index.html             # Gateway status page
├── nginx.conf                  # Nginx configuration
├── docker-compose.nginx.yml    # Alternative nginx deployment
└── README.md                   # This documentation
```

## 🚀 Features

### IPFS Gateway Endpoints
- **Content Retrieval**: `GET /ipfs/{cid}` - Retrieve content by CID
- **Metadata Access**: `HEAD /ipfs/{cid}` - Get content metadata without body
- **CORS Support**: Full cross-origin resource sharing
- **Caching Headers**: Proper ETags and cache-control headers

### IPFS API v0 Endpoints  
- `GET|POST /api/v0/cat` - Display content of IPFS objects
- `GET|POST /api/v0/ls` - List links from IPFS objects
- `POST /api/v0/add` - Add content to IPFS (mock response)
- `GET|POST /api/v0/id` - Show IPFS node identity
- `GET|POST /api/v0/version` - Show IPFS version information

### Smart Content Generation
- **NFT Metadata**: Gaming-themed JSON metadata for weapons, armor, utilities
- **Text Content**: Mission briefings, achievement notifications
- **Image Generation**: Deterministic PNG creation with CID-based colors
- **Binary Data**: Consistent binary content generation

### Health Monitoring
- **Health Endpoint**: `GET /health` - Service health status
- **Structured Logging**: JSON-formatted logs
- **Docker Health Checks**: Built-in container monitoring

## 🎮 Sample Data

### NFT Metadata (`nft-metadata.json`)
Gaming-focused NFT metadata with attributes like:
- Weapon types and rarities
- Attack power and durability stats
- Special abilities and set bonuses
- Creator and blockchain information

### Game Content (`game-content.txt`)
Mission briefings and game narrative content:
- Operation objectives and intel
- Equipment loadouts and stats
- Risk assessments and tactical information

### Achievement System (`achievement-notification.json`)
Player progression and reward data:
- Achievement unlocks and points
- Reward distributions (XP, tokens, items)
- Progress tracking and milestones

### Marketplace Listings (`marketplace-listing.json`)
NFT trading and marketplace data:
- Listing details and pricing
- Seller information and reputation
- Blockchain and contract verification

## 🛠️ Deployment Options

### Option 1: Rust-based Server (Recommended)
The primary implementation runs automatically via docker-compose:

```bash
# Start the full platform (includes IPFS mock)
docker-compose up ipfs-mock

# Test the service
curl http://localhost:8080/health
curl http://localhost:8080/ipfs/QmNFTMetadata123
```

### Option 2: Nginx-based Server (Alternative)
For simple static file serving:

```bash
# Start nginx-based alternative
cd infra/mocks/ipfs
docker-compose -f docker-compose.nginx.yml up

# Test the service
curl http://localhost:8080/health
curl http://localhost:8080/ipfs/QmSampleContent
```

## 🧪 Testing the Implementation

### Manual Testing
```bash
# Health check
curl http://localhost:8080/health

# NFT metadata
curl http://localhost:8080/ipfs/QmNFTMetadata123

# Text content
curl http://localhost:8080/ipfs/QmTextContent456

# API endpoints
curl http://localhost:8080/api/v0/version
curl http://localhost:8080/api/v0/cat?arg=QmSomeHash
```

### Automated Testing
Run the comprehensive test script:

```bash
cd services/ipfs-mock
./test_endpoints.sh
```

## 📊 Content Type Detection

The mock gateway automatically determines content type based on CID patterns:

- **JSON/Metadata**: CIDs containing "json", "meta" or ending with 1,3
- **Images**: CIDs containing "image", "png", "jpg" or ending with 2,6  
- **Text**: CIDs containing "txt", "text" or ending with 4,8
- **Default**: JSON NFT metadata for gaming assets

## 🔧 Configuration

### Environment Variables
- `RUST_LOG`: Logging level (default: info)
- `IPFS_GATEWAY_PORT`: Service port (default: 8080)
- `IPFS_SERVICE_NAME`: Service name for logging

### Docker Compose Integration
The IPFS mock is automatically included in the platform's main docker-compose.yml:

```yaml
ipfs-mock:
  build:
    context: .
    dockerfile: infra/dockerfiles/ipfs-mock/Dockerfile
  container_name: ipfs-mock
  ports:
    - "8080:8080"
  environment:
    - RUST_LOG=info
    - IPFS_GATEWAY_PORT=8080
  healthcheck:
    test: ["CMD", "/usr/local/bin/ipfs-mock", "--health-check"]
    interval: 30s
    timeout: 10s
    retries: 3
```

## 🔒 Security Features

- **Non-root Execution**: Runs as UID 65532 in distroless container
- **Minimal Attack Surface**: Distroless base image with no shell
- **Input Validation**: CID format validation and sanitization
- **Security Headers**: Proper CORS and caching headers
- **Container Scanning**: Security compliance labels

## 🎯 Benefits for Development

### Development Speed
- **No IPFS Infrastructure**: Eliminates complex IPFS node setup
- **Instant Responses**: No network delays or pinning issues
- **Predictable Content**: Deterministic responses for testing
- **Easy Debugging**: Structured logs and health monitoring

### Testing Advantages
- **Isolated Testing**: No external dependencies
- **Reproducible Results**: Consistent content generation
- **Error Simulation**: Built-in error injection capabilities
- **Platform Integration**: Seamless integration with Bunkerverse services

## 🔮 Extension Points

The mock gateway can be extended with:
- Custom content generators
- Pluggable storage backends
- Advanced routing patterns
- Additional protocol support
- Redis backing store for persistence
- Prometheus metrics endpoint

## 📝 Usage Examples

### Standard IPFS Client Libraries
The mock gateway is compatible with standard IPFS client libraries:

```javascript
// JavaScript example
const ipfs = new IPFS({ host: 'localhost', port: 8080, protocol: 'http' });
const content = await ipfs.cat('QmSomeHash');
```

### Browser Fetch API
```javascript
// Browser example with CORS support
fetch('http://localhost:8080/ipfs/QmNFTMetadata123')
  .then(response => response.json())
  .then(metadata => console.log(metadata.name));
```

### cURL Commands
```bash
# Get NFT metadata
curl -H "Accept: application/json" http://localhost:8080/ipfs/QmNFTMetadata

# Get with headers
curl -I http://localhost:8080/ipfs/QmImageContent

# API calls
curl -X POST http://localhost:8080/api/v0/add -d '{"content":"test"}'
```

## 🏁 Conclusion

This IPFS Mock Gateway provides a complete, production-ready solution for simulating IPFS behavior in development and testing environments. It offers:

1. **Full Protocol Compatibility**: Standard IPFS gateway and API support
2. **Gaming-Optimized Content**: Rich NFT metadata and gaming-specific content  
3. **Enterprise Security**: Hardened containers and security best practices
4. **Development Optimized**: Fast, predictable, and easy to debug
5. **Platform Integrated**: Seamless integration with Bunkerverse infrastructure

The implementation supports the entire development lifecycle from initial prototyping through production deployment, providing a robust foundation for IPFS-dependent features in the Bunkerverse platform.