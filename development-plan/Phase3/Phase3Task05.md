Task 3.5: Integration Testing (Core Platform Loops)
(Full Social & Economic Loops, Secure Backend & L3 Integration - Principles O, R)
Technical Reference
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
Context/Problem Statement
With the completion of the core UI navigation (3.1), the Plaza (3.2), the Archives (3.3), and the Social Interface (3.4), all the major platform components for Phase 3 have been implemented. Before proceeding to the next phase, we must conduct a comprehensive integration test to validate that these new, complex systems function correctly as a single, cohesive unit. This is the final validation gate for all of Phase 3's development work.
Measurable Objectives
The complete MVE platform stack for Phase 3 is successfully launched via the local Docker Compose setup.
Key end-to-end user scenarios for the newly implemented platform features (Marketplace, Archives, Social) pass successfully.
End-to-end data consistency across the L3 -> Indexer -> Client UI pipeline is validated for all new features.
A suite of basic negative security tests for the new services passes, validating the robustness of their boundaries.
Implementation Guidance
Action: Conduct comprehensive integration tests focusing on the end-to-end flow of all Phase 3 platform features. This includes validating the entire pipeline from user actions in the Control Center Client, through the new Rust backend services, to the L3 Smart Contracts, and back through the Rust Indexer to the UI.
Test Environment Setup (Full Local Docker Compose):
Run the full MVE platform stack via docker-compose up. This now includes the functional Rust Identity, Account, AI Data, Marketplace, Forum, Social, and Chat services, the L3 Smart Contracts deployed to a local Arbitrum Orbit node, and the live Indexer.
Key E2E Scenarios (Manual QA with detailed scripts):
Use the Admin CLI to mint a test NFT to Player A's account on the L3.
As Player A, log into the client, navigate to THE GUARD -> INVENTORY, and list the NFT for sale on the unified Marketplace. Verify the listItem transaction on the block explorer.
Log in as Player B. Navigate to THE PLAZA -> MARKETPLACE. Verify the listing from Player A appears correctly, with the 15% fee displayed.
As Player B, buy the item. Verify the executeTrade transaction on the block explorer.
Verify the item is removed from the marketplace and now appears in Player B's inventory, and that Player A's inventory is updated.
As Player A, navigate to THE NEXUS -> ARCHIVES. Create a new thread and a new post.
Log in as Player B, navigate to the Archives, and verify Player A's post is visible. Reply to the post.
As an administrator using the Admin CLI, execute a RewardPost command targeting Player A's post.
Verify on the block explorer that a transaction occurred, transferring NTC to Player A. Verify Player A's NTC balance updates in the client UI.
Log in as Player A and Player B on two different client instances.
As Player A, send a friend request to Player B via the SOCIAL interface.
As Player B, accept the friend request. Verify the SocialConnectionAdded transaction is sent to the L3.
Verify both players' friend lists update.
Successfully send and receive real-time text messages between Player A and Player B.
Security Negative Tests (Basic):
Forum Input Validation: Attempt to create a forum post with a malicious script tag (<script>alert('xss')</script>). Verify that the content is properly sanitized by the backend and rendered harmlessly in the client UI.
Chat Input Validation: Perform the same XSS test in the real-time chat.
Service Authentication: Use the Admin CLI to attempt to call a protected gRPC endpoint (e.g., AccountService.GetPlayerProfile) without a valid JWT. Verify it returns UNAUTHENTICATED.
Design Rationale
This comprehensive integration test is the ultimate validation gate for the platform-centric phases. It ensures that all the complex, interconnected social and economic systems work together as a single, cohesive product before we enter the next major stage of development (Dynamic Progression in Phase 4).
Operational Considerations
The full Docker Compose environment validated in this task will now be the stable baseline for the entire project moving forward. The E2E test scripts will become part of the core regression suite.
Verification & Validation Criteria
All P3 E2E tests pass, including basic security negative tests.
The full data flow and functional loops for the MVE's core platform features (Marketplace, Archives, Social) are successfully demonstrated.
The system is proven to be functional, secure, and data-consistent across the entire platform stack.
Testing Methodologies
A combination of scripted manual QA for the full E2E user scenarios and automated integration tests where possible (e.g., for the backend service interactions).
Version Control Strategy
Branching: The integration test suite and any necessary fixes are developed on a feature/p3-integration-tests branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security Lead must review and sign off on the results of the negative security tests, particularly the validation of XSS sanitization in the Archives and Chat services.
ReviewedBy: QA Lead, All Tech Leads, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All P3 E2E tests pass. The full data flow and functional loops for the core platform features are successfully demonstrated.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 3.5: Integration Tests for Core Platform Loops (Marketplace, Archives, Social)." @Phase3/