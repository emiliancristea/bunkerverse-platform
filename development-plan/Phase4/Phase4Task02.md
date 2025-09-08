Task 4.2: L3 Smart Contracts - NFT Module
(Full Logic & Secure Storage for All MVE ItemTypes, Rarities, Conditions, Stats, Traits, Affinities - Principles I, N, R)
Technical Reference
Finalized P4 GDDs (defining all 10 ItemTypes, 6 Rarities, 4 Conditions, and their attributes)
Rust (w/ Arbitrum Stylus) Smart Contract Language Documentation
Finalized v0.1 Protobuf Schemas (for NftDetailsProto, NftMutableStateProto)

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
The NFT logic implemented in Phase 2 was basic, supporting only default item types. To create a rich and strategic economy, our on-chain system must support the full diversity of all 10 MVE ItemTypes, 6 Rarities, and 4 Conditions, with all their GDD-defined attributes (stats, traits, etc.). This task involves finalizing the NFT module within the L3 smart contracts to become the canonical source of truth for all item attributes, and hardening the minting process to be securely driven by an on-chain GDD, preventing all forms of item exploits.
Measurable Objectives
The NftContract suite is finalized to support the full lifecycle and state for all 10 MVE ItemTypes.
The adminMintNft transaction logic is hardened to use a comprehensive, on-chain GDD as the sole source for an item's canonical properties.
The logic for tracking and updating mutable NFT state (owner, condition, market status) is fully implemented.
The system is validated by successfully minting and storing examples of all 10 ItemTypes with their full, GDD-defined attributes.
Implementation Guidance
Action: Finalize the NFT module in the L3 Smart Contracts to support the complete lifecycle, state, and detailed attributes for all 10 MVE ItemTypes, 6 Rarities, and 4 Conditions. The L3 Smart Contracts will now be the canonical source for GDD-defined item attributes, not just P2 defaults.
1. Implementation Details (in /contracts/src/):
On-Chain GddItemDatabase:
Finalize the on-chain database (e.g., a dedicated smart contract) that maps a string item_gdd_id to its full, specific NftDetailsProto.
This database must be populated (e.g., via an admin-only setter function in a deployment script) with the GDD data for a wide variety of items across all types and rarities, with their specific stat boosts, class affinities, and traits.
NftMintIntent Processing: The admin_mint_nft function in the BunkerguardManager contract now uses this comprehensive on-chain GDD. It is the only source for an item's canonical, non-mutable properties.
NftMutableStateProto Canonical Storage: The global storage mapping for mutable NFT state (mapping(uint256 => NftMutableState) in the NFT contract) is fully implemented, tracking current_owner_id, current_condition, is_soulbound, and market_status.
NftConditionUpdated Event Logic: The on-chain logic that applies this event is now fully implemented. If this event is applied to an NFT that is currently equipped by a player (this check is done during the transaction execution), it must trigger the full dynamic robot stat/class/affiliation re-evaluation logic (Task 4.3).
2. Security Code Review:
o Re-review the NFT minting logic to ensure NftDetails are always sourced from the trusted on-chain GDD, preventing any possibility of minting custom/exploited items. This is a critical security checkpoint.
3. Update docs/progress_logs/progress_phase_4.md:
o Log implementation of the canonical on-chain storage for GDD-defined rich NftDetailsProto and NftMutableStateProto.
o Detail the final, secure minting process.
Design Rationale
Making the L3 smart contracts the canonical source for all GDD-defined item attributes ensures that the "rules of physics" for items are transparent, verifiable, and immutable. The on-chain GDD registry is the most critical security pattern in our economy; it makes it impossible for an attacker (or even a compromised backend service) to mint an item with stats that are not explicitly defined and approved in the game's design. This protects the integrity of the entire game economy.
Operational Considerations
Local-First: All development and testing of the enhanced NFT contracts will be done against the local Arbitrum Orbit simulation.
Cloud-Ready: The GDD data will be deployed to the on-chain GDD registry contract via a secure, multi-sig-protected administrative transaction as part of the production deployment and future content patches.
Verification & Validation Criteria
Unit and integration tests pass for minting examples of all 10 ItemTypes with their full, GDD-defined attributes correctly stored on-chain.
An attempt to mint an item with an invalid item_gdd_id or to provide custom stats in the transaction is correctly rejected by the smart contract.
Testing Methodologies
Unit Tests (Smart Contracts): Write exhaustive unit tests for the GDD database contract lookup logic. Test the admin_mint_nft function with both valid and invalid item_gdd_ids. Test the logic for all mutable state update functions.
Integration Tests: Use the Admin CLI to call the admin_mint_nft function for one item of each of the 10 ItemTypes. Then, use the CLI's state-derive and indexer query commands to verify that the minted NFTs exist on-chain and are indexed with the correct, full set of GDD-defined attributes.
Version Control Strategy
Branching: The enhanced NFT module will be developed on a feature/l3-nft-module branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory, in-depth security review of the entire NFT minting and state management logic in the smart contracts is required.
The access control for populating and updating the on-chain GDD registry is a critical security checkpoint to prevent the injection of malicious item definitions.
ReviewedBy: L3 Smart Contracts Lead, Security Lead, Game Design Lead (for item GDD accuracy).
ReviewOutcome: Approved.
ValidationMethod: Tests pass for minting examples of all 10 ItemTypes with their full, GDD-defined attributes correctly stored in the L3 smart contracts.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 4.2: Finalized L3 Smart Contract NFT Module (Full ItemType/Rarity/Condition/Stat/Trait/Affinity Logic & Secure GDD-Sourced Storage)." @Phase4/