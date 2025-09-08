Task 2.7: C++ Control Center Client - Social UI Implementation
(Functional Basic Connections via L3 Smart Contracts, Live Basic Indexer Data, Local NAR Narrative - Principles H, J, K, P, R)
Technical Reference
Finalized v0.1 social_service.proto API Contract
Finalized L3 Smart Contract ABIs
docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md (for Social screen)
CXX-Qt and QML Documentation
Context/Problem Statement
While the MVE focuses on the platform, a foundational social layer is essential for community building and retention. The client currently has a placeholder for the "SOCIAL" interface. With a live backend and functional on-chain logic, we can now implement a basic but complete social loop. This task involves building a functional UI section that allows users to add on-chain social connections and view their list of connections, with the data being served by the live Indexer and the actions being recorded by the L3 Smart Contracts.
Measurable Objectives
A functional Social UI is implemented, replacing the P1.10 placeholder screen.
The UI can successfully fetch and display a user's list of on-chain social connections from the live Indexer.
The "Add Connection" functionality correctly submits a SocialConnectionAdded transaction to the L3 smart contracts.
A simple, contextual NAR narrative is generated for the social action.
Implementation Guidance
Action: Implement a functional (but simplified for P2) Social UI section in the Control Center Client (QML UI with Rust Application Logic). This UI will allow users to add social connections (which creates a canonical event in the L3 Smart Contracts) and view their list of connections, which is fetched from the live Rust Indexer. The local Rust NAR will generate simple narratives for these social actions.
Rust Application Logic (/client/rust-app-logic/src/social/handler.rs - Functional for P2 Scope, Secure API Calls):
o This module, part of the rustBackend QObject, will handle client-side logic for social features.
o #[cxx_qt::qinvokable] async fn fetch_social_connections(&self, player_id: &str):
Validates the player_id.
Calls the functional Rust Social Service's GetSocialConnections gRPC endpoint. (The Social Service in P2 will query the functional Rust Indexer for any SocialConnectionAdded events related to the player).
Receives the list of live connections and updates its internal state, emitting a socialConnectionsChanged signal for the QML UI.
o #[cxx_qt::qinvokable] async fn add_social_connection(&self, target_player_id: &str):
Validation: Perform client-side validation of the target_player_id format.
Transaction Intent: Construct a SocialConnectionAdded transaction intent with the source player ID (from the current session JWT) and the target player ID.
L3 Submission: Submit the intent to the L3 sequencer via a generic, secure submit_l3_transaction method.
UI Feedback & NAR Trigger: On receiving a successful transaction receipt from the L3:
Emit a signal to QML to show a "Connection Request Sent..." status.
Trigger the local Rust NAR (P2.5) with the context of adding a new connection, including the target player's name (stubbed for now).
QML Frontend (/client/qml-ui/screens/SocialScreen.qml - Functional for P2 Scope):
o Replace the placeholder screen with a functional UI.
o Connections List (ListView):
On Component.onCompleted, calls rustBackend.fetchSocialConnections.
Binds to the rustBackend.socialConnectionsModel to display the live list of connections.
o "Add Connection" Form:
A TextField for entering a target player's BunkerTag (name).
An "Add" Button that invokes rustBackend.addSocialConnection with the input text.
The UI will display a "Request Sent..." confirmation message upon success.
o Narrative Display: The Social screen will have an area to display the local Rust NAR narrative generated after successfully adding a connection.
Testing:
a. Component/Unit Tests (Rust App Logic & QML):
i. Test the Rust social handler methods, mocking the Social Service gRPC client and the L3 submission client. Test input validation.
ii. Use Qt Test to test the QML social components, verifying data bindings and interactions.
b. E2E Tests (Client + Functional Rust Backend):
Start the full backend stack (including the L3 and services) in the local Docker Compose simulation.
Log in as two different test players.
Add Connection: As Player A, navigate to the Social screen and add Player B as a connection.
Verify L3 & Indexer: Verify the add_social_connection transaction is made to the L3, the SocialConnectionAdded event is emitted, and after a short delay, the Indexer ingests this event.
Verify UI Update: When Player A re-fetches their connections, verify that Player B now appears in their list, reflecting the live data from the Indexer.
Verify NAR: Verify that a simple, contextual narrative for the social action is displayed in Player A's UI.
Update docs/progress_logs/progress_phase_2.md:
a. Log the implementation of the functional P2 Social UI in QML and its Rust application logic handler.
b. Detail the interaction with the live P2 Indexer (for connection data) and functional P2 L3 Smart Contracts (for adding connections).
c. Document the NAR prompts used for P2 social narratives.
d. Document adherence to First Principles H, J, K, P, R.
Design Rationale
Establishing the social graph directly on-chain is a core tenet of the BUNKERVERSE. It makes social connections a persistent, verifiable part of a user's identity, rather than a transient entry in a centralized database. This on-chain social layer can be used in the future for advanced features like governance, team formation, and social quests. The implementation follows the established pattern: UI triggers Rust logic, which submits a secure transaction to the L3, with the result being read back via the Indexer.
Operational Considerations
As the social graph grows, queries to the Indexer for connection lists may become more complex. The Indexer's data model for social connections will need to be designed for efficient querying (e.g., indexing both sides of a connection) to ensure this UI remains performant at scale.
Verification & Validation Criteria
All unit and E2E tests pass.
A user can successfully add a social connection, which is recorded as a transaction on the L3.
The connection is reflected back in the UI for both users via the live Indexer.
A basic NAR narrative is generated for the action.
Testing Methodologies
A combination of unit/component tests and a full E2E test scenario involving two client instances and the full backend stack running in the local Docker Compose simulation.
Version Control Strategy
Branching: The Social UI and its associated logic are developed on a feature/social-ui-p2 branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
Input validation for the target_player_id is a key security check to prevent malformed data from being sent to the backend.
The add_social_connection function in the L3 smart contract must be reviewed to ensure it cannot be exploited (e.g., to create spam connections or an excessive number of events).
ReviewedBy: Client Lead (Rust & QML), Rust Social Service Lead, L3 Smart Contracts Lead, UI/UX Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All unit and E2E tests pass. A user can successfully add a social connection, which is recorded in the L3 Smart Contracts and reflected back in the UI via the live Indexer. A basic NAR narrative is generated.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 2.7: Implemented CC Social UI (Live Basic Connections from Indexer, Functional Add via L3, Local NAR Narratives)." @Phase2/