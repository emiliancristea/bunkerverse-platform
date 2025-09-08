Task 1.4: Global Indexing Layer (Rust) - Service Foundation & Functional API Stubs
(Full Schema Support & Default Values for MVE - Principles H, J, K, L, O, R)
Technical Reference
 gRPC/tonic Documentation
 tracing crate documentation
 Finalized v0.1 indexer_service.proto API Contract
 docs/SMART_STUBS_SPECIFICATION.md
Context/Problem Statement
The client application requires a fast and efficient way to query complex game state data (e.g., player profiles, NFT details, marketplace listings). Querying the L2 smart contracts directly for this data is highly inefficient and slow. The Indexer is the dedicated "read layer" service that solves this problem. In this initial phase, before the live L2 event ingestion pipeline is built (in Phase 2), the Indexer must act as a functional but static data provider. This involves implementing its full gRPC API but having the handlers return hardcoded, yet fully schema-compliant, default responses. This allows the client team to build against a stable, predictable API from day one.
Measurable Objectives
 The basic Rust service structure for the Indexer is implemented and runs within the local Docker Compose simulation.
 All gRPC handler functions for the GlobalQueryService are implemented as functional stubs.
 Each endpoint returns a hardcoded, schema-valid default response that fully and correctly populates the rich Protobuf messages from P0.3.
 Each endpoint performs strict, security-focused input validation on all request parameters.
Implementation Guidance
Action: Implement the basic Rust service structure for the Indexer in /indexer/src/. This involves setting up its gRPC API handlers to return schema-valid, hardcoded default responses that fully conform to the rich Protobuf schemas defined in P0.3. These responses will be tailored for a new player's experience in the MVE, providing plausible default data for detailed player profiles (including default class/affiliation/stats), NFTs (with default full attributes), etc. This phase does not involve implementing live event ingestion from the L2; the Indexer acts as a functional but static data provider.
Indexer Service Core (/indexer/src/main.rs, /indexer/src/service/query_handler.rs - gRPC Handlers for GlobalQueryService):
 Initialize the chosen indexing technology backend client (e.g., elasticsearch::Elasticsearch or typesense::Client from their official Rust crates) but do not implement any data writing logic in this task. For P1, the client can be initialized with placeholder/dev credentials.
 Implement all gRPC handler functions for the GlobalQueryService as defined in /protos/indexer_service.proto.
 Functional API Stubs with Full Schema & Sensible MVE Defaults: Each gRPC handler must perform the following steps:
 Log the incoming request with its parameters using the tracing crate.
 Perform strict input validation on all request parameters (as defined in P0.3, e.g., PlayerId format, pagination limits). This is a critical security step (Principle R).
 Construct and return a schema-valid, hardcoded default response. The response must fully populate the rich Protobuf messages from P0.3 with sensible default values that represent a new player's state in the MVE.
