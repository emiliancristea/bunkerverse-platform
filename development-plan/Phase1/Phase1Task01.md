Task 1.1: Rust Identity Service - Full zkLogin Implementation & Functional Secure API
(Interacting with L3 Smart Contract & Account Service Stub, Handling Full UserRegistered Payload for MVE)
Technical Reference
OAuth 2.0 / OpenID Connect (OIDC) Specifications
oauth2 and openidconnect Rust crates
jsonwebtoken Rust crate
sqlx Rust crate documentation
ethers-rs or similar L3 client library documentation
ADR-005 (zkLogin Flow), Finalized v0.1 gRPC/OpenAPI contracts (from P0.3)
Context/Problem Statement
The first functional pillar of the BUNKERVERSE must be a user's identity. To achieve our "On-Chain in Substance, Off-Chain in Appearance" doctrine, we need a robust, secure, and seamless onboarding system that completely abstracts away Web3 complexity. This task involves building the complete, security-hardened zkLogin flow in a dedicated Rust microservice. This service will handle the entire OAuth 2.0 process, securely manage user data, and-critically-interact with our live L3 Testnet to create the user's initial on-chain footprint.
Measurable Objectives
The Rust Identity Service is fully functional and can handle the complete server-side zkLogin flow for all configured OAuth providers (Google, Apple, etc.).
A successful new user registration results in a UserRegistered transaction being successfully submitted to the L3 Smart Contracts on the live Testnet.
A valid platform session JWT is securely generated and issued to the client upon successful registration or login.
The service is fully unit and integration tested, including all security-critical paths.
The service's implementation adheres to all 18 First Principles.
Implementation Guidance
Action: Implement the complete, security-hardened zkLogin flow in the Rust Identity Service (/services/identity/rust/src/main.rs and related modules) based on ADR-005 and finalized gRPC/OpenAPI contracts v0.1 (from P0.3). The implementation must be secure for the MVE and adhere to Principles A, H, I, J, K, L, M, R.
Secure OAuth Integration (Rust):
Implement Rust OAuth 2.0/OpenID Connect client logic using the chosen, security-vetted Rust crate (selection finalized in P0.2 PoC).
Integrate support for all selected OAuth providers.
Securely manage OAuth client IDs and secrets: Load from configuration (which loads from a local .env file for the local Docker Compose simulation).
Implement gRPC/REST endpoints as per P0.3 API contract:
GetLoginProviders(): Returns a list of configured OAuth providers.
InitiateZkLoginRequest: Generates a cryptographically secure state parameter (for CSRF protection) and nonce (for ID token replay protection). If using PKCE, generates a code_verifier and code_challenge. Securely stores these values associated with a short-lived session ID (e.g., in Redis with TTL). Returns the provider's authorization URL.
HandleOAuthCallback: Verifies the received state against the stored value. Exchanges the authorization code for an ID token. Rigorously validates the received ID token's signature, issuer, audience, expiry, and critically, the nonce against the stored value. Extracts verified user claims.
Secure ZKP Input Preparation & User Salt Management (Rust):
Implement Rust logic to construct the full ZKP inputs based on the verified ID token claims.
User Salt Management:
Upon first successful login with a new provider subject ID: Generate a cryptographically secure random user salt.
Store the salt securely encrypted in the Identity Service's MySQL database. Encryption must use AES-256-GCM with a dedicated Data Encryption Key (DEK) per salt, where the DEK itself is encrypted with a Key Encryption Key (KEK) managed via a master key from the local .env file. The MySQL table schema: (user_account_id_hash VARCHAR(64) PRIMARY KEY, provider_id VARCHAR(255), encrypted_salt BLOB, dek_encrypted_with_kek BLOB, kek_id VARCHAR(255)).
ZKP Verification (Mocked Cryptographically for P1 - Critical Security Note):
The ProcessZkProofAndGetSession endpoint will receive ZKP-related data.
For P1: Perform structural validation of the received proof object.
Log "CRITICAL TODO (Security P1.1): Implement full ZKP cryptographic verification (PLONK-based) in Rust for on-chain activation. Currently using MOCK verification for P1."
This mock verification is acceptable ONLY for the MVE and must be replaced before any non-custodial features go live.
On-Chain Account Identifier Derivation (Rust):
Implement the secure account identifier derivation logic as defined in ADR-005 (e.g., hash of (provider_subject_id + user_salt + app_specific_pepper)). Use a cryptographically secure hash function (e.g., BLAKE3).
User Registration/Session Management (Rust gRPC/REST Endpoint ProcessZkProofAndGetSession):
Accepts ZKP-related data.
Performs (mocked for P1) ZKP verification.
Derives the user's on-chain account address.
Calls the Rust Account Service's CheckExistenceByAccountId gRPC endpoint (Account Service uses its Indexer client stub from P0, which returns a hardcoded "not found" for new users in P1).
If user is new:
Construct a UserRegisteredPayload canonical event. This payload must include the account_id, zklogin_provider_info, and an initial ActiveBunkerguardDataProto with all default values as per P0.3 schema finalization for the MVE. Mode On-Chain fields (e.g., ntc_staking_details_proto) must be initialized to their empty/default states.
Submit this event by creating and sending a transaction to the L3 sequencer's SubmitTransaction gRPC method.
Generate Platform Session JWT:
Use a security-vetted Rust JWT crate.
Sign the JWT using a strong asymmetric key loaded securely from the local .env file.
JWT Claims: sub (user's on-chain address), iss (IdentityService URL), aud (BUNKERVERSE platform), exp (e.g., 1 hour).
Return the JWT to the client.
Database (MySQL via Rust sqlx):
Implement the MySQL schema (using sqlx-cli migrations) for the user_salts table.
All database queries must use sqlx parameterized queries to prevent SQL injection.
Logging & Health (Principles K, L):
Implement detailed structured logging using the tracing crate for all steps. Include trace IDs.
Expose a gRPC health check endpoint.
Security Hardening (Principle R):
Implement strict input validation on all API request parameters.
Implement rate limiting on sensitive endpoints.
Ensure error messages are generic and do not leak internal details.
Design Rationale
This service acts as the secure bridge between the Web2 and Web3 worlds. The zkLogin flow provides a best-in-class user experience, while the secure salt management and on-chain transaction submission ensure that this seamless experience is built on a foundation of non-custodial principles. Mocking ZKP verification is a pragmatic choice for the MVE that allows the full user flow to be built and tested while deferring the most complex cryptographic implementation, which is only strictly necessary for full non-custodial, Mode On-Chain features.
Operational Considerations
Local-First: The service will run in a Docker container as part of the docker-compose.yml setup. It will connect to a MySQL container and a Redis container. All secrets (OAuth credentials, JWT key, KEK) will be provided via the .env file.
Cloud-Ready: The service is architected to be stateless (with session/salt data in Redis/MySQL). Its configuration loader is designed to switch from the .env file to AWS Secrets Manager with no code changes, facilitating a seamless future migration. The database schema migrations are managed by sqlx-cli, ensuring repeatable deployments.
Verification & Validation Criteria
All unit and integration tests pass.
A successful end-to-end zkLogin flow, demonstrated against the local Docker Compose simulation, results in a UserRegistered transaction being submitted to the mock L3 sequencer stub, and a valid platform JWT being issued.
The security review checklist for the Identity Service P1 scope is completed and signed off by the Security Lead.
Testing Methodologies
Unit Tests:
Test OAuth client logic (mocking provider responses).
Test ID token validation logic (valid/invalid signatures, expiry, nonce).
Test secure salt generation, encryption, storage, and retrieval/decryption (mocking KMS).
Test UserRegisteredPayloadProto construction.
Test JWT generation and validation.
Integration Tests:
Set up the Identity Service with a test MySQL database and a mock OAuth provider endpoint (e.g., using wiremock-rs).
Mock the Account Service and L3 sequencer gRPC endpoints.
Test the full zkLogin flow from InitiateZkLoginRequest to JWT issuance.
Verify the correct UserRegistered event is submitted to the mock L3 sequencer.
Verify secure salt handling throughout the flow.
Test all error paths (invalid OAuth code, failed ID token validation, etc.).
Version Control Strategy
Branching: All service code is developed on feature/ branches off of develop.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory, detailed security code review of the entire zkLogin flow is required. This must cover:
OAuth state/nonce/PKCE validation to prevent CSRF and replay attacks.
Secure salt management (generation, encryption-at-rest, access control).
Secure JWT handling (strong signing key, correct claims, short expiry).
Prevention of SQL injection.
The documented "CRITICAL TODO" for ZKP verification is noted and accepted for the MVE scope.
ReviewedBy: Rust Backend Lead, Security Lead, Lead Architect, Client Lead (as consumer of this service).
ReviewOutcome: Approved for P1 functionality (MVE focus, ZKP crypto verification mocked).
ValidationMethod: All unit and integration tests pass. Successful end-to-end zkLogin flow demonstrated in a test environment, resulting in a UserRegistered transaction being submitted to the L3 sequencer stub, and a platform JWT being issued. Security review checklist for Identity Service P1 scope completed and signed off.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.1: Implemented Rust Identity Service - Functional & Secure zkLogin Flow for MVE (Full UserRegistered Payload to L3 Sequencer Stub, ZKP Crypto Mocked)." @Phase1/