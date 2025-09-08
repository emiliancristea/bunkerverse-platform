Task 1.12: Basic Admin CLI - Enhanced Interaction
(Functional L3 Transaction Submission & Indexer Stub Queries - Principles H, R)
Technical Reference
clap and tonic crate documentation
Finalized v0.1 Protobuf Schemas (especially UserRegisteredPayloadProto, NftMintedPayloadProto)
serde_json for deserialization
OpenSSL for generating self-signed certificates
Context/Problem Statement
The basic Admin CLI created in Task 0.12 was designed to interact with simple stubs. Now that we have a functional Transaction Submission Service (Task 1.3) and Indexer stubs (Task 1.4) that handle the full, rich data schemas, the CLI must be enhanced to match. This upgrade is essential for enabling developers and QA to perform meaningful, complex testing of the Phase 1 backend, such as registering a new user with a complete default state.
Measurable Objectives
The Admin CLI is enhanced to accept full JSON payloads for transaction submission.
The CLI can successfully submit a full-schema UserRegistered event.
The CLI can successfully query the full-schema default responses from the Indexer stubs.
All CLI-to-service communication remains secured with TLS.
Implementation Guidance
Action: Enhance the Rust Admin CLI (from P0.12) to fully support submitting CanonicalEventProto payloads with full schema details to the now functional Rust Transaction Submission Service (P1.3) and querying the full schema default responses from the functional Rust Indexer API stubs (P1.4), ensuring all communication uses secure gRPC.
Command Implementation & Security:
o Update CLI commands (/tools/bunkerverse-admin-cli) to construct full Protobuf messages for P1 scope.
o bunkerverse-admin-cli netchain submit-transaction --payload-json '{...full_payload...}':
The --payload-json must now accept a JSON representation of the complete event payload (e.g., UserRegisteredPayloadProto or NftMintedPayloadProto).
The CLI will use serde_json to deserialize the JSON string into the appropriate full Rust Protobuf struct (e.g., deserializing the nested ActiveBunkerguardDataProto for a new user).
The CLI then constructs the TransactionRequestProto and sends it.
o All gRPC calls from the CLI to the L3 Transaction Submission Service and Indexer services (even when running locally in Docker Compose) must use a tonic client configured with TLS. For local development, this will use the self-signed certificates generated for the services.
o If services require authentication (even mock tokens for stubs), the CLI must support providing them via a command-line flag (e.g., --auth-token <token>).
Testing:
o Unit Tests (Rust CLI): Test the JSON deserialization logic for complex, nested Protobuf payloads. Mock the gRPC client to test command logic.
o Integration Tests (CLI against Docker Compose):
Run the full backend stack (functional Transaction Submission Service, Indexer API stubs) in the local Docker Compose simulation.
Use the CLI to submit a UserRegistered event with a full default payload. Verify the Transaction Submission Service logs its acceptance.
Use the CLI to query the Indexer stub for a player profile and verify the full-schema default response is returned and printed correctly.
Verify that attempting to connect without TLS (if the server is configured to require it) fails.
Update docs/progress_logs/progress_phase_1.md:
o Log the updates to the CLI to handle full-schema JSON payloads for event submission.
o Detail the enforcement of secure gRPC (TLS) for all CLI-to-service communication.
Design Rationale
Enhancing the Admin CLI is a critical enabler for effective testing. It allows for the precise simulation of complex events and state conditions that would be difficult or time-consuming to create through the UI. It becomes an indispensable tool for developers to test their service logic and for QA to write automated test scripts against the backend.
Operational Considerations
Local-First: This enhanced CLI will be the primary tool for developers to test and debug their services within the local Docker Compose environment throughout the MVE development.
Cloud-Ready: The same CLI binary can be used against deployed cloud environments by simply changing its configuration to point to the new endpoints and providing valid production credentials.
Verification & Validation Criteria
The enhanced CLI successfully interacts with the functional Transaction Submission Service and Indexer stubs running in the Docker Compose environment.
It demonstrates the ability to submit and query data using the full P1 MVE schemas over secure channels.
TLS communication is verified as mandatory.
Testing Methodologies
A combination of unit tests for the CLI's internal logic (especially JSON parsing) and integration tests that run the CLI against the live stack of stubs in the Docker Compose environment.
Version Control Strategy
Branching: The CLI enhancements will be developed on a feature/ branch (e.g., feature/admin-cli-p1-enhancements).
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The requirement for TLS on all gRPC connections is re-verified as a mandatory security checkpoint.
The logic for handling authentication tokens in the CLI must be reviewed to ensure it doesn't accidentally log or expose credentials.
ReviewedBy: Backend Lead, Indexer Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: The enhanced CLI successfully interacts with the functional services in the local Docker Compose simulation, demonstrating the ability to submit and query data using the full P1 MVE schemas over secure channels.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.12: Enhanced Rust Admin CLI for Functional L3 Transaction Submission & Indexer API Stubs (Full Schema Events & Responses, Secure gRPC)." @Phase1/