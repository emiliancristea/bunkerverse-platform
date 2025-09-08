Task 0.12: Basic Admin CLI - Structure & "Smart Stubs" Interaction
(Rust - Interacting with Secure L3 Sequencer & Indexer Smart Stubs - Principles H, R)
Technical Reference
clap crate documentation (for command-line parsing)
tonic crate documentation (for gRPC client)
Finalized v0.1 netchain_service.proto and indexer_service.proto API Contracts
OpenSSL (for generating self-signed certificates for local TLS)
Context/Problem Statement
With our backend services being developed as "smart stubs" running in Docker Compose, we require a powerful, scriptable tool to interact with their gRPC APIs for testing and debugging. A Command-Line Interface (CLI) is the perfect tool for this, allowing developers to easily submit transactions, query state, and validate the behavior of the backend without needing to use the full Control Center client. This accelerates development and provides a crucial tool for future server administration.
Measurable Objectives
A functional Rust-based Admin CLI is implemented using clap and tonic.
The CLI can successfully execute all defined commands against the L3 sequencer and Indexer smart stubs running in the local Docker Compose environment.
All gRPC communication from the CLI to the services is secured with TLS, even in the local environment.
Implementation Guidance
Action: Implement the initial structure and basic commands for a Rust-based Admin CLI. This CLI will interact with the "smart stub" versions of the L3 sequencer and the Indexer, using secure gRPC communication patterns.
Implementation (/tools/bunkerverse-admin-cli - Rust Project using clap and tonic):
Project Setup: Create a new Rust binary project using clap for command-line argument parsing and tonic for gRPC client functionality.
Configuration: CLI loads connection endpoints for the L3 sequencer stub and Indexer stub from a config file or environment variables.
Core Commands (Interacting with Smart Stubs):
bunkerverse-admin-cli netchain submit-transaction --payload-json <JSON_string_for_event_payload>:
Constructs a TransactionRequestProto containing a serialized event payload (deserialized from the JSON payload, matching the P0.3 defined schemas) and a placeholder signature.
Submits it to the L3 sequencer smart stub's SubmitTransaction gRPC endpoint.
Prints the TransactionAcknowledgementProto response.
bunkerverse-admin-cli indexer get-player-profile --player-id <PlayerID>:
Calls the Indexer smart stub's GetPlayerProfile gRPC endpoint.
Prints the returned PlayerProfileResponseProto (as formatted JSON).
bunkerverse-admin-cli indexer get-nft-details --nft-id <NftID>:
Calls the Indexer smart stub's GetNftDetails gRPC endpoint.
Prints the returned NftDetailsResponseProto (as formatted JSON).
Secure gRPC Communication:
All gRPC calls from the CLI to the L3/Indexer stubs must be configured to use TLS, even for local development against Docker Compose stubs (using self-signed certificates for local dev is acceptable for P0, with documentation on generating them). This establishes secure communication patterns from the outset.
If stubs are configured to simulate authentication (e.g., requiring a mock bearer token for admin actions), the CLI must support providing this token via a flag or config.
Error Handling: Robustly handle gRPC connection errors, API errors returned by stubs, and payload deserialization errors. Print user-friendly error messages.
Update docs/progress_logs/progress_phase_0.md:
Detail the CLI command structure and its implementation using clap.
Provide examples of gRPC calls made by the CLI to the L3 sequencer and Indexer stubs.
Document security considerations for CLI access and communication (TLS enforcement, mock token handling for stubs).
Design Rationale
A powerful admin CLI is an essential internal tool. It allows for scripted testing, easy state verification, and manual intervention if needed, greatly accelerating development and future operations. Enforcing TLS for all gRPC communication from day one, even locally, builds security into our default practices and tooling, which aligns with Principle R (Secure by Design).
Operational Considerations
Local-First: This CLI will be the primary tool for interacting with the backend services running in the local Docker Compose simulation.
Cloud-Ready: Because the CLI is configured via endpoints, the same tool can be used to interact with services deployed in the future cloud environment by simply pointing it to the new addresses and providing the correct credentials.
Verification & Validation Criteria
Successful execution of all defined CLI commands against the smart stubs running in the Docker Compose environment.
TLS communication is verified (e.g., using Wireshark locally or by stubs logging TLS handshake success).
The CLI correctly handles and displays errors returned by the stubs.
Testing Methodologies
Unit Tests (Rust CLI): Test the JSON deserialization logic for complex, nested Protobuf payloads. Mock the gRPC client to test command logic.
Integration Tests (CLI against Docker Compose): Run the full backend stack in Docker Compose. Use the CLI to submit a transaction and query the Indexer, verifying the expected responses.
Version Control Strategy
Branching: The CLI will be developed on a feature/ branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The requirement for TLS on all gRPC connections, even for an internal tool, is a mandatory security checkpoint.
The logic for handling authentication tokens in the CLI must be reviewed to ensure it doesn't accidentally leak credentials.
ReviewedBy: Backend Lead, Indexer Lead, Security Lead.
ReviewOutcome: Rust Admin CLI Structure & Smart Stub Interaction Approved for P0.
ValidationMethod: Successful execution of all defined CLI commands against the smart stubs running in Docker Compose. TLS communication verified. CLI handles stub errors gracefully.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 0.12: Rust Admin CLI 'Smart Stub' Integration (Full Schema, Secure gRPC Interaction with L3 Sequencer & Indexer Stubs)." @Phase0/