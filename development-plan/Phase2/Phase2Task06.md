Task 2.6: C++ Control Center Client - Marketplace UI
(Functional Listings from Live Indexer, Functional Basic Buy/List via L3 Smart Contracts, Rich Local NAR Narrative - Principles H, J, K, P, O, R)
Technical Reference
Finalized v0.1 API Contracts (marketplace_service.proto, indexer_service.proto)
Finalized L3 Smart Contract ABIs for Marketplace and NFT contracts.
docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md (for Marketplace screen)
CXX-Qt and QML Documentation
Context/Problem Statement
The client's Marketplace UI is currently a non-functional placeholder. With the backend Indexer now serving live data (Task 2.4) and the L3 smart contracts supporting basic economic actions (Task 2.2), we must build the client-side UI and logic to interact with these systems. This task involves implementing a functional, albeit simplified for P2, Marketplace UI. This UI will display listings fetched from the live Indexer and will support basic "List" and "Buy" actions by submitting secure transactions to the L3.
Measurable Objectives
A functional Marketplace UI is implemented, replacing the P1.10 placeholder screen.
The UI can successfully fetch and display a (sparsely populated) list of items for sale from the live Indexer.
The "List" and "Buy" functionalities are implemented, correctly submitting transactions to the L3 smart contracts.
Contextual NAR narratives are generated for successful marketplace actions.
Implementation Guidance
Action: Implement a functional (but simplified for P2) Marketplace UI section in the Control Center Client (QML UI with Rust Application Logic). This UI will display listings fetched from the now live (but sparsely populated) Rust Indexer. It will support basic "List" and "Buy" actions, which will submit transactions to the functional L3 Smart Contracts. It will also display rich, contextual narratives from the local Rust NAR for these marketplace actions, using the live P2 default item details.
Rust Application Logic (/client/rust-app-logic/src/marketplace/handler.rs - Functional for P2 Scope, Secure API Calls):
o This module, part of the rustBackend QObject, will handle all client-side logic for the marketplace.
o #[cxx_qt::qinvokable] async fn fetch_market_listings(&self, filters: &str, page: u32):
Parses and validates filters/pagination from the QML UI.
Calls the functional Rust Marketplace Service's GetMarketListingsPublic gRPC endpoint. (The Marketplace Service in P2 will query the functional Rust Indexer, which now serves live data).
Receives the list of live listings and updates its internal state, emitting a marketListingsChanged signal for the QML UI.
o #[cxx_qt::qinvokable] async fn fetch_nft_details_for_market(&self, nft_id: &str):
Validates the nft_id.
Calls the Rust Marketplace Service's GetNftDetailsPublic gRPC endpoint.
Returns the details to the QML UI for display in a modal.
o #[cxx_qt::qinvokable] async fn buy_item(&self, nft_id: &str, seller_id: &str, price_eur_cents: u64):
Validation: Perform client-side validation of inputs.
Transaction Intent: Construct a transaction call to the Marketplace.executeTrade function on the L3.
L3 Submission: Call a generic submit_l3_transaction method which securely submits the intent to the L3 sequencer.
UI Feedback & NAR Trigger: On receiving a successful transaction receipt from the L3:
Emit a signal to QML to show a "Purchase Processing..." status.
Trigger the local Rust NAR (P2.5) with the rich context of the purchased item.
o #[cxx_qt::qinvokable] async fn list_item(&self, nft_id: &str, price_eur_cents: u64):
Validation: Validate inputs.
Transaction Intent: Construct a transaction call to the Marketplace.listItemForSale function on the L3.
L3 Submission: Submit the intent to the L3 sequencer.
UI Feedback & NAR Trigger: On successful transaction receipt, update UI status and trigger local Rust NAR.
QML Frontend (/client/qml-ui/screens/MarketplaceScreen.qml & related components - Functional for P2 Scope with Full Schema Defaults):
o Replace the placeholder screen with a functional UI.
o Listings View (ListView or GridView): On Component.onCompleted, calls rustBackend.fetchMarketListings. Binds to the rustBackend.marketplaceListingsModel property to display the live list of items.
o Filtering & Sorting UI (Basic for P2): Implement basic UI controls for filtering. When filter values change, re-call rustBackend.fetchMarketListings.
o Item Details Modal: When a user clicks an item, it calls rustBackend.fetchNftDetailsForMarket. A modal dialog opens to display the full P2 default NFT details returned from the live Indexer.
o Functional "Buy" and "List Item" Buttons: The "Buy" button (in the details modal) will be functional. In the Guard UI's inventory section, a "List Item" button will be functional.
o The UI will show a "Processing..." or "Submitting to Netchain..." message while waiting for the L3 transaction to be confirmed.
o Narrative Display: The Marketplace screen will have a dedicated area to display the local Rust NAR narratives.
Testing:
Component/Unit Tests (Rust App Logic & QML): Test the Rust marketplace handler methods, mocking the Marketplace Service gRPC client and the L3 submission client. Test input validation. Use Qt Test to test the QML components.
E2E Tests (Client + Functional Rust Backend - L3 P2 submission & Indexer P2 live reads):
Start the full backend stack in the local Docker Compose simulation.
Use the Admin CLI to mint a few test NFTs to a test player by submitting transactions to the L3 Smart Contracts.
Launch the client and log in as the test player.
List Item: Navigate to the Guard inventory, select a test NFT, and use the UI to list it on the marketplace. Verify the listItem transaction is accepted by the L3 sequencer, and after a short delay, the item appears in the Marketplace listings view (fetched from the live Indexer).
Buy Item: Log in as a different test player. Navigate to the marketplace, find the listed item, and click "Buy". Verify the buyItem transaction is accepted by the L3 sequencer.
Verify State Change: After the purchase, verify that the item is removed from the marketplace listings and now appears in the buyer's Guard inventory (reflecting the live L3->Indexer data flow).
Verify NAR: For both listing and buying, verify that a contextual NAR narrative is displayed.
Update docs/progress_logs/progress_phase_2.md:
Log the implementation of the functional P2 Marketplace UI in QML and its corresponding Rust application logic handler.
Detail the interactions with the live P2 Indexer (for data) and functional P2 L3 Smart Contracts (for transactions).
Document the NAR prompts used for P2 marketplace narratives.
Detail the input validation implemented in the client's Rust handler before submitting transactions to the L3.
Document adherence to First Principles H, J, K, P, O, R.
Design Rationale
Implementing a functional, end-to-end economic loop is a major milestone for the MVE. This approach validates the entire architecture: the client UI triggers a Rust handler, which securely constructs and submits a transaction to the L3 smart contracts. The smart contracts execute the canonical logic, emit an event, which is then consumed by the Indexer, and the updated state is finally reflected back in the UI of all relevant clients. This proves the data consistency and integrity of the entire system.
Operational Considerations
The performance of the Indexer's GetMarketListings query will become increasingly important as the number of listings grows post-launch. The initial implementation will establish a baseline, and further optimization of the Indexer's database schema may be required in later phases.
Verification & Validation Criteria
All unit and E2E tests pass. A user can successfully list an item on the marketplace, and another user can buy it. The entire process is correctly reflected on-chain on the L3 (validated via the block explorer) and in the live Indexer, with the UI dynamically updating based on this live data. Contextual NAR narratives are generated correctly.
Testing Methodologies
A combination of unit/component tests for the client-side logic and a full E2E test scenario involving two client instances interacting with the full backend stack running in the local Docker Compose simulation.
Version Control Strategy
Branching: The Marketplace UI and its associated logic are developed on a feature/marketplace-ui-p2 branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security Lead must review the client-side logic for constructing L3 transactions to ensure it cannot be manipulated to create unintended or malicious transactions (e.g., buying an item for the wrong price).
Input validation in the Rust rustBackend handlers is a critical security gate.
ReviewedBy: Client Lead (Rust & QML), Rust Marketplace Service Lead, L3 Smart Contracts Lead, UI/UX Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All unit and E2E tests pass. A user can successfully list an item on the marketplace, and another user can buy it. The entire process is correctly reflected in the L3 Smart Contracts and the live Indexer.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 2.6: Implemented CC Marketplace UI (Live P2 Indexer Data with Full Schema Defaults, Functional P2 L3 List/Buy, Local NAR for P2 Items)." @Phase2/