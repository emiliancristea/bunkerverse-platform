Task 4.6: Global Indexing Layer (Rust) - Full Secure Marketplace & Dynamic Robot State Indexing
(All Item Attributes, All Classes, Affiliations, All Stats - Principles G, J, K, N, O, R)
Technical Reference
Elasticsearch/Typesense Documentation (for advanced query and index mapping)
Finalized v0.1 indexer_service.proto and L3 Smart Contract event schemas
P4 GDDs (for understanding the new data structures)

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
The Indexer, as of Phase 3, can only process basic events. With the L3 smart contracts now handling the full dynamic Bunkerguard state (Task 4.3) and comprehensive marketplace logic (Task 4.4), the Indexer is effectively blind to these critical state changes. This task is to enhance the Rust Indexer to fully and securely ingest all new dynamic events, build comprehensive new indices for the live market and player state, and provide performant, secure query capabilities for this rich data. This is the final step in making the full depth of the on-chain economy and progression system visible to the client.
Measurable Objectives
The Indexer's ingestion pipeline is enhanced to securely consume and transform all new P4 L3 smart contract events.
The player_index is updated to store the full, live, dynamic Bunkerguard state.
New, optimized indices for active marketplace listings and transaction history are created and populated.
The Indexer's gRPC API is enhanced with new RPCs and advanced filtering capabilities to serve this rich data.
Integration tests demonstrate the Indexer's state is accurate after complex on-chain event sequences.
Implementation Guidance
Action: Enhance the Rust Indexer to fully and securely ingest all new L3 smart contract events from P4 (dynamic BUNKERGUARD state changes, full marketplace activity). Provide comprehensive, secure, and performant query capabilities for this live, rich data.
Implementation Details:
Consume all new/updated L3 smart contract events: ItemEquipped, ItemUnequipped, RobotStatsUpdated, ClassAssigned, AffiliationAssigned, NftMarketStatusUpdated, TradeExecutedRecordAdded, AuctionSettled.
Player Account Index Update: Update the transformer to store the player's full, live ActiveBunkerguardDataProto as it changes. This includes:
current_class: The actual BunkerClassProto from the ClassAssigned event.
current_affiliation: The actual ClassAffiliationProto from the AffiliationAssigned event.
final_stats: The full CoreStatsProto with all 12 live sub-stats from the RobotStatsUpdated event.
equipped_items: A detailed list of equipped items, caching key details for quick UI display.
NFT Index Update: Update the transformer for NftMarketStatusUpdated and TradeExecutedRecordAdded to keep the current_owner_id and market_status fields of the NFT document perfectly in sync with the canonical L3 smart contract state.
New Marketplace Listings Index: Create a new, dedicated index for active listings (market_listings_active_index). This index is optimized for searching and filtering. Documents here will contain a denormalized combination of the full NftDetailsProto and the NftMarketStatusProto. This index is updated based on NftMarketStatusUpdated and TradeExecuted events.
New Transaction History Index: Create a new index to store detailed records of trades and other financial events for player history display.
GetMarketListings: This gRPC handler now queries the new, optimized market_listings_active_index. Implement full, performant filtering on all indexed attributes (type, rarity, specific stat ranges, class affinities, trait, origin, condition, price). Implement sorting.
GetPlayerRobotState: A new RPC that returns the full, live ActiveBunkerguardDataProto from the player_index.
GetNftActivity: A new RPC that queries the transaction_history_index for an NFT's history.
GetAIAgentInputDataContext: This handler is now significantly enhanced. It returns the live comprehensive context including the player's actual robot class, affiliation, all 12 live stats, a detailed list of GDD-defined equipped items, and recent marketplace activity.
Update docs/progress_logs/progress_phase_4.md:
Log the Indexer enhancements for full dynamic state. Detail the new market_listings_active_index and advanced query/filter capabilities.
Design Rationale
A highly denormalized and specialized index (like market_listings_active_index) is a critical performance optimization. It allows the client to perform complex filtering and sorting for the marketplace with a single, fast query, rather than attempting to perform slow, complex joins at query time. This ensures the marketplace UI feels rich and responsive, even with thousands of listings. Indexing the full, dynamic Bunkerguard state is essential to providing the rich context needed for the Guard UI and the NAR.
Operational Considerations
Local-First: All new indices will be created and managed in the Elasticsearch/Typesense container within the local Docker Compose simulation.
Cloud-Ready: In production, these new indices will require careful capacity planning (sharding, replicas) on the managed AWS Elasticsearch/OpenSearch service to handle the query and ingestion load. Index lifecycle management policies may be needed for the transaction history index.
Verification & Validation Criteria
Integration tests demonstrate that after complex L3 smart contract event sequences (equipping full gear sets, listing/trading items), the Indexer's state is accurate and its advanced filtering queries return correct results.
The new RPCs (GetPlayerRobotState, GetNftActivity) function correctly.
Testing Methodologies
Unit Tests: For the new event transformer logic, ensuring correct mapping from L3 events to the new index document structures.
Integration Tests: Create a complex E2E data validation test. The script will:
Use the Admin CLI to submit a series of transactions to the L3 to create a complex player state (e.g., equip a full set of "Corrupt Vanguard" gear).
Wait for the Indexer to ingest the events.
Call the new GetPlayerRobotState and GetMarketListings gRPC endpoints with various filters.
Assert that the returned data is 100% consistent with the on-chain state created in step 1.
Version Control Strategy
Branching: The Indexer enhancements will be developed on a feature/indexer-dynamic-state branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A security review of the new query handlers is required to ensure that the enhanced filtering capabilities do not introduce new injection vectors.
The authorization logic for the new RPCs must be reviewed to ensure they do not leak data inappropriately (e.g., GetNftActivity should be public, but GetPlayerRobotState should be protected).
ReviewedBy: Indexer Lead, L3 Smart Contracts Lead, Security Lead, Client Leads.
ReviewOutcome: Approved.
ValidationMethod: Integration tests demonstrate that after complex L3 event sequences, the Indexer's state is accurate and its advanced filtering queries return correct results.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 4.6: Implemented Rust Indexer for Full Secure Marketplace & Dynamic Robot State Indexing & Querying (All Live Attributes)." @Phase4/