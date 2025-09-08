Task 3.6 (Revised & Expanded): THE NEXUS - "CAMP" (Community Hub) Implementation
(Functional Missions, Events, Social & Squad Hubs with L3 Rewards - Principles H, J, K, R)
1. Technical Reference
API Contracts (v0.1): events_service.proto, mission_service.proto, social_service.proto
L3 Smart Contracts: NTC.sol (for transfer), BunkerguardManager.sol (for on-chain state), NftContract.sol (for adminMint) ABIs.
UI/UX Design Document: docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md (specifically the sections for THE NEXUS -> CAMP and all its subpages).
Game Design Documents: Missions_GDD_V1.2.md, Events_GDD_V1.1.md.
Libraries: gRPC/tonic, sqlx, tokio-tungstenite, serde, ethers-rs, CXX-Qt, QML.

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
2. Context/Problem Statement
Context: Following the completion of the core UI shell (3.1), The Plaza (3.2), The Archives (3.3), and the foundational Social interface (3.4), the Control Center is a functional but static platform. The core economic and social loops exist but lack a central, driving purpose to engage the community.
Problem: The project needs a central, unified hub to drive long-term community engagement and provide all users-Founders, Players, and Spectators-with a clear sense of purpose and progression. Without this, the platform is merely a collection of disconnected features. The "CAMP" is designed to be this hub, integrating missions, events, and social features into a single, cohesive experience. This task involves building the complex backend services and the rich, multi-faceted client UI required to bring this vision to life.
3. Measurable Objectives
Backend:
Deploy a functional, secure, and persistent Rust Events Service capable of managing the full lifecycle of events and tournaments.
Deploy a functional, secure, and persistent Rust Mission Service capable of defining, tracking, and orchestrating rewards for all defined in-verse missions.
Establish a secure inter-service communication channel (e.g., via a message bus) for the Mission Service to receive progress updates from other services.
Client (QML & Rust):
o Implement the full CAMP UI, including all defined subpages (CampPage, MissionsPage, EventsPage, SocialPage, SquadsPage, TournamentsPage), replacing all placeholders.
o Ensure all subpages are populated with live, dynamically fetched data from their respective backend services.
End-to-End Validation:
Successfully validate the entire in-verse mission loop: A user can accept a mission (e.g., "Add a friend"), perform the action in another part of the client, see their mission progress update automatically, and securely claim the NTC/XP reward, with the reward being verifiably recorded on the L3.
Successfully validate the events loop: An admin can create an event, and a user can view that event and its associated leaderboard in the client.
4. Implementation Guidance
New Rust Events Service (/services/events/):
o Project Setup: Create the new Rust binary project. Add dependencies for tonic, sqlx, prost, tokio, tracing.
o API Definition (events_service.proto): Finalize the gRPC service with RPCs: GetActiveEvents, GetLeaderboard(eventId), GetTournamentDetails(tournamentId), and admin-only CreateEvent, UpdateLeaderboard, DistributeTournamentRewards.
o Database Schema (MySQL): Use sqlx-cli to create migrations for events (id, name, description, start_time, end_time), tournaments (id, event_id, bracket_info_json), and leaderboard_entries (id, leaderboard_id, player_id, score, rank).
o Service Logic: Implement the gRPC handlers. Get... RPCs will query the MySQL database. The DistributeTournamentRewards RPC is critical: it will be protected by an admin authorization check. It will read the winners and prize amounts from its database, construct a batch of transfer transactions, and submit them to the L3 Smart Contracts via the Transaction Submission Service.
New Rust Mission Service (/services/mission/):
o Project Setup: Create the new Rust binary project.
o API Definition (mission_service.proto): Finalize gRPC RPCs: GetAvailableMissions, AcceptMission(missionId), ClaimMissionReward(missionId).
o Mission Definitions (GDD): Implement a parser to load and parse the missions.yaml GDD file at service startup into an in-memory HashMap<MissionId, MissionDefinition>. This file will define mission objectives (e.g., type: FORUM_POST_CREATED, count: 1) and rewards (xp: 100, ntc: 50).
o Persistent Storage (MySQL): Use sqlx-cli to create the mission_player_progress database table to track accepted missions and objective progress.
o Inter-Service Event Listener:
Implement a subscriber for your chosen message bus (e.g., NATS).
This subscriber will listen for events like social.friend_added, forum.post_created, arcade.challenge_won.
Upon receiving an event, it will check its database for any players with active missions related to that event type and update their objective_progress_json.
o L3 Integration (ClaimMissionReward): This function is the core of the play-to-earn loop. It will verify mission completion in its database and then securely construct and submit the appropriate reward transactions (e.g., mintNft, mintXp, transferNtc) to the L3 Smart Contracts.
Enhanced Social & Forum Services:
o These services must be enhanced to publish events to the message bus whenever a mission-relevant action occurs. For example, when a user successfully creates a post, the Forum Service will publish a forum.post_created event containing the player_id.
Rust Application Logic (/client/rust-app-logic/src/nexus/camp/):
o State Management: In the main rustBackend QObject, create new cxx_qt::qproperty members to hold the state for the CAMP, typed as QVariantList or custom QObject models: missionsModel, eventsModel, leaderboardModel.
o Handlers: Create new handler modules (missions_handler.rs, events_handler.rs). Implement the #[cxx_qt::qinvokable] async functions for each subpage (fetchMissions, acceptMission, claimMissionReward, fetchEvents, fetchLeaderboard). These functions will call the new backend services via secure gRPC, handle the responses, and update the corresponding state models, which will emit signals to QML.
QML Frontend (/client/qml-ui/screens/nexus/camp/):
CampPage.qml (Main Hub):
Layout: A dashboard-style view. Use a GridView for "Featured Events" and ListViews for "Daily Missions" and "Weekly Missions."
Backend Interaction: On Component.onCompleted, it will concurrently call rustBackend.fetchFeaturedEvents() and rustBackend.fetchSummaryMissions(). The UI elements will bind to the resulting properties.
MissionsPage.qml (Mission Board):
Layout: A TabBar for filtering ("Daily", "Weekly", "Story"). The content of each tab is a ListView with a custom MissionItemDelegate.qml.
Backend Interaction: The ListView's model will be bound to rustBackend.missionsModel. The page will call rustBackend.fetchMissions(filter) when the tab is changed.
Delegate Logic: The MissionItemDelegate will display title, description, and rewards. It will have an "Accept" button (enabled: mission.status === 'AVAILABLE') that calls rustBackend.acceptMission(mission.id). This button will change to a "Claim Reward" button (enabled: mission.status === 'COMPLETE') that calls rustBackend.claimMissionReward(mission.id). If prerequisites are not met, the button will be disabled, with a ToolTip explaining the reason (e.g., ToolTip.text: "Requires: Class - Vanguard").
EventsPage.qml (Event Schedule):
Layout: A calendar or timeline view.
Backend Interaction: Fetches data from rustBackend.fetchEvents(). Clicking an event opens a Dialog with details.
TournamentsPage.qml (Tournament Hub):
Layout: A UI to display tournament brackets (e.g., using a TreeView) and leaderboards (ListView).
Backend Interaction: A dropdown selects a tournament, which calls rustBackend.fetchLeaderboard(tournamentId).
SocialPage.qml & SquadsPage.qml (Placeholders):
Implement these as polished placeholder screens with basic UI elements and "Coming in a future update" text, but with functional navigation.
5. Design Rationale
Consolidating Missions, Events, and Social features into a single "CAMP" hub creates a powerful and intuitive center of gravity for the community. It provides a clear answer to "What should I do now?" for any player. Making in-verse actions into completable, rewarding missions is a brilliant way to incentivize the exact behaviors that build a strong community. The use of a message bus for inter-service communication creates a loosely coupled, scalable, and resilient backend architecture.
6. Operational Considerations
Local-First: All new services (Events, Mission) and the message bus (e.g., a NATS container) will be added to the docker-compose.yml file for the local simulation.
Cloud-Ready: In a production cloud environment, the message bus will be a managed service (e.g., AWS SNS/SQS or a managed NATS cluster). The databases will be managed AWS RDS instances.
Content Management: New missions and events will be added by updating the GDD files and using the Admin CLI to call admin-only gRPC endpoints.
7. Verification & Validation Criteria
A user can navigate to THE NEXUS -> CAMP and browse all its functional subpages.
A user can accept an in-verse mission (e.g., "Add a friend"), perform the action (in the SOCIAL interface), see the mission progress update automatically in MissionsPage, and successfully claim the on-chain NTC/XP reward.
The EventsPage and TournamentsPage correctly display data from the Events Service.
8. Testing Methodologies
Unit Tests: For all new service logic, especially mission prerequisite checking and reward orchestration.
Integration Tests: Validate the full flow of inter-service communication (e.g., a Forum Service event -> NATS -> Mission Service -> DB update).
E2E Tests: A detailed, scripted manual test where a user logs in, accepts a mission, performs the action, and verifies the reward is claimed and appears on the L3 block explorer.
9. Version Control Strategy
Branching: The CAMP hub is a major feature set and will be developed on a feature/nexus-camp-hub branch.
Commits: The Git Commit message for this task will be exactly as specified.
10. Security Audit & Compliance Checkpoints
A mandatory security review of the Mission Service is required, focusing on the reward claiming logic to prevent any exploits.
The inter-service communication bus must be secured (e.g., with authentication) to prevent a compromised service from spoofing mission progress.
Access control on all admin-only gRPC endpoints must be rigorously tested.
ReviewedBy: Client Lead (Rust & QML), Rust Backend Lead, L3 Smart Contracts Lead, UI/UX Lead, Security Lead, Game Design Lead.
ReviewOutcome: Approved.
ValidationMethod: All unit, integration, and E2E tests pass. A user can fully interact with the CAMP hub, and the mission/reward loop for in-verse activities is functional and secure.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 3.6: Implemented THE NEXUS - CAMP (Functional Mission, Event & Social Hub)." @Phase3/
