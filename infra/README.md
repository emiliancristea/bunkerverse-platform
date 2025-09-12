# Bunkerverse Platform - Infrastructure Documentation

## Overview

This document provides comprehensive setup instructions for the Bunkerverse Platform local development environment and cloud infrastructure. The infrastructure implements a "Smart Stubs" approach with dual-mode configuration supporting both MVE (Minimum Viable Experience) and full crypto functionality.

## Architecture

### Local Development Environment

The local development stack includes:
- **8 Smart Stub Services**: Marketplace, Indexer, Identity, AI Data, Account, Feedback, Mission, Payment, Social
- **Blockchain Infrastructure**: Arbitrum Orbit L2 + Ethereum L1 (Anvil)
- **Data Stores**: PostgreSQL, Redis, Elasticsearch
- **External Service Mocks**: IPFS, Arweave gateways
- **Security**: Network isolation, encrypted communication, non-root containers

### Cloud Infrastructure

The production-ready cloud deployment features:
- **Amazon EKS**: Managed Kubernetes with auto-scaling node groups
- **Networking**: Secure VPC with public/private/database subnets
- **Databases**: RDS MySQL, ElastiCache Redis, OpenSearch
- **Container Registry**: Amazon ECR with lifecycle policies
- **Load Balancing**: ALB with SSL termination
- **Game Servers**: Agones for dedicated game server management
- **Security**: IAM roles, Network policies, Pod Security Standards

## Quick Start - Local Development

### Prerequisites

- Docker and Docker Compose v3.8+
- 16GB+ RAM recommended
- 50GB+ free disk space

### Setup Instructions

1. **Clone and Navigate**
   ```bash
   git clone https://github.com/emiliancristea/bunkerverse-platform.git
   cd bunkerverse-platform
   ```

2. **Configure Environment**
   ```bash
   cp .env.example .env
   # Edit .env file with your desired configuration
   ```

3. **Start the Complete Stack**
   ```bash
   # Start all services
   docker-compose up -d
   
   # View logs
   docker-compose logs -f marketplace-service
   ```

4. **Verify Services**
   ```bash
   # Check service health
   curl http://localhost:8081/health  # Marketplace service
   curl http://localhost:8082/health  # Indexer service
   curl http://localhost:8083/health  # Identity service
   ```

### Service Endpoints

| Service | Port | Health Check | Description |
|---------|------|-------------|-------------|
| Marketplace | 8081 | `/health` | NFT marketplace and trading |
| Indexer | 8082 | `/health` | Blockchain data indexing |
| Identity | 8083 | `/health` | Authentication & authorization |
| AI Data | 8084 | `/health` | AI/ML data processing |
| Account | 8085 | `/health` | Player account management |
| Feedback | 8086 | `/health` | User feedback system |
| Mission | 8087 | `/health` | Game missions & quests |
| Payment | 8088 | `/health` | Payment processing |
| Social | 8089 | `/health` | Social features |

### Blockchain Infrastructure

| Component | Port | Purpose |
|-----------|------|---------|
| Ethereum L1 (Anvil) | 8545 | Local Ethereum node |
| Arbitrum L2 Sequencer | 8547 | Local Arbitrum Orbit chain |
| IPFS Mock | 8080 | Decentralized storage mock |
| Arweave Mock | 1984 | Permanent storage mock |

### Data Stores

| Service | Port | Default Credentials |
|---------|------|-------------------|
| PostgreSQL | 5432 | `bunkerverse:dev123@bunkerverse` |
| Redis | 6379 | Password: `dev123` |
| Elasticsearch | 9200 | No authentication (dev only) |

## Dual-Mode Configuration

### MVE Mode (Default: `ENABLE_CRYPTO=false`)

- **Features**: User accounts, social features, marketplace browsing, missions
- **Disabled**: NFT operations, crypto payments, blockchain transactions
- **Use Case**: Initial launch, regulatory-safe markets, content testing

