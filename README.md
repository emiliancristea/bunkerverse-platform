# BUNKERVERSE Platform

A next-generation gaming and digital asset platform built on Arbitrum Orbit L3 blockchain technology.

## Architecture Overview

This monorepo contains the complete BUNKERVERSE platform implementation:

- **Blockchain Layer**: Arbitrum Orbit L3 with custom smart contracts
- **Backend Services**: Rust microservices with secure APIs
- **Client Application**: Qt/QML Control Center with CXX-Qt integration
- **Game Servers**: Unreal Engine 5 multiplayer infrastructure
- **AI Integration**: NAR (llama.cpp) with secure FFI wrapper

## Directory Structure

- `contracts/` - L3 smart contracts (Rust/WASM via Arbitrum Stylus)
- `services/` - Platform microservices (identity, account, marketplace, etc.)
- `indexer/` - Global decentralized indexing layer
- `libs/` - Shared libraries and NAR wrapper
- `client/` - Control Center client (Rust + QML + C++)
- `games/` - UE5 game servers and Netchain plugin
- `tools/` - Development utilities
- `infra/` - Infrastructure as Code
- `docs/` - Documentation and ADRs

## Quick Start

1. Follow setup instructions in `docs/DEVELOPMENT_ENVIRONMENT.md`
2. Build all components: `cargo build --workspace`
3. Run local development environment with Docker Compose

## Security First

This project implements security-first development practices:
- Mandatory pre-commit hooks with security scanning
- STRIDE threat modeling for all components  
- Regular dependency audits with `cargo audit`
- Secure FFI patterns for C++ and NAR integration

## Contributing

See `docs/PROJECT_GOVERNANCE_AND_WORKFLOWS.md` for development guidelines and contribution process.