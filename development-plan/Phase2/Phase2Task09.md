Task 2.9: Integration Testing (Core MVE Loops)
(Full Schema P2 Defaults & Live Data Flow from L3 to Indexer to Client UI & Richer NAR for P2 - Security Considerations - Principles O, R)
Technical Reference
All Phase 2 Test Plans
docker-compose.yml for the full P2 MVE stack
bunkerverse-admin-cli tool (from Task 2.8)
The live L3 Testnet block explorer
Context/Problem Statement
All individual components for Phase 2 have been implemented and unit-tested. The L3 smart contracts contain canonical logic, and the Indexer serves live data. However, the true functionality, data consistency, and security of the system can only be validated by testing the entire, interconnected ecosystem as a whole. This task focuses on conducting comprehensive integration tests on the end-to-end flow of all P2 canonical data, from on-chain event creation through the Indexer to the client UI, including the richer NAR responses.
Measurable Objectives
The complete MVE stack for Phase 2 is successfully launched via the local Docker Compose setup.
Key end-to-end user scenarios (BUNKERGUARD Initial Setup, Marketplace Cycle) pass successfully.
End-to-end data consistency across the L3 -> Indexer -> Client UI pipeline is validated.
A suite of basic negative security tests passes, validating the robustness of the system's boundaries.
Implementation Guidance
Action: Conduct comprehensive integration tests focusing on the end-to-end flow of all P2 detailed canonical data. This includes P2 default full NFT attributes and P2 default robot states. The tests will validate the entire pipeline from L3 Smart Contract event creation through the Rust Indexer to the C++ Control Center Client UI, including the more contextual Rust NAR responses that now reflect this live P2 default data. Basic negative security tests will be included to ensure robustness.
Test Environment Setup (Full Local Docker Compose):
o Run the full MVE stack via docker-compose up. This includes the functional Rust Identity service, L3 Smart Contracts deployed to a local Arbitrum Orbit node, Indexer (with P2.3/P2.4 live ingestion/querying and Elasticsearch/Typesense), all Platform Services (Account, Marketplace, Social, AI Data Service - all interacting with the functional Indexer using P2 full schema defaults), and the compiled C++ Control Center Client (configured to connect to the Docker Compose services).
Key E2E Scenarios (Automated where possible, otherwise Manual QA with detailed scripts):
BUNKERGUARD Initial Setup & Display (P2 Defaults from Live Indexer):
User completes zkLogin.
An on-chain UserRegistered event is created by the L3 Smart Contracts.
The Indexer ingests this live event.
The Client's HOMESCREEN queries the live Indexer (via Account Svc) and displays the correct default stats (all 12 at 10), "Unclassed" class, and "Neutral" affiliation.
Marketplace Cycle (NFT with P2 Default Full Details from Live Indexer):
Use the Admin CLI to mint an NFT (e.g., "Standard Explorer's Compass" Perk) by submitting a transaction to the L3 Smart Contracts.
As Player A, use the client to list this NFT on the marketplace.
As Player B, use the client to browse the marketplace. Verify the listing appears with its correct P2 default details read from the live Indexer.
As Player B, buy the item.
Verify the L3 Smart Contracts process the TradeExecuted transaction.
Verify the Indexer updates ownership and the item is removed from marketplace listings.
Verify the item now appears in Player B's inventory in the Guard UI.
Verify both Player A and Player B receive contextual NAR narratives.
Security Negative Tests (Basic):
o Client Input Validation: Attempt to call the Marketplace buyItem method in the client's Rust backend handler with an invalid nft_id format. Verify the Rust handler rejects it with an error.
o FFI Robustness: Attempt to call the generate_local_narrative method with an overly long prompt string. Verify the Rust NAR FFI or wrapper handles it gracefully and does not crash the client.
o Smart Contract Validation: Use the Admin CLI to try and submit a transaction to mint a malformed NFT (e.g., with an invalid ItemRarityProto enum value). Verify the L3 Smart Contract's validation logic rejects it.
Focus of Validation:
o End-to-end data consistency for P2 default full schema attributes across the live L3 Smart Contracts -> live Indexer -> Client UI pipeline.
o Correctness of the L3 Smart Contracts' default state initialization logic and the Indexer's live reflection of this state.
o Contextual accuracy of Rust NAR narratives.
o Basic validation of input sanitization and error handling at the Client, Service, L3 Smart Contract, and FFI boundaries.
Update docs/progress_logs/progress_phase_2.md:
a. Document the E2E test scenarios for the P2 default full schema BUNKERGUARD and Marketplace loops.
b. Log the outcomes, including validation of data consistency and examples of the richer, live-data-driven NAR narratives.
c. Detail the basic security negative test cases that were executed and their outcomes.
Design Rationale
This final, comprehensive integration test is the ultimate validation gate for Phase 2. It moves beyond testing individual components and validates the behavior of the entire, interconnected system as it will run in the local development environment. Passing these tests provides high confidence that the core logic of the universe is not just written, but is alive, functional, and consistent across the entire stack.
Operational Considerations
The Docker Compose environment and the associated E2E test suites created in this task will become a cornerstone of our regression testing strategy. They will be run in CI on every major change to ensure that new features do not break the core economic and social loops.
Verification & Validation Criteria
All P2 E2E tests pass, including basic security negative tests.
The full data flow and functional loops for P2's default detailed MVE state (sourced live from the Indexer) and the corresponding NAR responses are successfully demonstrated.
Testing Methodologies
A combination of automated E2E tests for the backend flows and scripted manual QA for the full client-to-backend scenarios, all orchestrated within the local Docker Compose environment.
Version Control Strategy
Branching: The integration test suite is developed on a feature/p2-integration-tests branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security Lead must review the results of the negative security tests to confirm that the system's boundaries (Client, Service, L3, FFI) are behaving as expected under adversarial conditions. This is the final security sign-off for Phase 2's implementation.
ReviewedBy: QA Lead, All Tech Leads, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All P2 E2E tests pass. The full data flow and functional loops for P2's default detailed MVE state are successfully demonstrated.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 2.9: Integration Tests for Core MVE Loops (Live Full Schema P2 Defaults from L3->Indexer->Client & Richer NAR, Basic Security Tests)." @Phase2/
Phase 2 Deliverable
(Security-Hardened Process-Hardened Foundation with Functional Core MVE Canonical Logic, Full Schema Defaults Live from Indexer, and Basic Contextual Secure AI - Incorporating V2.4 BUNKERVERSE Overview)
Description
Upon completion of Phase 2, the project has transitioned from a foundation of stubs and PoCs to a system with a live, logical core. The MVE now possesses a functional, secure, and data-consistent backend where the canonical rules of the universe are enforced by the L3 Smart Contracts and accurately reflected by the Indexer in near real-time. This deliverable is a major milestone, demonstrating the viability of the entire architecture for handling rich, lore-driven mechanics.
1. Functional L3 Smart Contracts with Core Canonical Logic
Description: The L3 Smart Contracts on the Arbitrum Orbit testnet implement the core state machine and transaction execution logic for the MVE. They correctly apply all P2-scope events to mutate player state, and the transaction execution logic is functional and secure, correctly validating stateful intents and generating the appropriate canonical events.
Key Implemented Logic:
o NFTs: The smart contracts can mint all 10 ItemType NFTs. Critically, this is done by looking up a GDD-defined ID from an on-chain registry and creating the NFT with its full, canonical attributes. Basic NFT transfers are handled correctly.
o BUNKERGUARD Robots: The smart contracts correctly process RobotLinked events, which atomically trigger a sequence of subsequent events (RobotStatsUpdated, ClassAssigned, AffiliationAssigned) to initialize the player's robot with the full, default state. Basic robot leveling (XP burn for level increment) is functional.
o Tokens: XP balance updates are handled correctly using checked arithmetic to prevent overflows/underflows.
o Social: Basic social connection events are recorded on-chain.
Acceptance Criteria: The L3 smart contracts correctly process all defined P2 transaction intents submitted via their public functions, enforce canonical rules securely, and emit the resulting events. The on-chain state can be deterministically re-derived from these events. This is validated by extensive integration tests and the Admin CLI's state-derive command.
2. Live & Functional Rust Decentralized Global Indexing Layer
Description: The Rust Indexer is now a live data-processing pipeline, no longer relying on hardcoded stubs. It has a functional and secure ingestion pipeline that reliably consumes the stream of detailed MVE events from the L3 Smart Contracts (via a secure RPC connection). It correctly transforms these events and updates comprehensive indexes for player accounts, NFTs, and basic transaction/social logs.
Key Implemented Capabilities:
Live Player State: The Player Index contains live, up-to-date information reflecting on-chain events, including player balances and the full ActiveBunkerguardDataProto.
Live NFT State: The NFT Index contains documents for each minted NFT, storing their full, GDD-defined NftDetailsProto and their current ownership.
Live Queries: The Indexer's gRPC API serves queries using this live data. API endpoints have strict input validation and placeholder authorization logic.
Acceptance Criteria: The Indexer successfully consumes and indexes all P2 L3 smart contract events. Its API provides accurate, near real-time query results. The data consistency between the L3 Smart Contracts (validated by state-derive) and the Indexer (validated by indexer query) is proven through testing.
3. Enhanced & Contextual Rust Netchain AI Runtime (NAR)
Description: The Rust NAR now performs functional contextual inference. The client-side prompt engineering is enhanced to use the detailed live P2 context fetched from the live Rust Indexer (via the functional Rust AI Data Service).
Key Implemented Capabilities:
o Generates more specific and relevant narrative text for core P2 events like NFT minting, robot linking, and basic marketplace/social actions.
o Includes basic resource limiting (timeout) and continues to enforce hardened FFI security.
Acceptance Criteria: The NAR generates narratives that correctly incorporate specific details from live P2 events. The integration is stable and secure, validated by qualitative review and testing.
4. Functional Client UI for Core Loops
Description: The Control Center Client has functional UI sections for the Marketplace and Social features, replacing the placeholders.
Key Implemented Capabilities:
o Marketplace UI: Allows users to browse NFT listings (with P2 default details from the functional Indexer) and initiate functional "List" and "Buy" actions. These actions correctly submit transactions to the L3 sequencer and trigger local Rust NAR-generated narratives.
o Social UI: Allows users to view basic social connection data (from the functional Indexer) and initiate a functional "add connection" action, which submits a transaction to the L3 sequencer and triggers a local Rust NAR narrative.
Acceptance Criteria: A user can perform basic list/buy and social-add actions via the client UI. These actions are correctly processed by the L3 Smart Contracts, the state change is reflected in the Indexer, and the UI dynamically updates based on this live data.
5. Validated End-to-End Data Flow & Security for P2 Scope
Description: Comprehensive integration tests validate the complete, live data flow for all P2 features. A user action in the client triggers a transaction to the L3 Smart Contracts, the contracts apply canonical rules and emit an event, the Indexer consumes the event, and the client's UI is updated with the new state queried from the Indexer.
Acceptance Criteria: All P2 integration tests, including basic security negative tests, pass successfully. The core MVE loops for P2 (basic economy and social interaction) are functional, secure, and data-consistent across the entire stack.
6. Complete Audit Trail & Process Adherence
Description: The docs/progress_logs/progress_phase_2.md log provides a complete, detailed, and reviewed audit trail for this phase, including all security considerations, canonical rule implementations, and test outcomes.
Acceptance Criteria: The progress log is complete and signed off, demonstrating adherence to all First Principles and established development processes.
In summary, Phase 2 transforms the MVE from a collection of well-designed stubs into a living system with an active, rule-enforcing on-chain heart (the L3 Smart Contracts). The platform now has its core canonical logic for player identity, assets, and basic personalization functionally implemented in the L3 Smart Contracts with security hardening. This detailed state is accurately reflected live in the Rust Indexer. Basic economic and social loops are functional, with more deeply integrated local AI narrative generation using live (though default) context. The system is robustly prepared for the full integration of the game MVPs and their dynamic, gameplay-driven state changes in Phase 3.