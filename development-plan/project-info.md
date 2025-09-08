BUNKERVERSE Development Plan (V8.0 - Final MVE)

Version: 8.0 (Definitive MVE Launch Plan - Full Lore & Mechanics Foundation)
Date: 29/07/2025

I. Overarching Project Governance & First Principles (Applied Throughout All Phases)

This document outlines the definitive development plan for the BUNKERVERSE Minimum Viable Ecosystem (MVE). All development, architectural decisions, and operational procedures will strictly adhere to the following 18 First Principles.

A. Minimize and Justify Dependencies:

Principle: Every external dependency introduces potential risks (complexity, security, maintenance, licensing).

Mandate: For every proposed library/framework (Rust crates, C++ libraries, OS-level tools):

Necessity Justification: Document in the relevant ADR why the dependency is essential and cannot be reasonably achieved with existing project code or standard language features.

Due Diligence: Document its maintenance status, community support, security vulnerability history (via CVE databases and tools like cargo audit), API stability, and licensing (must be compatible with project goals, e.g., MIT, Apache-2.0).

Transitive Dependency Analysis: Identify and vet all significant transitive dependencies introduced.

Centralized Record: Maintain a DEPENDENCIES.md file at the root, listing all chosen external dependencies, their exact pinned versions, justification, license, and a link to their security assessment.

B. Unified Version Control for Key Shared Definitions:

Principle: Data structures and API contracts shared across language/service boundaries (Protocol Buffers for gRPC) require a single source of truth and consistent versioning.

Mandate:

All .proto files will reside in the /protos directory, versioned in Git.

Generated code (Rust via tonic-build/prost-build, C++ via protoc) will not be committed to Git and will be generated as part of each component's build process.

A documented process for updating .proto definitions, regenerating code for all affected components, and coordinating these updates will be followed. This will involve versioning schemas in their file paths (e.g., v1/my_service.proto, v2/my_service.proto).

C. Explicit Version Pinning & Management:

Principle: "Latest" is not a version. All dependencies must be pinned for reproducible and stable builds.

Mandate:

Rust (Cargo): Cargo.toml will specify exact versions (e.g., tokio = "1.35"). Cargo.lock will be committed to Git. cargo update will be a deliberate, tested action.

C++ (CMake): For dependencies like Qt, llama.cpp, and GoogleTest, FetchContent will be used, pinning to specific Git commit SHAs or release tags.

Git Submodules: llama.cpp will be included as a Git submodule pinned to a specific commit SHA.

D. Cross-Language Build & CI Integration:

Principle: The build system and CI must reliably build all components (Rust, C++, QML) in the correct order, using consistent toolchain versions.

Mandate:

CI jobs (GitHub Actions) will explicitly install and use pinned versions of Rust toolchains (defined in rust-toolchain.toml), C++ compilers (GCC, Clang, MSVC versions specified in docs/DEVELOPMENT_ENVIRONMENT.md), Qt SDK (for CXX-Qt/QML builds), CMake, and the Protocol Buffer Compiler (protoc).

Docker images for CI runners will pre-install these toolchains where possible.

The monorepo build, orchestrated by CI workflow logic, will respect inter-component dependencies (e.g., build /protos codegen first, then shared Rust libs, then Rust services, then the Rust Client Application Logic and Rust NAR library, then the C++ Client Shell that links them).

E. Compatibility Matrix for Critical Libraries:

Principle: Version compatibility between libraries used across language boundaries must be ensured.

Mandate: Maintain docs/COMPATIBILITY_MATRIX.md specifying known compatible versions of:

protoc compiler vs. Rust tonic/prost vs. C++ grpc::.

llama.cpp Git commit SHA vs. Rust NAR FFI wrapper.

Qt SDK version vs. CXX-Qt crate version vs. rustc version.

OpenSSL (version used by C++ vs. version Rust openssl-sys crate binds to).

F. Dependency Update Strategy:

Principle: Regular, cautious updates for security and features, minimizing disruption.

Mandate:

Conduct a quarterly review of all major dependencies for updates and security advisories.