### Full Crypto Mode (`ENABLE_CRYPTO=true`)

- **Features**: Complete NFT marketplace, crypto payments, DeFi integration
- **Blockchain**: Full Arbitrum Orbit L2 with Ethereum L1 settlement
- **Use Case**: Web3-enabled markets, crypto-native features

### Switching Modes

1. **Environment Variables**
   ```bash
   # In .env file
   ENABLE_CRYPTO=true
   SHOW_CRYPTO=true
   ```

2. **Runtime Configuration**
   ```bash
   # Restart services to apply changes
   docker-compose restart marketplace-service indexer-service
   ```

3. **API Testing**
   ```bash
   # Test crypto-enabled endpoints
   curl http://localhost:8081/api/marketplace/nfts/test-nft-001
   ```

## Smart Stubs Features

### Intelligent Response Generation

- **Schema-Valid Responses**: All responses match production API schemas
- **Contextual Data**: Responses change based on dual-mode configuration
- **Relationship Consistency**: Maintains state across related API calls

### Configurable Behavior

- **Latency Simulation**: Configurable response delays with multiple distributions
- **Error Injection**: Controllable error rates and specific error scenarios
- **Network Conditions**: Simulate good/poor/variable network conditions

### Structured Logging

All services provide comprehensive JSON logging:

```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "level": "INFO",
  "stub_name": "marketplace-service-stub",
  "stub_version": "1.0.0",
  "event_type": "request_received",
  "endpoint": "/api/marketplace/listings",
  "method": "GET",
  "request_id": "req-uuid-here",
  "enable_crypto": false,
  "simulated_latency_ms": 150,
  "response_status": 200,
  "error_injected": false
}
```

## Cloud Deployment

### Prerequisites

- AWS CLI configured with appropriate permissions
- Terraform 1.0+
- kubectl
- Helm 3.0+

### Infrastructure Deployment

1. **Configure Terraform Backend**
   ```bash
   # Edit infra/terraform/main.tf backend configuration
   # Create S3 bucket and DynamoDB table for state management
   ```

2. **Set Variables**
   ```bash
   cd infra/terraform
   cp terraform.tfvars.example terraform.tfvars
   # Edit terraform.tfvars with your configuration
   ```

3. **Deploy Infrastructure**
   ```bash
   terraform init
   terraform plan
   terraform apply
   ```

4. **Configure kubectl**
   ```bash
   aws eks update-kubeconfig --region us-east-1 --name bunkerverse-dev-cluster
   ```

### Security Configuration

#### Network Security

- **VPC**: Isolated network with public/private/database subnets
- **Security Groups**: Restrictive ingress/egress rules
- **Network Policies**: Kubernetes-native network segmentation
- **VPC Flow Logs**: Complete network traffic monitoring

#### Application Security

- **Pod Security Standards**: Restricted profile enforcement
- **Non-root Containers**: All containers run as UID 65532
- **Read-only Filesystems**: Immutable container filesystems
- **Resource Limits**: CPU, memory, and storage quotas
- **Service Accounts**: Minimal RBAC permissions

#### Data Security

- **Encryption at Rest**: All databases and storage encrypted
- **Encryption in Transit**: TLS 1.2+ for all communications
- **Secrets Management**: AWS Secrets Manager integration
- **Backup Encryption**: Automated encrypted backups

## Monitoring and Observability

### Health Checks

All services implement comprehensive health checks:

```bash
# Service health
curl http://localhost:8081/health

# Detailed service info
curl http://localhost:8081/stub/config

# Reset stub state
curl -X POST http://localhost:8081/stub/reset
```

### Logging

- **Structured JSON**: Machine-readable log format
- **Request Tracing**: Unique request IDs for correlation
- **Performance Metrics**: Response times and error rates
- **Audit Trail**: Complete API interaction history

### Metrics Collection

