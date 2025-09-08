Task 3.10 (Previously 3.5): Integration Testing (Core Platform Loops)
(Full Social, Economic & Launcher Loops with Secure Backend & L3 Integration - Principles O, R)
1. Technical Reference
All Phase 3 Test Plans
docker-compose.yml for the full P3 Platform stack
bunkerverse-admin-cli tool
The live L3 Testnet block explorer

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
2. Context/Problem Statement
With the completion of the core UI navigation (3.1) and all the major functional hubs-The Plaza (3.2), Archives (3.3), Social Interface (3.4), Camp (3.6), Arcade (3.7), and the Constructs launcher (3.9)-all platform components for the MVE have now been implemented. Before proceeding to Phase 4 (Dynamic Progression), we must conduct a comprehensive integration test to validate that these new, complex systems function correctly and securely as a single, cohesive unit. This is the final validation gate for all of Phase 3's development work.
3. Measurable Objectives
The complete MVE platform stack for Phase 3 is successfully launched via the local Docker Compose setup.
Key end-to-end user scenarios for all implemented platform features (Marketplace, Archives, Social, Camp, Arcade, Constructs) pass successfully.
End-to-end data consistency across the L3 -> Indexer -> Client UI pipeline is validated for all new features.
A suite of basic negative security tests for the new services passes, validating the robustness of their boundaries.
4. Implementation Guidance
Action: Conduct comprehensive integration tests focusing on the end-to-end flow of all Phase 3 platform features. This includes validating the entire pipeline from user actions in the Control Center Client, through the new Rust backend services, to the L3 Smart Contracts, and back through the Rust Indexer to the UI.
Test Environment Setup (Full Local Docker Compose):
o Run the full MVE platform stack via docker-compose up. This now includes the functional Rust Identity, Account, AI Data, Marketplace, Forum, Social, Chat, Events, Arcade, and Matchmaking services, the L3 Smart Contracts deployed to a local Arbitrum Orbit node, and the live Indexer.
Key E2E Scenarios (Manual QA with detailed scripts):
o ##### Full Marketplace & Auction Cycle:
Verify a user can list an item for a direct sale, and another can buy it, with the 15% fee correctly handled on-chain.
Verify a user can list an item for auction, another can bid via the service, and the auction is correctly settled on the L3.
o ##### Full Archives & Mission Cycle:
Verify a user can accept an in-verse mission from the CAMP, complete the objective by posting in the ARCHIVES, and claim the on-chain NTC/XP reward.
o ##### Full Social & Mission Cycle:
Verify two users can become friends on-chain via the SOCIAL interface and communicate via real-time text chat.
Verify a mission to "Add a friend" can be completed and the reward claimed.
o ##### Full Arcade & Mission Cycle:
Verify a user can accept a challenge in the CAMP, complete it by playing an HTML5 game in the ARCADE, and securely receive an on-chain NTC/XP reward.
o ##### Full Constructs Launcher Cycle:
i. As a Player or Founder, navigate to THE CONSTRUCTS.
ii. Verify all 9 constructs are displayed with their correct previews and live queue data from the Matchmaking Service.
iii. Click "Launch" on a construct. Verify the UI shows the "Finding Match..." status.
iv. Verify the Matchmaking Service successfully allocates a stubbed game server from Agones running in the local simulation.
v. Verify the client UI updates to "Match Found!" and the Rust backend logs the received connection info.
Security Negative Tests (Basic):
o XSS Validation: Attempt to create a forum post and send a chat message with a malicious script tag (<script>alert('xss')</script>). Verify the content is properly sanitized.
o Service Authentication: Use the Admin CLI to attempt to call a protected gRPC endpoint (e.g., AccountService.GetPlayerProfile) without a valid JWT. Verify it returns UNAUTHENTICATED.
o Matchmaking Spam: Attempt to queue for a construct repeatedly in a short period to test rate limiting on the Matchmaking Service.
5. Design Rationale
This comprehensive integration test is the ultimate validation gate for the platform-centric phases. It ensures that all the complex, interconnected social, economic, and game-launch systems work together as a single, cohesive product before we enter the next major stage of development (Dynamic Progression in Phase 4).
6. Operational Considerations
The full Docker Compose environment validated in this task will now be the stable baseline for the entire project moving forward. The E2E test scripts created here will form the core of the project's regression suite.
7. Verification & Validation Criteria
All P3 E2E tests pass, including basic security negative tests.
The full data flow and functional loops for the MVE's core platform features are successfully demonstrated.
The system is proven to be functional, secure, and data-consistent across the entire platform stack.
8. Testing Methodologies
A combination of scripted manual QA for the full E2E user scenarios and automated integration tests where possible (e.g., for the backend service interactions).
9. Version Control Strategy
Branching: The integration test suite and any necessary fixes are developed on a feature/p3-integration-tests branch.
Commits: The Git Commit message for this task will be exactly as specified.
10. Security Audit & Compliance Checkpoints
The Security Lead must review and sign off on the results of the negative security tests, particularly the validation of XSS sanitization in the Archives and Chat services, and the security of the matchmaking flow.
ReviewedBy: QA Lead, All Tech Leads, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All P3 E2E tests pass. The full data flow and functional loops for the core platform features are successfully demonstrated.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 3.10: Integration Tests for Core Platform Loops (Marketplace, Archives, Social, Camp, Arcade, Constructs)." @Phase3/