Use cargo-audit and manual CVE tracking for C++ dependencies.

Perform updates in dedicated branches, triggering the full CI pipeline (all unit, integration, E2E tests). Manual QA sign-off is required before merging.

Update one major dependency or a small group of related dependencies at a time.

G. Abstraction Layers for Volatile/Replaceable Dependencies:

Principle: Encapsulate external dependencies with unstable APIs or those likely to be replaced behind internal abstraction interfaces.

Mandate:

Service Storage: Any service requiring persistent storage (e.g., Mission Service) will use a Rust trait to abstract the chosen database (e.g., MySQL, Redb).

NAR Inference Engine: The Rust NAR core library will expose an internal Rust trait for inference, implemented by a wrapper around llama.cpp's C API (via Rust FFI).

ZKP Prover Service: The Rust Identity Service will use an internal trait for ZKP interactions, abstracting the specific prover API.

H. API First Design:

Principle: Define API and FFI contracts before implementation.

Mandate: .proto and OpenAPI specs for Rust services, the C-FFI header for the Rust NAR library, and the Rust-QML FFI contract (via CXX-Qt) for the client will be finalized, reviewed, and code-generated before service/component implementation begins in earnest.

I. Idempotency in Services:

Principle: Design service request handlers to be idempotent or use mechanisms to prevent duplicate processing on retries.

Mandate: The design of key service endpoints (e.g., payment webhooks, mission reward claims) will include strategies for idempotency (e.g., unique request IDs, checking processed IDs from a cache like Redis). This will be explicitly tested.

J. Asynchronous Everywhere:

Principle: Utilize non-blocking, asynchronous operations for all I/O to maximize performance and responsiveness.

Mandate: The entire Rust backend (all microservices, including the Indexer) will extensively use async/await with the Tokio runtime. The C++ Game Servers will use Asio or similar patterns. The Rust Client Application logic will use Tokio for its asynchronous gRPC calls, integrated with the Qt event loop.

K. Distributed Tracing & Structured Logging:

Principle: Comprehensive, consistent observability is mandatory for debugging and monitoring a distributed system.

Mandate: Implement structured JSON logging across all services and C++ components using the Rust tracing crate and C++ spdlog/Boost.Log. Integrate OpenTelemetry for distributed tracing: trace IDs will be generated at API entry points and propagated through all subsequent service calls. All logs must include the trace_id.

L. Health Checks & Service Discovery:

Principle: All services must be monitorable and discoverable for robust production operations.

Mandate: All Rust backend services will expose gRPC health check endpoints (grpc_health_v1). Service discovery will be handled via DNS within the local Docker Compose network and Kubernetes in the future cloud environment.

M. Configuration Management & Secrets:

Principle: Externalized, layered configuration and secure secret management are non-negotiable.

Mandate: Employ a consistent configuration pattern: secure defaults in Git, environment-specific overrides not in Git, with environment variables having the highest precedence. In the cloud-ready architecture, use AWS Secrets Manager for all production secrets (API keys, database credentials, JWT keys).

N. Data Migration Strategies:

Principle: Plan for schema evolution from day one.

Mandate: On-chain event schemas emitted by the L3 smart contracts will be versioned. The Indexer will have documented strategies for re-indexing or online schema migration (e.g., blue/green index deployments with alias swapping in Elasticsearch/Typesense). These strategies will be documented in ADRs.

O. Testing Pyramid Adherence:

Principle: Prioritize fast, reliable tests to ensure code quality and development velocity.

Mandate: The project will have a strong emphasis on unit tests for business logic. Service-level integration tests will mock external APIs and dependencies. A smaller number of carefully chosen end-to-end (E2E) tests will validate full user flows. This strategy is documented in docs/TESTING_STRATEGY.md.

P. Client-Side Resilience:

Principle: The Control Center Client must operate gracefully during temporary backend service disruptions.

Mandate: The client's Rust application logic will implement API call retries (with exponential backoff & jitter), local caching for non-critical UI data (using SQLite or similar), clear user-facing error messages, and graceful degradation of features when a backend service is unavailable.

Q. NAR Model Management & Updates:

