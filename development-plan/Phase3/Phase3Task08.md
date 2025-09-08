Task 3.8 (Revised): Integration Testing (Core Platform Loops)
(Full Social & Economic Loops, Secure Backend & L3 Integration - Principles O, R)
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
With the completion of the core UI navigation (3.1) and all the major functional hubs-The Plaza (3.2), Archives (3.3), Social Interface (3.4), Camp (3.6), and Arcade (3.7)-all platform components for Phase 3 have been implemented. Before proceeding to the next major development phase, we must conduct a comprehensive integration test to validate that these new, complex systems function correctly and securely as a single, cohesive unit. This is the final validation gate for all of Phase 3's development work.
3. Measurable Objectives
The complete MVE platform stack for Phase 3 is successfully launched via the local Docker Compose setup.
Key end-to-end user scenarios for all newly implemented platform features (Marketplace, Auctions, Archives, Chat, Friends, Camp, Arcade) pass successfully.
End-to-end data consistency across the L3 -> Indexer -> Client UI pipeline is validated for all new features.
A suite of basic negative security tests for the new services passes, validating the robustness of their boundaries.
4. Implementation Guidance
Action: Conduct comprehensive integration tests focusing on the end-to-end flow of all Phase 3 platform features. This includes validating the entire pipeline from user actions in the Control Center Client, through the new Rust backend services, to the L3 Smart Contracts, and back through the Rust Indexer to the UI.
Test Environment Setup (Full Local Docker Compose):
Run the full MVE platform stack via docker-compose up. This now includes the functional Rust Identity, Account, AI Data, Marketplace, Forum, Social, Chat, Events, and Arcade services, the L3 Smart Contracts deployed to a local Arbitrum Orbit node, and the live Indexer.
Key E2E Scenarios (Manual QA with detailed scripts):
Verify a user can list an item for a direct sale, and another can buy it.
Verify a user can list an item for auction, another can bid, and the auction is correctly settled on the L3 after its timer expires.
As Player A, navigate to the CAMP and accept a mission to "Create a post in the Archives."
Navigate to the ARCHIVES, create a new thread and post.
Verify the Mission Service detects the completion via the inter-service message bus.
Navigate back to the CAMP and verify the mission is now completable.
Claim the reward and verify the NTC/XP transaction on the L3 block explorer.
As Player A, accept a mission to "Add a new friend."
Navigate to the SOCIAL interface and successfully send a friend request to Player B.
As Player B, accept the request. Verify the on-chain SocialConnectionAdded event.
As Player A, verify the mission is now complete and claim the reward.
i. As Player A, accept a challenge in the ARCADE.
ii. Play the required HTML5 game and achieve the winning score.
iii. Verify the Arcade Service securely validates the score and triggers the on-chain NTC/XP reward transaction.
Security Negative Tests (Basic):
XSS Validation: Attempt to create a forum post and send a chat message containing a malicious script tag (<script>alert('xss')</script>). Verify the content is properly sanitized by the backend and rendered harmlessly in the client UI.
Auction Snipe: Attempt to place a bid on an auction via the gRPC API just as the auction is closing to test for race conditions.
5. Design Rationale
This comprehensive integration test is the ultimate validation gate for the platform-centric phases. It ensures that all the complex, interconnected social and economic systems work together as a single, cohesive product before we enter the next major stage of development (Dynamic Progression in Phase 4).
6. Operational Considerations
The full Docker Compose environment validated in this task will now be the stable baseline for the entire project moving forward. The E2E test scripts will become part of the core regression suite.
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
The Security Lead must review and sign off on the results of the negative security tests, particularly the validation of XSS sanitization in the Archives and Chat services and the auction settlement security.
ReviewedBy: QA Lead, All Tech Leads, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All P3 E2E tests pass. The full data flow and functional loops for the core platform features are successfully demonstrated.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 3.8: Integration Tests for Core Platform Loops (Marketplace, Archives, Social, Camp, Arcade)." @Phase3/