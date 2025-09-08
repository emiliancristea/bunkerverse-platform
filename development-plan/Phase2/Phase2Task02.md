Task 2.2: L3 Smart Contracts - Transaction Execution Logic
(Functional & Secure for Core MVE Features, incl. Full Schema NFT Minting & Basic Robot Stat/Class/Affiliation Initialization - Principles I, J, N, R)
Technical Reference
Finalized P2 GDDs (for canonical rules)
Rust (w/ Arbitrum Stylus) Smart Contract Language Documentation
OpenZeppelin Ownable and AccessControl libraries
Finalized v0.1 Smart Contract Schemas and Events
Context/Problem Statement
The smart contracts from Task 2.1 contain the pure state machine logic (apply functions) but lack the public-facing entry points for users and services to initiate state changes. This task involves implementing the functional processing of transaction intents. This logic runs within the L3 smart contracts, is called by an authenticated user or service, derives the current state, performs rigorous stateful validation against that state, and, if valid, generates the definitive sequence of events to be committed.
Measurable Objectives
Public-facing functions for all P2-scope transactions (NFT minting, robot linking, etc.) are implemented in the L3 smart contracts.
Each function performs rigorous, stateful validation before generating events.
The secure NFT minting process, which sources all item details from an on-chain GDD, is implemented and tested.
The logic for initializing a robot's default state upon linking is functional.
All new code passes exhaustive unit tests and a mandatory security review.
Implementation Guidance
Action: In /contracts/src/, implement the functional processing of transaction intents within the L3 Smart Contracts. This logic involves deriving the current state, performing rigorous stateful validation, and generating the definitive sequence of CanonicalEventProtos to be committed. For Phase 2, this covers token operations, NFT minting, basic NFT transfers, and initializing the robot's class/affiliation to defaults upon linking.
Stateful Validation & Event Generation (public functions in L3 Smart Contracts):
These public functions are the heart of the contracts' business logic and security. They take a transaction intent, derive the required current state from on-chain storage, validate the intent against that state, and return one or more events on success or an error on failure.
NFT Minting (Functional for P2 - Admin CLI, Future Game Reward Stubs - with Full NftDetailsProto from GDD):
Intent: pub fn admin_mint_nft(&mut self, owner: Address, item_gdd_id: String)
Execution:
Authorization: This function must be protected with an access control modifier (e.g., only_admin or only_minter_role) to ensure only authorized addresses can mint NFTs.
Canonical Details Lookup (Critical Security Step): The contract will read from a deployed, on-chain GddItemDatabase contract. This contract will map a string item_gdd_id to a full, GDD-defined NftDetailsProto struct. The L3 smart contracts must NEVER trust NftDetails provided by a client or external service in a minting intent.
Strict Validation of GDD-sourced details: Even though sourced from the on-chain GDD, perform a sanity check on the loaded NftDetailsProto to ensure it conforms to system limits (e.g., no stat boost > 100).
Event Generation: Generate a new unique NftId. Emit an NftMinted event containing the new nft_id and the gdd_sourced_details.
Canonical NFT State Storage: As part of this transaction's logic, create and persist the canonical NftDetailsProto and an initial NftMutableStateProto to their own dedicated global storage mappings within the NFT contract.
NFT Transfer (Simplified P2 - Functional for Owner Update):
Intent: pub fn transfer_nft(&mut self, to: Address, nft_id: U256) (standard ERC-721 transferFrom logic).
Execution:
Stateful Validation: The standard _transfer function in an ERC-721 implementation correctly performs the necessary stateful validation (verifies ownership, checks for approvals). It should also include a check to ensure the NFT is not soulbound.
Event Generation: The _transfer function will emit the NftTransferred event.
Robot Linking/Unlinking (Functional with Default Class/Affiliation Initialization):
Intent: pub fn link_robot(&mut self, robot_nft_id: U256)
Execution:
Stateful Validation:
Requires that msg.sender owns the robot_nft_id.
Requires that the player's current active_bunkerguard_data.robot_id is zero (or null).
Event Generation (Atomic Sequence): Generate a sequence of events to be applied atomically by the state machine:
RobotLinked event.
RobotStatsUpdated with the default CoreStatsProto.
ClassAssigned with new_class_opt = None (Unclassed).
AffiliationAssigned with new_affiliation = Neutral.
Robot Leveling (Functional for XP Burn & Level Increment - P2 Scope):
Intent: pub fn level_up_robot(&mut self, xp_cost_for_level: U256)
Execution:
Stateful Validation: Requires active_bunkerguard_data to exist. Requires the player's balances.xp_balance is sufficient. Validates the requested level up is sequential.
Event Generation: Generate and emit [XpBurned { ... }, RobotLevelledUp { ... }].
Social Connection Add (Functional Stub for P2):
Intent: pub fn add_social_connection(&mut self, target_player: Address)
Execution:
Validate that the target_player exists.
Generate and return a SocialConnectionAdded event for the msg.sender.
Threat Model Update & Secure Coding Review (Principle R):
Review and update the threat model for the L3 Smart Contracts (ADR-001), focusing on vulnerabilities related to flawed stateful validation for P2 features.
Specific Threats to Address:
Item Minting Exploit: Attacker tries to mint an item with overpowered stats. Mitigation: Transaction Execution logic always sources NFT details from its internal, secure on-chain GDD database based on an ID, never from the transaction intent itself.
Economic Exploits: Flaws in XP/Credit logic leading to free currency. Mitigation: Mandatory use of a safe math library and rigorous validation of all costs/transfers.
All new smart contract code for transaction execution must undergo a security-focused peer review.
Design Rationale
This layer of logic is the most critical security boundary for the entire on-chain ecosystem. By performing rigorous stateful validation before generating events, the smart contracts act as the ultimate gatekeeper, ensuring that no invalid state transitions can ever be recorded on the ledger. The on-chain GDD registry is a crucial security pattern that prevents a whole class of item minting exploits.
Operational Considerations
These public-facing functions are the ones that will be called by backend services and, in the future, directly by users. Their gas cost must be carefully profiled and optimized, especially the link_robot function which generates a sequence of four events.
Verification & Validation Criteria
All unit and integration tests pass.
The L3 smart contracts successfully process transactions for all P2 core MVE features, creating/updating canonical state for NFTs and initializing basic robot states correctly and securely.
The secure NFT minting process is validated to prevent the creation of non-GDD-compliant items.
Testing Methodologies
Unit Tests:
Exhaustive tests for the stateful validation logic of each transaction type.
Test the event generation logic for each intent, ensuring the correct sequence and content of events is produced.
Test the on-chain GddItemDatabase lookup.
Integration Tests (Local L3 Cluster with functional contracts):
Start a local Arbitrum Orbit node (e.g., using Anvil).
Deploy the full set of P2.1/P2.2 logic contracts.
Use the Admin CLI (or a test harness) to submit transaction intents to the contracts: NftMintIntent, LinkRobotIntent, etc.
Verification: Query the contract's state directly to verify the intent was validated and the correct events were emitted and applied. Use derive_agent_state view functions to verify the final player state reflects the actions.
Version Control Strategy
Branching: All contract development occurs on the feature/p2-l3-execution-logic branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory, security-focused peer review of all new smart contract code is required.
The on-chain GDD lookup for NFT minting is a critical security pattern that must be verified.
All stateful validation logic must be reviewed for potential race conditions or economic exploits (e.g., re-entrancy, oracle manipulation, etc.).
ReviewedBy: L3 Smart Contracts Lead, Rust Backend Lead, Security Lead, Game Design Lead (for P2 scope of item/robot rules).
ReviewOutcome: Approved.
ValidationMethod: All unit and integration tests pass. The L3 smart contracts successfully process transactions for P2 core MVE features.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 2.2: Implemented L3 Smart Contract Transaction Execution (Functional & Secure for P2 Features, GDD-Sourced Full Schema NFT Minting, Default Robot Init)." @Phase2/