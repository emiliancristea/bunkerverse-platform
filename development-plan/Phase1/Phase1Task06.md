Task 1.6: Server-Side AI Data Service (Rust)
(Functional Stub for Context Query via Functional Rust Indexer API Stubs - Principles H, J, K, L, R)
Technical Reference
* gRPC/tonic Documentation
* jsonwebtoken Rust crate
* sqlx Rust crate
* Finalized v0.1 ai_data_service.proto and indexer_service.proto API Contracts
Context/Problem Statement
The client-side Netchain AI Runtime (NAR) requires a rich, comprehensive set of data about the player's current state to generate meaningful and contextual narratives. Directly exposing the full, complex query capabilities of the Indexer to the client is a poor security practice and creates a tight coupling. This AI Data Service acts as a dedicated, secure proxy. Its primary role is to provide a single, hardened endpoint for the client to fetch a pre-aggregated, ready-to-use "AI Context" data package, abstracting away the underlying complexity of the Indexer.
Measurable Objectives
* A functional Rust AI Data Service is implemented and running in the local Docker Compose simulation.
* The GetAgentContextForNAR endpoint successfully validates a platform JWT and enforces correct authorization.
* The service correctly interacts with the functional Rust Indexer API stubs to fetch the comprehensive AI context data.
* An optional caching/storage layer using MySQL is implemented and tested.
Implementation Guidance
Action: Implement the functional Rust AI Data Service in /services/ai-data-service/rust/src/. In Phase 1, this service acts as a specialized, secure query proxy to the Indexer. Its primary role is to fetch the comprehensive (but hardcoded default for P1) AI context data from the functional Rust Indexer API stubs and provide it in a clean, ready-to-use format for the Control Center Client to engineer prompts for the Rust NAR.
* Endpoint Implementation (gRPC, as per P0.3 API Contract /services/ai-data-service/protos/ai_data_service.proto):
o GetAgentContextForNAR(GetAgentContextForNARRequest):
* Authentication: This endpoint is critical as it exposes rich player data. It must be protected. The primary caller will be the Control Center Client (via its Rust app logic), so it will validate the player's platform session JWT.
* Implement logic to extract and rigorously validate the platform session JWT (same validation process as in the Account Service, Task 1.5).
* Extract the player_id from the JWT sub claim.
* Authorize the request: The player_id in the GetAgentContextForNARRequest must match the player_id from the JWT.
* Input Validation: Perform strict validation on the GetAgentContextForNARRequest message fields.
* Interaction with Indexer Stub:
* After successful authN/authZ, use its gRPC client (tonic) to call the functional Rust Indexer's GetAIAgentInputDataContext gRPC endpoint (from Task 1.4).
* The request to the Indexer will use the validated player_id.
* Receive the AIAgentInputDataContextProto from the Indexer stub. In P1, this will be the hardcoded, schema-valid, default AI context.
* Optional Caching/Storage (in its MySQL database):
* The service can optionally store (cache) the fetched AIAgentInputDataContextProto in its own MySQL ai_agent_data table. This acts as a snapshot for later analysis or to reduce load on the Indexer for frequently accessed contexts.
* If implementing, the MySQL table schema should be designed to store the serialized Protobuf or a JSON representation of the context, keyed by player_id.
* Use sqlx with parameterized queries for secure database interaction.
* Response: Return the (stubbed by Indexer, but fully structured according to P0.3) AIAgentInputDataContextProto response directly to the caller.
* Database (MySQL via sqlx - for optional caching):
* If optional caching is implemented, create the ai_agent_data table schema using sqlx-cli migrations.
* Ensure the service uses a secure DB connection (TLS enabled, credentials from config).
* Logging (tracing), Health Check (grpc_health_v1) (Principles K, L):
o Implement structured logging for all requests, authN/authZ outcomes, calls to the Indexer, and any database interactions.
o Implement the gRPC health check service.
* Security Review (Principle R):
o Mandatory peer review of the AI Data Service code, focusing on:
* Robustness of its API authentication (JWT validation) and authorization logic.
* Security of its interaction with the Indexer (e.g., handling errors gracefully).
* Security of its MySQL database interaction (preventing SQLi).
* Strict input validation on its own API.
* Testing (Rust):
o Unit Tests:
* Test JWT validation and authorization logic (mocking the JWT verifier).
* Test the logic for calling the Indexer gRPC client (mocking the client with mockall).
* Test the logic for caching data to the mock MySQL database.
* Test input validation for its own API request.
o Integration Tests:
* Set up a test environment with: the AI Data Service, a functional Rust Identity Service (for getting a JWT), a functional Rust Indexer (API stubs), and a test MySQL instance.
* Scenario: Fetch AI Context:
* Obtain a valid platform session JWT from the Identity Service.
* Use this JWT to call the AI Data Service's GetAgentContextForNAR endpoint.
* Verify the service successfully validates the JWT, calls the Indexer's GetAIAgentInputDataContext API stub, and returns a response that exactly matches the Indexer's full-schema default data.
* If caching is implemented, verify that the data is correctly written to the test MySQL database.
* Test failure modes (e.g., invalid JWT, Indexer unavailable) to ensure they are handled gracefully.
* Update docs/progress_logs/progress_phase_1.md:
1. Log the implementation of the AI Data Service, its gRPC endpoint, and its role as a secure proxy to the Indexer.
2. Detail the optional MySQL schema for caching.
3. Describe its secure interaction with the Indexer API stubs, emphasizing that it's fetching a rich, full-schema default context for NAR prompts.
4. Document the input validation and JWT authentication/authorization logic.
5. Log the outcome of the security code review.
6. Document adherence to First Principles H, J, K, L, R.
Design Rationale
This service acts as a dedicated Backend for Frontend (BFF) specifically for the NAR. This is a strong security pattern as it creates a narrow, purpose-built API that only exposes the data necessary for the AI context, rather than exposing the full, powerful query capabilities of the Indexer directly to the client. This reduces the attack surface and allows for independent scaling and caching of this specific, potentially high-volume, data access pattern.
Operational Considerations
* Local-First: The service will run in its own Docker container within the docker-compose.yml setup, communicating with the Indexer stub and a MySQL container via their DNS names.
* Cloud-Ready: As a stateless, containerized service, it is architected for horizontal scaling in a Kubernetes environment. In production, it will connect to a managed database like AWS RDS.
Verification & Validation Criteria
1. All unit and integration tests pass.
2. The service successfully authenticates requests, fetches the full-schema default AI context from the Indexer stub via a secure gRPC call, and returns it correctly.
3. All defined security practices are implemented and peer-reviewed.
Testing Methodologies
A combination of unit tests with mocked dependencies (mockall for gRPC clients and DB interaction) and full integration tests run against the local Docker Compose stack.
Version Control Strategy
* Branching: The service is developed on a feature/ai-data-service branch.
* Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
* Mandatory security peer review of the JWT authentication and authorization logic.
* Review of the database interaction logic to ensure no possibility of SQL injection.
* Review of the service's IAM role (for the cloud-ready IaC) to ensure it has least-privilege access to any required resources.
ReviewedBy: AI Lead, Indexer Lead, Rust Backend Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All unit and integration tests pass. The service successfully authenticates requests, fetches the full-schema default AI context from the Indexer stub, and returns it correctly. Secure practices are implemented.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.6: Implemented Rust AI Data Service (Functional Stub for Context Query via Functional Indexer API Stubs with Full Schema Defaults)." @Phase1/

------------------------------------------------------------------------------------------------------------------
