Task 3.9: THE CONSTRUCTS (Game Hub) UI Implementation
(Functional Game Launcher for Post-MVE Games - Principles H, J, K, P, R)
Technical Reference
gRPC/tonic Documentation
Agones & Kubernetes Documentation (for conceptual understanding of server allocation)
Finalized v0.1 API Contracts (matchmaking_service.proto)
docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md (for THE CONSTRUCTS and its subpages)

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
The BUNKERVERSE MVE is a "Platform-First" initiative. While the core game MVPs are slated for post-MVE development, the platform itself must be complete at launch. This requires a fully functional game launcher hub, THE CONSTRUCTS. This task involves building the UI that allows players to browse all planned game constructs, view details, and interact with the matchmaking system, even though the actual game clients are not yet built. This completes the core UI of the Control Center, making the platform feature-complete and ready for the games to be integrated later.
Measurable Objectives
The full CONSTRUCTS UI and its subpages (SitesPage, IslandsPage, CitadelsPage) are implemented, replacing the placeholder.
The UI can display all 9 planned constructs (3 per subpage), with polished previews and descriptions.
The UI is fully integrated with the Matchmaking Service, displaying live player counts for each queue.
The "Launch" button is functional: a user can queue for a construct, and the Matchmaking Service will successfully allocate a stubbed game server instance via Agones.
The client correctly receives the connection info for the stubbed server, but will not launch a game client (as none exists yet).
Implementation Guidance
Action: Implement the fully functional "Constructs" UI section in the Control Center Client. This UI will allow players to browse all 9 Constructs, view high-quality previews for each, and queue for matchmaking.
Backend Service Enhancement (Matchmaking Service):
Placeholder Logic: Enhance the Team Formation Logic for all 9 Constructs. For the 6 non-MVE games, it will simply acknowledge the queue request. For the 3 post-MVE MVPs, it will form a match but will allocate a generic "stub" server.
Agones Integration: Ensure Agones is configured in the local Kubernetes simulation with fleets for "stub" game servers. These stubs can be extremely lightweight containers that simply report "Ready" and "Allocated" states back to Agones.
Client Implementation (Rust & QML):
Rust Application Logic (/client/rust-app-logic/src/constructs/handler.rs):
Implement the #[cxx_qt::qinvokable] functions for the Constructs hub: fetchConstructsData, getQueueStatus(constructType), and enqueueForConstruct(constructType).
The enqueueForConstruct function will validate the player's JWT and call the Matchmaking Service's Enqueue gRPC endpoint. It will then handle the "match ready" notification (via WebSocket/polling) from the service.
Upon receiving the match details (IP, Port), the Rust logic will log a message like "Match found for [Construct]. Ready to launch game client at [IP:Port]. (Game client launch deferred post-MVE)." It will not launch an executable.
QML Frontend (/client/qml-ui/screens/constructs/):
i. ConstructsPage.qml (Main Hub Page):
Layout: A visually stunning page with a TabBar for the three sub-categories: SITES, ISLANDS, CITADELS. The main view will display a GridView of the 3 constructs for the selected category.
ii. Component Logic (ConstructCard.qml): Each construct in the grid will have a unique, high-quality 3D preview scene (e.g., a pre-rendered looping video). It will display the live player count for its queue by polling rustBackend.getQueueStatus.
iii. Game Launch Flow: Clicking "Launch" on a Construct card will call rustBackend.enqueueForConstruct. The UI will then display the queue status ("Finding Match...", "Match Found!"). For P3, it will stop here, as no game client will be launched.
iv. Subpages (SitesPage.qml, etc.): These will be implemented as views that contain the ConstructsPage logic, pre-filtered for their respective category.
Design Rationale
Building a fully functional launcher before the games are ready is a core tenet of the "Platform-First" doctrine. It fully decouples the platform from the game content. This allows the entire platform-including the complex matchmaking and server allocation flow-to be fully tested and hardened. When the game MVPs are developed post-MVE, they simply need to be "plugged in" to this existing, stable launcher, dramatically simplifying their integration.
Operational Considerations
Local-First: The Agones controllers and stub game server fleets will be added to the docker-compose.yml (or local minikube) environment.
Cloud-Ready: The entire matchmaking and Agones-based server allocation flow is a cloud-native pattern. The work done here will translate directly to the production cloud environment without architectural changes.
Verification & Validation Criteria
A user can navigate to THE CONSTRUCTS and browse all 9 constructs across the 3 sub-tabs.
The UI correctly displays live queue counts from the Matchmaking Service.
A user can click "Launch," successfully enter a queue, and the UI will correctly display the "Match Found" status after the Matchmaking Service successfully allocates a stub server via Agones.
The client's Rust logic correctly logs that it would launch a game client, but does not error or crash.
Testing Methodologies
Unit Tests: For the new client-side Rust handler logic.
Integration Tests: A test harness will call the enqueueForConstruct RPC on the Matchmaking Service, and verify in the local Kubernetes simulation that an Agones GameServer is successfully allocated.
E2E Tests: A manual test where a user logs into the client, navigates to the Constructs UI, queues for a match, and verifies the UI correctly updates to show "Match Found."
Version Control Strategy
Branching: The Constructs UI will be developed on a feature/constructs-launcher-ui branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The enqueueForConstruct flow must be reviewed by the Security Lead to ensure the JWT is handled securely and that there are no exploits that could cause denial-of-service against the Matchmaking service.
ReviewedBy: Client Lead (Rust & QML), Rust Backend Lead (Matchmaking), UI/UX Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All tests pass. The Constructs UI is fully functional as a game launcher, successfully interacting with the backend to queue players and allocate stub servers.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 3.9: Implemented THE CONSTRUCTS UI (Functional Game Launcher)." @Phase3/