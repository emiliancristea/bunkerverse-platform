Task 4.9: C++ Control Center Client - Marketplace UI Implementation
(Full Functionality with L3/Indexer, Stripe, All Item Details & Traits, Secure Transactions - Principles H, I, J, K, O, P, R)
Technical Reference
CXX-Qt, QML (including WebView module), Stripe.js Documentation
Finalized P4 API Contracts (marketplace_service.proto, payment_service.proto, indexer_service.proto)
Finalized L3 Smart Contract ABIs
docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
In Phase 3, we built a basic marketplace UI. Now, with the full dynamic item attributes being indexed (Task 4.6) and the complete on-chain marketplace/auction logic in place (Task 4.4), the client UI must be significantly enhanced to match these new capabilities. This task involves implementing the full, secure, and feature-rich marketplace and monetization functionality in the client, including advanced filtering and the secure Stripe integration for purchasing Credits.
Measurable Objectives
The Marketplace UI is fully implemented with comprehensive filtering and sorting on all GDD-defined item attributes.
All UI flows (List, Buy, Auction, Purchase Credits) are fully functional, secure, and provide a polished user experience.
The Stripe credit card payment flow is securely embedded in the client.
Rich, contextual NAR narratives are generated for all marketplace actions.
Implementation Guidance
Action: Implement the full, secure marketplace functionality in the client, integrating with the now fully functional Rust backend services (Marketplace, L3 Smart Contracts, Indexer, Payment). The UI must handle all item details and traits for filtering and display.
1. Implementation Details:
o ##### Rust Application Logic (/client/rust-app-logic/src/plaza/marketplace_handler.rs):
i. Finalize all methods for marketplace actions, calling the functional Rust services with strict input validation.
ii. buyItem: Submits the ExecuteTrade transaction to the L3 Smart Contracts.
iii. buyCreditsInitiate: Calls the functional Rust Payment Service to create a Stripe PaymentIntent. Securely handles the client_secret returned, passing it directly to the QML/web layer for Stripe.js to use, without storing it long-term.
o ##### QML Frontend (/client/qml-ui/screens/plaza/MarketplacePage.qml):
Browse & View: Implement full filtering and sorting on all indexed NftDetailsProto attributes (type, rarity, specific stat ranges, class affinities, trait, origin, condition, price). The UI will feel rich and responsive, querying the optimized market_listings_active_index from the Indexer.
List/Buy/Auction/Purchase: All UI flows are now fully functional. The "Purchase Credits" flow will embed a secure Stripe.js element (in a small, dedicated WebView) to handle credit card input, which uses the client_secret from the Rust app logic.
o Security: Ensure the client's Rust app logic validates all marketplace parameters from the QML UI before calling backend services. The secure handling of the Stripe client_secret is critical.
2. Testing:
o E2E Tests (Critical - Staging with full Rust backend & Rust NAR in the local Docker Compose simulation):
Perform full marketplace lifecycle tests for items of all types, rarities, and traits.
Test all filters and sorting options.
Test the ExecuteTrade transaction on the L3 via P2P sales.
Test Corp sales and Class Pack sales.
Test the MVE-scoped auction flow from bidding (via Marketplace Service) to settlement (via L3 Smart Contracts).
Test the full Credit purchase flow using Stripe's test mode and verify the Credits balance update on the L3 via the Indexer.
Verify rich NAR narratives are generated for all actions, mentioning full item details.
3. Update docs/progress_logs/progress_phase_4.md:
o Log the full Marketplace UI implementation.
o Document the secure Stripe Elements integration.
o Detail NAR prompts for marketplace actions.
Design Rationale
A rich, performant, and feature-complete marketplace is the engine of the in-verse economy. Implementing advanced filtering directly against a highly optimized Indexer is the only way to provide a responsive user experience at scale. Using Stripe.js Elements in a dedicated WebView is the industry-standard, PCI-compliant method for securely handling credit card payments in a desktop application, as it ensures that sensitive cardholder data never touches the client application or our backend servers.
Operational Considerations
Local-First: The Stripe integration will be tested locally using the Stripe CLI to forward test webhooks to the Payment Service running in Docker Compose.
Cloud-Ready: The client's WebView will need to be configured with the production URL for the Stripe.js library. The Payment Service will be configured with live Stripe API keys loaded from AWS Secrets Manager.
Verification & Validation Criteria
E2E tests pass for all marketplace and credit purchase flows.
All interactions are functional, secure, reflect live data from the Indexer and L3, and are accompanied by rich NAR narratives.
Testing Methodologies
Extensive, scripted E2E tests are the primary validation method for this task, as they are the only way to confirm the correct interaction between the client UI, the multiple backend services, the Stripe API, and the L3 smart contracts.
Version Control Strategy
Branching: The full Marketplace UI will be developed on a feature/plaza-full-ui branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory security review of the Stripe integration is required. This includes the handling of the client_secret and the configuration of the WebView to prevent any potential for XSS or data leakage.
The final, user-facing transaction confirmation dialogs in the UI must be reviewed to ensure they clearly and accurately display all costs, including the 15% P2P fee, to prevent user confusion.
ReviewedBy: Client Lead, UI/UX Lead, Rust Marketplace/Payment/L3 Leads, Security Lead (mandatory for payment and trade logic).
ReviewOutcome: Approved.
ValidationMethod: E2E tests pass for all marketplace and credit purchase flows.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 4.9: Implemented CC Marketplace UI (Full Secure Functionality - All Item Details/Traits, L3/Indexer, Stripe, Rich NAR Narratives)." @Phase4/