# ADR-0006: Marketplace Service Architecture

**Status:** Draft  
**Date:** 2024-09-08  
**Deciders:** Lead Architect, Backend Lead  

## Context

The Bunkerverse Platform requires a high-performance marketplace service to handle user-to-user trading, item listings, search functionality, and transaction processing. The service must support both off-chain MVE operations and future blockchain-based trading.

## Decision

We will use **Rust/Axum with WebSocket Integration**:
- **Axum Framework**: Modern async web framework for HTTP APIs
- **WebSocket Support**: Real-time updates for marketplace events
- **gRPC Integration**: Inter-service communication with other platform components
- **Database Integration**: PostgreSQL for persistent storage, Redis for caching

## Rationale

### Performance Requirements
- **High Throughput**: 8,000+ requests/second validated in PoC
- **Low Latency**: Sub-100ms response times for search and listing operations
- **Real-time Updates**: Instant notifications for price changes, new listings
- **Concurrent Users**: Support for thousands of simultaneous marketplace browsers

### Technical Benefits
- **Async Performance**: Tokio-based async runtime for maximum concurrency
- **Type Safety**: Rust's type system prevents common API bugs
- **Memory Safety**: No memory leaks or corruption in high-load scenarios
- **Modern Architecture**: Built for cloud-native deployment patterns

### Integration Advantages
- **WebSocket Support**: Real-time bidirectional communication
- **gRPC Ready**: Efficient inter-service communication
- **Database Agnostic**: Flexible database backend selection
- **Monitoring**: Built-in metrics and observability features

## Consequences

### Positive
- Excellent performance characteristics meet all scalability requirements
- Real-time features provide superior user experience
- Memory-safe implementation reduces operational risks
- Modern architecture supports future scaling and features

### Negative
- Rust learning curve for developers unfamiliar with the language
- More complex async debugging compared to synchronous alternatives
- Additional infrastructure complexity with WebSocket connections
- Database connection pool management requires careful tuning

## Security Considerations & Threat Model Outline

### Threat Categories (STRIDE Analysis)

**Spoofing:**
- JWT authentication prevents user impersonation
- API key validation for service-to-service communication
- Rate limiting per user and IP address

**Tampering:**
- Input validation prevents data tampering
- Database transactions ensure data consistency
- Request signing for sensitive operations

**Repudiation:**
- Comprehensive audit logging of all marketplace transactions
- User action tracking with timestamps and IP addresses
- Digital signatures for high-value transactions

**Information Disclosure:**
- Authorization checks prevent unauthorized data access
- Field-level permissions for sensitive user information
- TLS encryption for all API communications

**Denial of Service:**
- Rate limiting and request throttling
- Resource quotas per user and operation type
- Circuit breakers for downstream service protection

**Elevation of Privilege:**
- Role-based access control (RBAC) for administrative functions
- Principle of least privilege for service accounts
- Regular security audits and access reviews

### Security Controls
- **Input Validation**: Comprehensive sanitization of all user inputs
- **Authentication**: JWT-based authentication with refresh tokens
- **Authorization**: Fine-grained permissions for marketplace operations
- **Audit Logging**: Complete audit trail for compliance and security monitoring
- **Rate Limiting**: Protection against abuse and DoS attacks

## Dual-Mode (Off-Chain/On-Chain) Design Considerations

### MVE Mode (Off-Chain First)
- **Full Marketplace**: Complete trading functionality using traditional payment methods
- **Fiat Integration**: Credit card and bank transfer payments
- **Centralized Escrow**: Platform-managed transaction security
- **Traditional KYC**: Standard identity verification processes

### Full Blockchain Mode
- **Crypto Payments**: Native support for NTC and other cryptocurrency payments
- **Smart Contract Escrow**: Blockchain-based trustless transactions
- **NFT Trading**: Native support for NFT marketplace operations
- **Decentralized Identity**: Integration with blockchain-based identity systems

### Configuration Management
```rust
pub struct MarketplaceConfig {
    pub enable_crypto_payments: bool,
    pub enable_nft_trading: bool,
    pub enable_smart_contract_escrow: bool,
    pub supported_payment_methods: Vec<PaymentMethod>,
}

pub enum PaymentMethod {
    CreditCard,
    BankTransfer,
    Cryptocurrency(String), // Token symbol
    NFT(String), // Contract address
}
```

## Architecture Details

### Component Architecture
```
┌─────────────────┐
│   HTTP/WS APIs  │ ← External Interface
├─────────────────┤
│  Axum Framework │ ← Web Framework
├─────────────────┤
│ Business Logic  │ ← Marketplace Logic
├─────────────────┤
│  gRPC Services  │ ← Inter-Service Communication
├─────────────────┤
│ Database Layer  │ ← PostgreSQL + Redis
└─────────────────┘
```

### Service Boundaries
- **Listing Service**: Item creation, updates, search
- **Transaction Service**: Purchase processing, escrow management
- **Notification Service**: Real-time updates via WebSocket
- **Payment Service**: Payment processing and reconciliation
- **Analytics Service**: Marketplace metrics and insights

## Implementation Plan

