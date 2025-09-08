Task 1.3: L3 Transaction Submission Service
(Functional Ingestion, Full Schema Validation, & Secure Submission to L3 Sequencer - Principles I, J, K, L, R)
Technical Reference
gRPC/tonic Documentation
Finalized v0.1 Protobuf Schemas
Arbitrum Orbit Sequencer API/RPC documentation
ethers-rs or similar L3 client library documentation
Context/Problem Statement
While our backend services (like the Identity Service) need to submit transactions to our sovereign L3, they should not be directly responsible for the low-level complexities of transaction signing, nonce management, or direct interaction with the L3 sequencer. We need a centralized, secure, and robust entry point for all internally generated transactions. This task implements a lightweight "Transaction Submission Service" that acts as this gateway. It performs rigorous stateless validation on all incoming transaction payloads before signing and securely forwarding them to the L3 sequencer.
Measurable Objectives
A functional Rust gRPC service is implemented that exposes a SubmitTransaction endpoint.
The service performs thorough, mandatory stateless validation against the full MVE schemas for all incoming transaction payloads.
The service correctly signs valid transactions and submits them to the Arbitrum Orbit L3 sequencer running in the local Docker Compose simulation.
The service returns an appropriate acknowledgment, including the transaction hash from the L3.
The service's input validation logic passes a mandatory security review.
Implementation Guidance
Action: Implement the initial functional primitives for a Rust-based Transaction Submission Service. This MVE implementation will feature a centralized gRPC endpoint that performs rigorous stateless validation on incoming transaction requests against the full MVE schemas. It will then sign and securely submit these transactions to the appropriate L3 sequencer.
Submission Service Core (/services/submission/src/main.rs - gRPC Handler for SubmitTransaction):
Implement the SubmitTransaction(TransactionRequestProto) gRPC handler. This is the primary internal entry point for services to submit a transaction to the L3 sequencer.
Transaction Validation (Functional Stateless against Full Schema - Principle R):
Deserialize TransactionRequestProto.serialized_event_proto to the appropriate EventProto.
Perform thorough, mandatory, stateless validation based on the EventProto's type and all fields in its payload. This logic is critical for protecting the L3 Smart Contracts from malformed data.
Validation Rules Examples:
NftMintedPayloadProto: nft_details field must be present, enums must be valid, stat boosts must be within a sane range (e.g., -100 to +100).
UserRegisteredPayloadProto: account_address must be present and match expected format, default active_bunkerguard_data must be present and match P1 specification.
ItemEquippedPayloadProto: item_slot must be a valid enum member.
Signature & Replay Protection (Simplified for MVE Service-to-Service):
For internal service-to-service calls, trust is established via mTLS. We will bypass complex signature and replay validation at this layer for P1, as the primary validation will occur at the L3 smart contract level (e.g., checking msg.sender).
Transaction Submission Logic:
If any validation check fails, reject the request immediately with a specific gRPC error code (e.g., INVALID_ARGUMENT).
If validation passes, the service will:
Load its securely configured "admin" private key (from the local .env file).
Use an L3 client library (e.g., ethers-rs) to connect to the L3 sequencer's RPC endpoint.
Construct the raw transaction data to call the appropriate smart contract function (e.g., BunkerguardManager.registerUser(...)).
Sign the transaction.
Send the raw signed transaction to the sequencer.
Return Acknowledgment:
If the transaction is successfully accepted by the sequencer, return a TransactionAcknowledgementProto containing the transaction hash.
If validation fails, return an appropriate error.
Logging (tracing - Principles K):
Implement detailed structured logging throughout the submission and validation process.
Log every incoming transaction request with its unique ID.
Log detailed validation results (pass/fail with specific reasons for failure).
Log the transaction hash upon successful submission to the L3 sequencer.
Security Review (Principle R):
Mandatory, detailed peer review of the service's input validation logic. This is a critical security gate. The review must ensure that all fields of all possible P1 event types are validated against the full schema and sane limits, leaving no room for malformed or malicious transaction payloads.
Review logic for DoS protection (rate limiting, payload size limits enforced by the gRPC server).
Testing (Rust):
Unit Tests:
Exhaustively test the stateless transaction validation logic for every P1-scope EventProto type with both valid and invalid payloads.
Integration Tests:
In an integration test, start the Submission Service.
Mock the L3 sequencer's RPC endpoint.
Use a gRPC client to submit a high volume of TransactionRequestProto messages.
Verify the service correctly validates transactions against the full schemas (accepting valid ones, rejecting invalid ones).
Verify the service correctly constructs and forwards valid transactions to the mock sequencer.
Update docs/progress_logs/progress_phase_1.md:
Log the implementation of the Submission Service's gRPC handler and stateless validation module.
Document the outcome of the mandatory security review of the validation logic.
Document adherence to First Principles I, J, K, L, R.
Design Rationale
This Transaction Submission Service acts as a secure, centralized gateway to the L3. It abstracts the complexities of L3 interaction (nonce management, signing, gas estimation) away from other business-logic services, promoting a clean separation of concerns. Its primary role is to act as a stateless validation firewall, ensuring that only well-formed, schema-compliant transaction requests are ever signed and submitted to the L3 sequencer, thus protecting the on-chain smart contracts from a wide class of malformed data attacks.
Operational Considerations
Local-First: This service will run in its own Docker container and communicate with the Arbitrum Orbit sequencer container over the Docker network. Its admin key will be loaded from the shared .env file.
Cloud-Ready: In a production cloud environment, this service will be a critical, horizontally scalable component. Its admin key will be loaded securely from AWS Secrets Manager, and it will be granted network access to the production L3 sequencer endpoints.
Verification & Validation Criteria
All unit and integration tests pass. The service successfully ingests, performs full stateless schema validation, and routes valid transactions to a mock L3 sequencer.
The service correctly rejects transactions with invalid payloads.
Testing Methodologies
Unit Tests: Exhaustively test the validation logic for every transaction type against the schemas.
Integration Tests: Start the service and a mock L3 sequencer to test the full flow from gRPC request to a signed transaction being sent.
Version Control Strategy
Branching: The service is developed on a feature/transaction-submission-service branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The stateless validation logic is a critical security gate and must undergo a mandatory, detailed peer review by the Security Lead.
The handling and security of the service's "admin" private key is a paramount security concern.
ReviewedBy: L3 Smart Contracts Lead, Rust Backend Lead, Security Lead, Lead Architect.
ReviewOutcome: Approved for P1 MVE Scope.
ValidationMethod: All unit and integration tests pass. The service successfully ingests, performs full stateless schema validation, and submits signed transactions to a mock L3 sequencer.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.3: Implemented Rust L3 Transaction Submission Service (Functional Ingestion, Full Schema Validation)." @Phase1/