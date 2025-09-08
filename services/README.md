# Platform Services

Rust microservices that power the BUNKERVERSE platform.

## Services Overview

- `identity/` - zkLogin identity service with ZKP verification
- `account/` - Player account management and profiles  
- `marketplace/` - Digital asset marketplace with Axum backend
- `payment/` - Stripe integration for payment processing
- `mission/` - Mission and quest management system
- `social/` - Social features and player interactions
- `ai_data/` - AI data service interfacing with NAR
- `feedback/` - User feedback collection (MVP stub)

## Development

Each service is a standalone Rust binary with:
- Axum web framework for HTTP APIs
- gRPC for inter-service communication  
- Tokio async runtime
- Structured logging with tracing
- Security-first design patterns

Build all services: `cargo build --workspace`