- **Prometheus**: Metrics collection and alerting
- **Grafana**: Visualization and dashboards
- **Jaeger**: Distributed tracing
- **CloudWatch**: AWS-native monitoring integration

## Development Workflow

### Service Development

1. **Local Testing**
   ```bash
   cd services/marketplace
   RUST_LOG=debug cargo run
   ```

2. **Container Testing**
   ```bash
   docker build -f infra/dockerfiles/marketplace/Dockerfile -t marketplace:test .
   docker run -p 8081:8081 marketplace:test
   ```

3. **Integration Testing**
   ```bash
   docker-compose up marketplace-service postgres redis
   ```

### Configuration Management

1. **Environment Variables**: Use `.env` file for local development
2. **ConfigMaps**: Kubernetes configuration for cloud deployment
3. **Secrets**: AWS Secrets Manager for production credentials

### Database Migrations

```bash
# Local development
docker-compose exec postgres psql -U bunkerverse -d bunkerverse

# Cloud deployment
kubectl exec -it postgres-pod -- psql -U admin -d bunkerverse
```

## Troubleshooting

### Common Issues

1. **Port Conflicts**
   ```bash
   # Check port usage
   netstat -tulpn | grep :8081
   
   # Stop conflicting services
   docker-compose down
   ```

2. **Database Connection Issues**
   ```bash
   # Check database logs
   docker-compose logs postgres
   
   # Reset database
   docker-compose down -v
   docker-compose up -d postgres
   ```

3. **Memory Issues**
   ```bash
   # Check resource usage
   docker stats
   
   # Limit service resources
   # Edit docker-compose.yml to add memory limits
   ```

### Service Debugging

1. **Container Logs**
   ```bash
   docker-compose logs -f marketplace-service
   ```

2. **Interactive Shell**
   ```bash
   docker-compose exec marketplace-service sh
   ```

3. **Health Check Details**
   ```bash
   curl -v http://localhost:8081/health
   ```

### Performance Tuning

1. **Latency Configuration**
   ```bash
   # Adjust in service configuration
   LATENCY_MIN_MS=50
   LATENCY_MAX_MS=200
   ```

2. **Error Rate Adjustment**
   ```bash
   # Configure error injection
   ERROR_RATE=0.05  # 5% error rate
   ```

3. **Resource Scaling**
   ```bash
   docker-compose up --scale marketplace-service=3
   ```

## Security Considerations

### Development Environment

- **Network Isolation**: Services communicate via Docker network
- **Default Credentials**: Change all default passwords in production
- **Data Persistence**: Use volumes for important data
- **Regular Updates**: Keep container images updated

### Production Environment

- **IAM Roles**: Use AWS IAM roles instead of access keys
- **Secrets Rotation**: Implement regular credential rotation
- **Network Monitoring**: Enable VPC Flow Logs and monitoring
- **Compliance**: Follow SOC2, PCI DSS requirements as needed

## Cost Optimization

### Development

- **Resource Limits**: Set appropriate CPU/memory limits
- **Service Scaling**: Scale down unused services
- **Data Cleanup**: Regular cleanup of development data

### Production

- **Spot Instances**: Use spot instances for non-critical workloads
- **Auto Scaling**: Implement horizontal pod autoscaling
- **Resource Optimization**: Right-size instances based on usage
- **Reserved Instances**: Use reserved instances for predictable workloads

## Support and Contributing

### Getting Help

1. **Documentation**: Check this README and inline code comments
2. **Health Endpoints**: Use service health checks for diagnostics
3. **Logs**: Check structured logs for detailed error information

### Development Guidelines

1. **Follow Security Standards**: Implement all security best practices
2. **Maintain Compatibility**: Ensure dual-mode functionality works
3. **Add Tests**: Include unit and integration tests
4. **Update Documentation**: Keep documentation current

---

**Generated with Smart Stubs v1.0.0**  
**Platform: Bunkerverse**  
**Environment: Development**  
**Security Level: Baseline**