### Phase 0 (Current)
- [x] PoC with basic HTTP endpoints and WebSocket support
- [x] Performance validation (8,000+ req/sec)
- [x] Authentication and authorization framework
- [x] Real-time notification system

### Phase 1 (MVE)
- [ ] Complete marketplace feature implementation
- [ ] Payment processor integration (Stripe/PayPal)
- [ ] Advanced search and filtering capabilities
- [ ] Comprehensive testing and security validation

### Phase 2 (On-Chain Integration)
- [ ] Cryptocurrency payment integration
- [ ] Smart contract escrow implementation  
- [ ] NFT marketplace features
- [ ] Blockchain analytics and reporting

## API Design

### RESTful HTTP APIs
```rust
// Listing endpoints
GET    /api/v1/listings              // Search listings
GET    /api/v1/listings/{id}         // Get specific listing
POST   /api/v1/listings              // Create new listing
PUT    /api/v1/listings/{id}         // Update listing
DELETE /api/v1/listings/{id}         // Delete listing

// Transaction endpoints  
POST   /api/v1/transactions          // Initiate purchase
GET    /api/v1/transactions/{id}     // Get transaction status
POST   /api/v1/transactions/{id}/cancel // Cancel transaction

// User endpoints
GET    /api/v1/users/{id}/listings   // User's listings
GET    /api/v1/users/{id}/purchases  // User's purchase history
```

### WebSocket Events
```rust
pub enum MarketplaceEvent {
    NewListing(ListingCreated),
    PriceUpdate(PriceChanged), 
    TransactionComplete(TransactionFinalized),
    UserNotification(UserAlert),
}
```

### gRPC Services
```rust
service MarketplaceService {
    rpc CreateListing(CreateListingRequest) returns (ListingResponse);
    rpc SearchListings(SearchRequest) returns (SearchResponse);
    rpc ProcessTransaction(TransactionRequest) returns (TransactionResponse);
    rpc GetUserActivity(UserActivityRequest) returns (UserActivityResponse);
}
```

## Data Model

### Core Entities
```rust
pub struct Listing {
    pub id: Uuid,
    pub seller_id: UserId,
    pub title: String,
    pub description: String,
    pub price: Decimal,
    pub currency: Currency,
    pub category: Category,
    pub images: Vec<String>,
    pub status: ListingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Transaction {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub buyer_id: UserId,
    pub seller_id: UserId,
    pub amount: Decimal,
    pub currency: Currency,
    pub status: TransactionStatus,
    pub escrow_details: Option<EscrowInfo>,
    pub created_at: DateTime<Utc>,
}
```

## Performance Benchmarks (PoC Results)
- **HTTP Throughput**: 8,000+ requests/second (exceeds requirements)
- **WebSocket Connections**: 1,000+ concurrent connections supported
- **Response Latency**: <50ms average for listing searches
- **Database Performance**: <10ms query response times
- **Memory Usage**: <100MB base, scales linearly with connections

## Alternatives Considered

### Node.js/Express
- **Pros**: Rapid development, large ecosystem, good WebSocket support
- **Cons**: Single-threaded limitations, runtime errors, memory leaks

### Java/Spring Boot
- **Pros**: Enterprise features, mature ecosystem, good performance
- **Cons**: Higher memory usage, slower startup, complex configuration

### Go/Gin or Fiber
- **Pros**: Good performance, simple deployment, fast compilation
- **Cons**: Less type safety, simpler ecosystem, manual memory management

### Python/FastAPI
- **Pros**: Rapid development, excellent documentation generation
- **Cons**: Performance limitations, GIL constraints, deployment complexity

### .NET Core
- **Pros**: Excellent performance, mature tooling, strong typing
- **Cons**: Windows-centric ecosystem, higher licensing costs

## Database Integration

### PostgreSQL (Primary)
- **ACID Transactions**: Ensures data consistency for financial operations
- **JSON Support**: Flexible schema for marketplace metadata
- **Full-Text Search**: Built-in search capabilities
- **Scalability**: Proven performance at scale

### Redis (Cache Layer)
- **Session Storage**: Fast session and authentication caching
- **Real-time Data**: WebSocket connection state management
- **Rate Limiting**: Distributed rate limiting counters
- **Analytics**: Real-time marketplace metrics

### Migration Strategy
```rust
// Database migrations with sqlx
pub async fn migrate_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await
}
```

## Monitoring and Observability

### Metrics Collection
- **Request Metrics**: Response times, error rates, throughput
- **Business Metrics**: Listings created, transactions completed, revenue
- **System Metrics**: Memory usage, CPU utilization, database connections
- **WebSocket Metrics**: Active connections, message throughput

### Logging Strategy
```rust
// Structured logging with tracing
#[instrument]
pub async fn create_listing(listing: CreateListingRequest) -> Result<Listing> {
    info!("Creating new listing for user {}", listing.seller_id);
    // Implementation
}
```

## References
- [Axum Documentation](https://docs.rs/axum/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [Performance Benchmarks](../performance/marketplace-benchmarks.md)
- [Security Assessment](../security/marketplace-security-review.md)

---

**Review Required By:** Security Lead, Backend Lead, Performance Lead  
**Implementation Target:** Phase 0.2 (PoC Complete), Phase 1.0 (Production)