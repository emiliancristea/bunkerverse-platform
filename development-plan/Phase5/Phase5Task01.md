Task 5.1: Rust Mission Service - Full Secure Implementation
(Rich Mission Definitions, Persistent MySQL Tracking, Secure L3 Reward Orchestration for All Reward Types - Principles H, I, J, K, L, M, N, O, R)
Technical Reference
Finalized P5 GDDs for all MVE Missions
gRPC/tonic, sqlx, and inter-service communication (e.g., NATS) crate documentation
Finalized v0.1 mission_service.proto API Contract
Finalized L3 Smart Contract ABIs (for minting/transferring rewards)
Context/Problem Statement
The MVE is a platform with rich social and economic features, but it lacks a guided progression system to drive long-term engagement. The Mission Service is the critical component that bridges this gap, providing players with clear objectives and rewarding them with on-chain assets for their accomplishments. This task involves fully implementing and hardening this service, including its persistent database for tracking progress, its logic for checking dynamic prerequisites against the live Indexer, and its secure orchestration of reward transactions on our L3.
Measurable Objectives
A fully functional and hardened Rust Mission Service with its own persistent MySQL storage is implemented and deployed to the local Docker Compose simulation.
The service accurately reflects all MVE mission designs from the GDDs.
The service correctly checks dynamic prerequisites (e.g., player's current BunkerClass) against the live Indexer.
The service securely tracks progress based on events from other services.
The service securely and correctly orchestrates the granting of all GDD-defined on-chain rewards via the L3 smart contracts.
Implementation Guidance
Action: Fully implement and harden the Rust Mission Service in /services/mission/rust/src/ with its own persistent MySQL storage. The service must accurately reflect all MVE mission designs, including checking prerequisites against the live Indexer, tracking progress, and securely orchestrating the granting of all GDD-defined L3 Smart Contract rewards.
Implementation Details:
a. Mission Definitions (/services/mission/rust/resources/missions.yaml or DB Table): Load and parse the comprehensive MVE mission definitions from the GDD into an in-memory structure at service startup.
b. Database Schema (MySQL - mission_player_progress table via sqlx): Finalize the schema: (player_id, mission_id, status, objective_progress_json, ...) using sqlx-cli for migrations.
c. API Endpoints (gRPC - /services/mission/protos/mission_service.proto - Functional & Secure):
rpc GetPlayerMissions(...):
Authenticates the player's JWT.
Fetches all mission definitions.
For each mission, it checks eligibility by querying the live Rust Indexer for the player's current BunkerClass, ClassAffiliation, etc., to validate prerequisites.
It fetches the player's current progress from its own MySQL database.
It merges this data to return a comprehensive list of missions for the player.
rpc ReportMissionProgress(...):
This endpoint is designed to be called by other backend services (or in the future, game servers) and must be protected by a robust service-to-service authentication method (e.g., mTLS).
It performs strict input validation on the player_id and the event_type.
It loads the player's active missions from MySQL that are interested in this event_type.
Crucially, before incrementing progress, it can perform a defense-in-depth check by querying the Indexer to re-verify any conditions (e.g., "was the player a 'Vanguard' when this action was reported?").
It updates the objective_progress_json in MySQL.
rpc ClaimMissionReward(...):
Authenticates the player's JWT.
Verifies in its MySQL DB that the mission status is ObjectivesComplete and not already Claimed.
It constructs and submits all necessary L3 transaction intents to the L3 sequencer (via the Transaction Submission Service) for all defined rewards: XpMinted, and NftMinted (using the item_gdd_id from the mission definition to look up the full GDD-defined NftDetailsProto to be minted).
After successfully submitting to the L3 sequencer, it updates the mission status to Claimed in its MySQL database.
d. Security Hardening: Threat model the Mission Service for reward exploits (claiming multiple times, false progress reporting). Implement server-side validation for all progress reports and claims. Ensure service-to-service authentication is robust.
Update docs/progress_logs/progress_phase_5.md:
Log the implementation of the rich mission definitions, MySQL schema, and the logic for GetPlayerMissions, ReportMissionProgress, and ClaimMissionReward.
Document the service-to-service authentication method. Detail the threat model and mitigations.
Design Rationale
A dedicated Mission Service is a critical component for creating a guided and rewarding player experience. By making it a central, stateful service, we can support complex, multi-stage missions that span different activities (forum posts, arcade games, and future gameplay). Checking prerequisites against the live Indexer ensures that missions are always offered based on the player's true, canonical on-chain state. Orchestrating rewards via L3 transactions makes the play-to-earn loop transparent, secure, and verifiable.
Operational Considerations
Local-First: The Mission Service and its MySQL database will run in the local Docker Compose simulation.
Cloud-Ready: The service is architected to be a standard, containerized microservice. In a production cloud environment, it will connect to a managed, high-availability AWS RDS instance to ensure mission progress data is never lost.
Verification & Validation Criteria
Integration tests show the service correctly checks dynamic prerequisites against the live Indexer.
The service correctly tracks progress based on mock events from other services.
The service successfully orchestrates the granting of all defined L3 reward types, including GDD-defined NFTs, while preventing common exploits like double-claiming.
Testing Methodologies
Unit Tests: For all business logic within the service, especially for prerequisite checking, progress updating, and reward transaction construction.
Integration Tests: Run the full stack in the local Docker Compose simulation. A test harness will: 1) trigger an action in a mock service (e.g., Forum Service), 2) verify the Mission Service consumes the event and updates its database, 3) call GetPlayerMissions to see the updated state, and 4) call ClaimMissionReward and verify the correct transaction is submitted to a mock L3 sequencer.
Version Control Strategy
Branching: The Mission Service will be developed on a feature/mission-service branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory, in-depth security review of the Mission Service is required before this phase can be completed.
The review must focus on preventing all reward-related exploits: claiming multiple times, claiming uncompleted missions, and spoofing mission progress.
The security of the service-to-service authentication for the ReportMissionProgress endpoint is a critical checkpoint.
ReviewedBy: Backend Lead, Game Design Lead, L3 Smart Contracts Lead, Indexer Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: Integration tests show the service correctly checks prerequisites, tracks progress, and orchestrates the granting of all defined L3 reward types while preventing common exploits.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 5.1: Implemented Secure Rust Mission Service (Rich Definitions, Persistent MySQL Tracking, Full Secure L3 Reward Orchestration)." @Phase5/