Detailed Default Response Logic:
 GetPlayerProfile(PlayerProfileRequest):
 Input Validation: Validate player_id format.
 Hardcoded Response Logic: Return a PlayerProfileResponseProto containing a fully populated ActiveBunkerguardDataProto with MVE defaults:
 level: 1.
 current_class_opt: None.
 current_affiliation: ClassAffiliationProto::Neutral.
 final_stats: A CoreStatsProto with all 12 sub-stats set to 10.
 equipped_items: Empty map.
 reputation_score: 0.
 The response will also include default balances suitable for the MVE:
 xp_balance: 0.
 ntc_balance: 0.
 credits_balance: 0 (as Credits are a future on-chain feature).
 GetNftDetails(NftDetailsRequest):
 Input Validation: Validate nft_id format.
 Hardcoded Response Logic: Return an NftDetailsResponseProto containing an NftWithDetailsAndMutableStateProto.
 details: A fully populated NftDetailsProto with plausible defaults for a generic item.
 mutable_state: A NftMutableStateProto with defaults: current_owner_id: the requested player_id, current_condition: ItemConditionProto::New, is_soulbound: false, market_status_opt: None.
 GetAIAgentInputDataContext(AIAgentInputDataContextRequest):
 Input Validation: Validate player_id format.
 Hardcoded Response Logic: Return a comprehensive AIAgentInputDataContextProto by combining the default data from GetPlayerProfile and a few default owned NFTs (like the one from GetNftDetails).
 GetMarketListings(MarketListingsRequest):
 Input Validation: Validate filter parameters and pagination settings (e.g., page_size <= 100).
 Hardcoded Response Logic: Return a MarketListingsResponse with an empty list of listings and a pagination_token of "".
 Pagination Handling: For list-based endpoints, ensure the response message correctly handles empty lists and includes pagination fields.
 L2 Event Ingestion (Placeholder - /indexer/src/ingestion/l2_event_subscriber.rs):
 Implement the basic structure for the L2 event subscriber module from the P0 design.
 It will contain a function async fn start_l2_subscription(...) that attempts a single gRPC connection to a configured L2 sequencer SubscribeToEvents endpoint stub.
 The function will log the connection attempt (e.g., "Attempting to subscribe to L2 event stream at [endpoint]...").
 It will not process any events; the stream can be immediately closed or the task can exit after the connection log. This verifies the module structure and configuration loading for P1.
 Logging (tracing) & Health (grpc_health_v1) (Principles K, L):
 Implement structured logging for all incoming gRPC requests and the hardcoded responses being returned.
 Implement the gRPC health check service to report a SERVING status.
 Security Review (Principle R):
 Mandatory peer review of the Indexer service stub implementation, focusing on:
 Thoroughness of the input validation logic for all query API request parameters.
 Ensuring hardcoded default responses do not leak any sensitive information.
 Confirming that error messages are generic and secure.
 Testing (Rust):
 Unit Tests:
 For each gRPC API handler, write a test that verifies it returns the exact, expected hardcoded default response.
 Validate that all fields of the rich P0.3 Protobuf schemas are correctly populated.
 Write tests for the input validation logic of each API handler, providing invalid inputs and asserting that the handler returns the correct INVALID_ARGUMENT gRPC status code.
 Integration Tests:
 Start the Indexer gRPC service stub in a test environment (e.g., within the local Docker Compose simulation).
 Use a Rust gRPC client to call each API endpoint with valid and invalid inputs and assert the correct schema-valid responses or gRPC error statuses are received.
 Update docs/progress_logs/progress_phase_1.md:
 Log the implementation of the Indexer's gRPC handlers as functional stubs.
 Crucially, detail the specific hardcoded default responses for the key APIs, ensuring they are documented to fully populate the rich P0.3 schemas as expected for the MVE.
 Document the strict input validation rules implemented for each API request.
 Log the outcome of the security review.
 Document adherence to First Principles H, J, K, L, O, R.
Design Rationale
Building a functional API stub is a cornerstone of parallel development. It provides a stable, predictable, and schema-compliant API for the client and other services to build against, long before the complex live data ingestion pipeline is complete. This decouples the development of the data consumer (the client) from the data producer (the live indexer), dramatically accelerating the overall project timeline.
Operational Considerations
 Local-First: This Indexer stub will run as a service within the docker-compose.yml file, providing a reliable data source for local client development and CI.
 Cloud-Ready: The service is already containerized and configured via environment variables. The hardcoded logic within its handlers will be replaced by live query logic in Phase 2, but the service's external contract and operational posture (logging, health checks) will remain the same, ensuring a smooth upgrade.
Verification & Validation Criteria
 All unit and integration tests pass.
 All gRPC endpoints are callable from another service (e.g., the Account Service) or a CLI tool. They perform strict input validation and return schema-valid default data that fully reflects the rich MVE schemas.
Testing Methodologies
As detailed in the implementation section, a combination of unit tests (for handler logic and validation) and integration tests (calling the running service stub) will be used.
Version Control Strategy
 Branching: The service stub is developed on a feature/ branch (e.g., feature/indexer-service-stubs).
 Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
 The mandatory peer review of the input validation logic is the primary security checkpoint for this task. It ensures that even the stub service is not vulnerable to basic malformed request attacks.
ReviewedBy: Indexer Lead, Rust Backend Lead, Security Lead, and leads of API consuming services (e.g., Account Service Lead, AI Data Service Lead).
ReviewOutcome: Approved.
ValidationMethod: All unit and integration tests pass. All gRPC endpoints are callable, perform strict input validation, and return schema-valid default data that fully reflects the rich MVE schemas with plausible, documented defaults for the MVE.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.4: Implemented Rust Indexer Service Foundation & Functional API Stubs (Full Schema with Sensible Default Hardcoded Responses for MVE, Input Validation)." @Phase1/

------------------------------------------------------------------------------------------------------------------

