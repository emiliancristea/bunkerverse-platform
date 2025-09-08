Task 2.4: Global Indexing Layer (Rust) - Functional Basic Query Implementation
(Serving Securely Indexed Live P2 MVE State with Full Schema Defaults - Principles H, J, K, O, R)
Technical Reference
gRPC/tonic Documentation
Elasticsearch/Typesense Rust Client & Query DSL Documentation
Finalized v0.1 indexer_service.proto API Contract and Protobuf Schemas
Context/Problem Statement
The Indexer's data ingestion pipeline is now live (Task 2.3), populating its database with real-time data from the L3 smart contracts. However, the Indexer's public-facing gRPC API still consists of the hardcoded "smart stubs" from Phase 1. This task is to replace those stubs with fully functional query handlers that perform real queries against the indexing engine. This is the critical step that makes the live on-chain data accessible to the rest of the backend and the client, completing the full data flow loop.
Measurable Objectives
All gRPC handlers in the Indexer service are re-implemented to perform real queries against the live Indexer Backend (Elasticsearch/Typesense).
All API endpoints are secured with placeholder authorization stubs and strict, mandatory input validation.
The service can successfully serve live P2 data ( reflecting default states from L3 smart contracts) for all core query types.
Initial performance baselines for key queries are established and documented.
Implementation Guidance
Action: Implement the gRPC API handlers in the Rust Indexer (/indexer/src/service/query_handler.rs) to serve queries based on the live P2 data that is now being ingested from the L3 Smart Contracts and stored in the Indexer Backend (Elasticsearch/Typesense) via Task 2.3. This task involves replacing all the P1 hardcoded API stubs with handlers that perform real queries against the indexing engine. All APIs must be secured with placeholder authentication/authorization stubs and strict input validation.
Query Handler Implementation (Rust - for all GlobalQueryService RPCs - Functional with Live P2 Data & Full Schema Defaults):
o Re-implement all gRPC handlers (e.g., GetPlayerProfile, GetPlayerOwnedNfts, GetNftDetails, GetAIAgentInputDataContext) to use the live Indexer Backend.
o Each handler will use the initialized Indexer Backend client (from P1.4) to construct and execute queries.
o Strict Input Validation (Principle R): This is the first step in every handler. All incoming gRPC request parameters must be rigorously validated against schema constraints and business rules before being used to construct a query.
Validate ID formats (e.g., is the player_id string a valid on-chain address format?).
Validate pagination parameters (e.g., page_size must be > 0 and <= 100).
Validate filter values (e.g., if filtering by ItemRarityProto, ensure the provided value is a valid enum member).
Sanitize any string inputs that might be used in complex query string construction to prevent injection attacks specific to the Indexer Backend's query language (e.g., Lucene query syntax injection).
Reject any invalid requests immediately with a grpc::Status::new(Code::InvalidArgument, "...").
o Authorization (Stub for P2 - Principle R): Before executing the query, implement a placeholder authorization check.
fn authorize_query(requesting_principal: &PrincipalId, resource_owner_id: Option<&PlayerId>) -> bool
For P2, this function can have simple logic: if resource_owner_id is provided, check if it matches the requesting_principal.
This ensures the authorization step exists in the code flow, ready to be enhanced with full role-based access control (RBAC) later.
If authorization fails, the handler must return a grpc::Status::new(Code::PermissionDenied, "...").
o Query Execution and Response Mapping:
GetPlayerProfile: Construct a query to fetch the document from the player_index. If found, map the indexed data (live ActiveBunkerguardDataProto with P2 default class/affiliation/stats, live XP/NTC balances) into the PlayerProfileResponseProto.
GetNftDetails: Construct a query to fetch the document from the nft_index. If found, map the indexed data into the NftDetailsResponseProto.
GetPlayerOwnedNfts: Query the player_index for the player's owned_nft_ids_by_type list. Then, perform a multi-get query on the nft_index to fetch the details for those IDs.
GetAIAgentInputDataContext: This will be the most complex query. It will fetch the player's profile from player_index and details for their key owned/equipped NFTs from nft_index. Combine this live data into the comprehensive AIAgentInputDataContextProto response.
GetMarketListings: For P2, this will query the nft_index for items where market_status.is_listed == true. Since few items will be listed in P2, this query will likely return an empty or small list, but it will be a live query.
o Error Handling, Logging (tracing with Trace IDs - Principle K):
Implement robust, security-conscious error handling. If the Indexer Backend returns an error, the service should log the detailed internal error but return a generic grpc::Status::new(Code::Internal, "An internal error occurred") to the client.
Use tracing to log the constructed query (in debug builds) and the outcome.
Testing (Rust):
Unit Tests: Test the query construction logic for each RPC handler. Test the response mapping logic. Exhaustively test the input validation logic. Test the placeholder authorization logic.
Integration Tests (Rust - Indexer Service with Live P2 Data):
Set up the full L3 Smart Contracts -> Indexer pipeline from Task 2.3's integration tests, populating a live Indexer Backend (e.g., Elasticsearch in Docker) with live P2 event data.
Start the Indexer gRPC service, now with the functional query handlers.
Use a gRPC client to call all GlobalQueryService RPCs:
Call GetPlayerProfile for a player created in the test setup. Verify the response contains the correct, live P2 data.
Call GetNftDetails for an NFT minted in the test setup. Verify the response contains its full, GDD-defined P2 default attributes.
Verify that queries for non-existent data return NotFound.
Verify that queries with invalid input parameters are rejected with InvalidArgument.
Verify that placeholder authorization checks correctly return PermissionDenied when tested.
Measure basic query latencies for fetching this rich P2 data to establish a baseline.
Update docs/progress_logs/progress_phase_2.md:
o Log the implementation of the functional query handlers, replacing the P1 stubs.
o Detail the queries being constructed for key RPCs.
o Document the strict input validation rules implemented for each RPC endpoint.
o Document the placeholder authorization logic.
o Report initial query performance baselines.
o Document adherence to First Principles H, J, K, O, R.
Design Rationale
Replacing the stubs with live query handlers is the final step in making the backend a truly dynamic system. The strict separation of concerns, where smart contracts handle writes and the indexer handles reads, is a fundamental pattern for building scalable blockchain applications. Implementing strict input validation and placeholder authorization at this stage embeds security into the core logic from the very beginning.
Operational Considerations
Local-First: The Indexer service, with its live query handlers, will run in the local Docker Compose simulation, querying the co-located Elasticsearch/Typesense container.
Cloud-Ready: The performance of these query handlers will be a critical set of metrics to monitor in a production environment. The initial performance baselines established here will inform the configuration (e.g., shard count, replica count) of the production Elasticsearch/OpenSearch cluster.
Verification & Validation Criteria
All unit and integration tests pass.
The Indexer service successfully serves queries for live indexed P2 data (originating from the L3 smart contracts) via all its gRPC endpoints.
Input validation and placeholder authorization are functional and proven to work via tests.
Testing Methodologies
A combination of unit tests with mocked Indexer Backend clients, and a full integration test that runs the entire on-chain and off-chain data pipeline locally to validate the queries against real, indexed data.
Version Control Strategy
Branching: The live query handlers are developed on a feature/indexer-live-queries branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory security review of all query handlers is required, with a primary focus on the input validation and sanitization logic to prevent any query injection vulnerabilities.
The placeholder authorization logic must be reviewed to ensure it correctly fails-closed (denies by default).
ReviewedBy: Indexer Lead, Rust Backend Lead, Security Lead, and leads of API consuming services.
ReviewOutcome: Approved.
ValidationMethod: All unit and integration tests pass. The Indexer service successfully serves queries for live indexed P2 data. Input validation and placeholder authorization are functional.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 2.4: Implemented Rust Indexer Functional Basic Query Endpoints (Serving Securely Indexed Live P2 MVE State with Full Schema Defaults)." @Phase2/