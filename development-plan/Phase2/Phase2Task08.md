Task 2.8: Rust Admin CLI - Interaction with Functional L3 Smart Contracts & Live Indexer
(Full Schema P2 Logic & Live Data Validation - Principles H, R)
Technical Reference
clap and tonic crate documentation
Finalized L3 Smart Contract ABIs and v0.1 indexer_service.proto
Ethers-rs or similar L3 client library (for state derive/read calls)
Context/Problem Statement
The Admin CLI from Phase 1 was built to interact with simple stubs. Now that the L3 smart contracts contain functional state logic (Task 2.1, 2.2) and the Indexer is serving live data (Task 2.4), the CLI must be updated to interact with these live components. This enhancement is essential for creating a powerful internal tool for testing, debugging, and, most importantly, validating the end-to-end data consistency between the canonical on-chain state and the indexed read-layer.
Measurable Objectives
The Admin CLI is updated to fully interact with the now-functional L3 smart contracts and the live Indexer query API.
A new state derive command is implemented to read and display the full on-chain state for a player directly from the L3 smart contracts.
An indexer query command is enhanced to query the full live data from the Indexer.
The CLI is successfully used in test scripts to validate the data pipeline's consistency.
Implementation Guidance
Action: Update the Rust Admin CLI to fully interact with the now-functional L3 Smart Contract event sourcing logic (Tasks 2.1, 2.2) and the functional Rust Indexer query capabilities (Task 2.4). This makes the CLI an essential tool for testing and validating the live backend state.
CLI Commands Enhancement & Security:
All commands must continue to use secure gRPC (TLS and auth tokens if configured).
bunkerverse-admin-cli netchain submit-transaction ... --payload-json <json_representing_full_p2_proto>:
Enhance this command to be a primary testing tool. It can now be used to submit transactions with full P2 schema data directly to the L3 sequencer, such as NftMinted with a full GDD-defined NftDetailsProto, or RobotLinked which should trigger the default class/affiliation events in the smart contract.
bunkerverse-admin-cli netchain state-derive --player-id <PlayerID> (New/Enhanced Command):
This command will now make a secure RPC read-call (e.g., using ethers-rs) to a public view function on the deployed BunkerguardManager smart contract.
This on-chain function will return the full AgentChainStateProto struct for the given PlayerID.
The CLI will pretty-print the full derived AgentChainStateProto in JSON format, showing the live P2 detailed state directly from the source of truth. This is critical for verifying L3 smart contract logic.
bunkerverse-admin-cli indexer query ... (Enhanced):
This command now queries the functional Rust Indexer (Task 2.4).
It will pretty-print the live indexed response (showing the live P2 full schema default player state, NFT details, etc. as seen by the Indexer).
Using state-derive and indexer query together allows for direct, scriptable comparison and validation of the L3 Smart Contracts -> Indexer data pipeline.
Testing:
o Use the CLI extensively in manual and automated test scripts to validate the L3->Indexer pipeline for all new rich P2 data.
o Example test script:
i. bunkerverse-admin-cli netchain submit-transaction --type NftMinted --payload-json '{...}' to mint an item for Player X.
ii. bunkerverse-admin-cli netchain submit-transaction --type RobotLinked --payload-json '{...}' for Player X.
iii. Wait a few seconds for indexing.
iv. bunkerverse-admin-cli netchain state-derive --player-id PlayerX -> Capture output A.
v. bunkerverse-admin-cli indexer query --player-id PlayerX -> Capture output B.
vi. Compare A and B to verify data consistency.
Update docs/progress_logs/progress_phase_2.md:
o Log the enhancements to the CLI, especially the new state-derive command for direct on-chain state validation.
o Provide example CLI usage for testing the L3 smart contract state and comparing it against the live Indexer state to ensure consistency.
Design Rationale
A powerful and reliable Admin CLI is an indispensable tool for a complex blockchain project. The state-derive command provides an unassailable source of truth by reading directly from the on-chain contracts, while the indexer query shows the state that the user would see. The ability to programmatically compare these two is the most effective way to validate the health and consistency of the entire data pipeline and to quickly diagnose issues like Indexer lag or data transformation bugs.
Operational Considerations
This CLI will be the primary tool for QA automation and for DevOps/SRE to debug potential data consistency issues in the staging and production environments. Its access in production will be tightly controlled and require secure credentials.
Verification & Validation Criteria
All CLI commands successfully interact with the functional L3 smart contracts and the Indexer backend running in the local Docker Compose simulation.
The state-derive and indexer query commands can be used together in a script to demonstrate and validate the data flow and state consistency for the P2 full schema defaults.
Testing Methodologies
Extensive integration testing of the CLI against the full backend stack running in the local Docker Compose environment. Automated test scripts will be created to perform the "Compare A and B" validation logic.
Version Control Strategy
Branching: The CLI enhancements will be developed on a feature/admin-cli-p2-enhancements branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The CLI must continue to enforce secure gRPC (TLS). Any mechanism for handling authentication tokens must be reviewed to prevent credential leakage.
The debug endpoint/view function on the smart contract for state-derive must be reviewed to ensure it's a read-only (view) function and cannot be used to alter state.
ReviewedBy: L3 Smart Contracts Lead, Indexer Lead, QA Lead, Security Lead (for CLI access patterns).
ReviewOutcome: Approved.
ValidationMethod: All CLI commands successfully interact with the functional L3/Indexer backend. The state derive and indexer query commands are used together to demonstrate data flow and state consistency.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 2.8: Enhanced Rust Admin CLI for Functional L3/Indexer Interaction (Live Full Schema P2 Logic & Data Validation)." @Phase2/