Principle: The local AI model must be delivered to users securely and robustly.

Mandate: The Control Center Client's nar-model-handler (implemented in its Rust application logic) will ensure: secure HTTPS download of the Gemma3 1B GGUF model from a production CDN/S3, mandatory SHA256 checksum verification against a trusted value, model version checking, and atomic updates (download to a temporary file, verify, then replace the existing model).

R. Secure by Design & Default:

Principle: Security is a foundational, non-negotiable quality attribute integrated into every phase of the lifecycle.

Mandate:

Threat Modeling: Conduct iterative threat modeling for all critical components (L3 Smart Contracts, Identity, Marketplace, NAR FFI, Client CXX-Qt FFI, Game Servers) and user flows, starting in Phase 0. Document threats and mitigations in ADRs.

Secure Coding Practices: Adhere to documented secure coding guidelines for Rust, C++, and QML. Conduct regular, security-focused peer code reviews.

Input Validation: Implement strict, schema-based input validation at all service and FFI boundaries for type, length, format, and range. Deny unexpected/malformed inputs by default.

Authentication & Authorization: Enforce robust authentication (zkLogin, JWT, mTLS) for all API endpoints and service-to-service communication. Implement fine-grained authorization based on the principle of least privilege.

Cryptography: Use strong, industry-standard cryptographic algorithms and libraries (AES-256-GCM, Ed25519/ML-DSA, PLONK-based ZKPs) correctly configured. Securely manage all keys via a secrets management system.

Dependency Management: Continuously monitor and rapidly patch vulnerable third-party libraries (as per Principle F).

Security Testing: Integrate SAST, DAST, fuzz testing (for FFIs and parsers), and vulnerability scanning (for container images) into CI/CD pipelines.

Secure Defaults: Ensure all system configurations and features default to the most secure settings (e.g., enable_crypto: false).

Incident Response Plan: Develop, maintain, and drill a comprehensive incident response plan.

Regular Security Audits: Conduct internal security reviews at each phase gate and schedule external smart contract and penetration testing before major public releases.



II. Core Platform Overview

The BUNKERVERSE platform is a proprietary, secure, and massively scalable gaming ecosystem designed to create a persistent, interconnected universe. It is built by BUNKER CORP, primarily in Rust, C++, and Unreal Engine 5. The platform's architecture is built on the BUNKERVERSE Netchain, a custom Layer 3 AppChain leveraging the Arbitrum Orbit framework. It settles to the Arbitrum One Layer 2, inheriting the full security of the underlying Ethereum mainnet.

Key Components & Technologies:

BUNKERVERSE Netchain (Custom L3 AppChain): The canonical source of truth. A proprietary Arbitrum Orbit chain, configured to use NTC as the native gas token and settling to Arbitrum One. All core logic (NFTs, marketplace, progression) is implemented as on-chain smart contracts written in Rust (via Arbitrum Stylus).

Platform Microservices: A suite of independent, horizontally scalable Rust services providing business logic. This includes the Marketplace Service (built with Axum), Identity Service (zkLogin), Account Service, Payment Service (Stripe), Mission Service, and others.

Control Center Client: The primary user interface and gateway to the BUNKERVERSE. It is architected with a C++ Qt shell, core application logic written in Rust, and a dynamic, hardware-accelerated UI built with QML. The Rust logic and QML UI are bridged securely using CXX-Qt.

Netchain AI Runtime (NAR): A client-side Rust library wrapping the llama.cpp C++ engine to run a local Gemma3 1B GGUF model. It integrates with the Control Center Client via a security-hardened C-FFI to generate dynamic, contextual narratives.

Gaming Ecosystem (Post-MVE): Exclusive gameplay within Constructs, hosted on C++ UE5 Dedicated Game Servers. Server orchestration and scaling are managed by Agones on Kubernetes.

Supporting Infrastructure: All services are containerized for local development via Docker Compose, and architected to be cloud-ready for future deployment to AWS/EKS. This includes Elasticsearch/Typesense for the Indexing Layer, MySQL and Redis for specific service needs, and IPFS for NFT metadata storage.