Phase 3 Deliverable
(Security-Hardened Process-Hardened Foundation with a Feature-Complete MVE Platform - Incorporating V2.4 BUNKERVERSE Overview)
Technical Reference
All Phase 3 Test Plans and E2E Test Results (from Task 3.10)
docs/progress_logs/progress_phase_3.md (as the master audit trail for this deliverable)
Context/Problem Statement
Phase 3 represented the largest single expansion of the MVE's scope, transitioning the project from a foundational set of services into a feature-complete, user-facing platform. This deliverable serves as the formal summary and acceptance checklist for this entire phase of work, confirming that the project has successfully built all the core social, economic, and pre-game hubs as defined in the "Imperial MVE" plan.
Measurable Objectives
All 6 Acceptance Criteria for the Phase 3 deliverables listed below are met and have been formally signed off on by the designated leads.
The progress_logs/progress_phase_3.md log is complete, audited, and provides a full audit trail for the entire phase.
A formal Phase 3 Closure Review Meeting has been successfully conducted, resulting in a "Go" decision to proceed to Phase 4.
Implementation Guidance
Upon completion of Phase 3, the project has a security-hardened and functional core platform with integrated social and economic hubs. The Control Center client is no longer a simple shell but a feature-rich application ready to onboard a community. All development has adhered to the established First Principles and is meticulously documented.
1. Fully Functional Control Center UI Framework & Navigation:
Description: The Control Center Client has a complete and functional UI navigation structure. This includes the main "VERSE" interface with its four primary tabs (THE NEXUS, THE PLAZA, THE CONSTRUCTS, THE GUARD) and the right-click logo menu for switching to the SOCIAL interface. The STREAMING and HARDWARE interfaces exist as locked placeholders.
Acceptance Criteria: All navigational elements are implemented and functional. A user can navigate to all primary sections of the application, which are no longer placeholders but functional hubs. The UI structure is stable and aligns with the signed-off UI Definition document.
2. Live & Functional Economic Hub (THE PLAZA):
Description: THE PLAZA tab is fully functional. This includes a unified MARKETPLACE UI that can display both primary BUNKER CORP and P2P secondary market listings, complete with functional filtering. The UI correctly displays the 15% fee on P2P items. The platform also has a functional off-chain auction system with secure on-chain L3 settlement. The EXCHANGE sub-tab exists as a polished but locked placeholder.
Acceptance Criteria: The full P2P marketplace and auction lifecycles are functional and validated by E2E tests. All transactions are correctly processed by the L3 Smart Contracts and the state change is reflected live in the Indexer and UI.
3. Live & Functional Community Hub (THE NEXUS):
Description: THE NEXUS tab is fully functional. The ARCHIVES (forum) is live, supported by a dedicated Rust service, and includes an L3-based reward mechanism. The CAMP is live, supported by its own service for managing events and displaying leaderboards, and fully integrates the Mission System for in-verse activities. The ARCADE is live, supported by its own service, and allows users to securely play curated HTML5 games and earn on-chain rewards.
Acceptance Criteria: A user can create posts in the Archives, view events in the Camp, accept and complete in-verse missions for L3 rewards, and play games in the Arcade for L3 rewards. The data flow for these actions is proven to be consistent and secure.
4. Functional Real-Time Social Hub (SOCIAL Interface):
Description: The "SOCIAL" interface is fully functional for all foundational real-time communication. This includes a friends list system based on on-chain data, and a real-time text chat system supported by a dedicated WebSocket-based Rust service.
Acceptance Criteria: A user can manage their on-chain friends list. Two users can successfully engage in real-time text chat with each other within the client. The entire flow is validated by E2E tests.
5. Functional Game Hub & Launcher (THE CONSTRUCTS):
Description: THE CONSTRUCTS tab is a fully functional game launcher. It displays all 9 planned constructs across their respective sub-tabs (SITES, ISLANDS, CITADELS) with high-quality previews. The UI is fully integrated with the Rust Matchmaking service and can display live queue data. The launch flow is functional up to the point of server allocation via Agones.
Acceptance Criteria: A user can browse all constructs, queue for matchmaking, and have a stubbed game server instance successfully allocated for them. The client correctly receives the connection information.
6. Validated End-to-End Platform Loops & Security:
Description: Comprehensive integration tests (Task 3.10) have validated the complete, live data flow for all Phase 3 platform features. A user action in the client correctly triggers interactions with the appropriate backend services, which in turn results in transactions on the L3, which are consumed by the Indexer, and reflected back in the client's UI.
Acceptance Criteria: All P3 integration tests, including basic security negative tests (e.g., XSS validation for forum/chat, auction settlement security), pass successfully. The core platform loops are functional, secure, and data-consistent across the entire platform stack.
In summary, Phase 3 transforms the MVE from a basic login-and-view application into a living, interactive platform. The client now possesses its final navigational structure and the core social and economic hubs that will define the user experience. The system is robustly prepared for the next major development phase: implementing the full dynamic BUNKERGUARD progression and on-chain logic.