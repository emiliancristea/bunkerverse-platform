Task 3.7 (Revised & Expanded): THE NEXUS - "ARCADE" Implementation
(Functional & Secure HTML5 Game Arcade with L3 Rewards - Principles H, J, K, P, R)
1. Technical Reference
API Contracts (v0.1): arcade_service.proto
L3 Smart Contracts: NTC.sol and XpToken.sol ABIs (for reward minting)
Libraries: Qt WebView and QWebChannel modules, gRPC/tonic, sqlx.
Security Standards: Content Security Policy (CSP) Level 3, OWASP Web Security Testing Guide.
UI/UX Design Document: docs/ui/CLIENT_QML..._MVE_V0.1.md (for THE NEXUS/ARCADE ).

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
2. Context/Problem Statement
Context: The platform needs a casual engagement loop that is accessible to all users, including non-paying Spectators, to drive retention and provide a low-stakes introduction to the play-to-earn economy.
Problem: Integrating third-party web-based games (HTML5) into a native desktop client is a significant security risk. The web content must be strictly isolated from the main client application and the user's system to prevent exploits like Cross-Site Scripting (XSS) or remote code execution. Furthermore, a secure backend system is required to manage the game library, validate game results, and orchestrate the on-chain L3 rewards without being vulnerable to cheating.
3. Measurable Objectives
Backend: A new, functional Rust Arcade Service is implemented and deployed to the local Docker Compose environment.
Client (QML & Rust): The full ARCADE UI and its subpages are implemented, featuring a securely sandboxed WebView for playing games.
Security: The WebView implementation is hardened with a strict Content Security Policy and a limited-bridge communication channel.
End-to-End Validation: A user can launch an HTML5 game, play it, and upon winning a challenge, have their score securely validated by the backend, resulting in a verifiable NTC/XP reward being minted on the L3.
4. Implementation Guidance
API Definition (arcade_service.proto):
o Define gRPC RPCs: GetGamesList, GetChallenges(gameId), StartGameSession(gameId), and SubmitGameResult(signedPayload).
Service Logic (Axum/Tonic):
Game & Challenge Management: The service will load a GDD file (arcade_games.yaml) defining the available games, their URLs, and associated challenges (e.g., "Score 10,000 points in Astro-Smasher").
StartGameSession: When a user starts a game, this RPC generates a unique, single-use session_id and returns it to the client. The client's Rust logic will inject this session_id into the game's JavaScript environment.
SubmitGameResult (Critical Security):
This endpoint receives a cryptographically signed payload from the game's JavaScript. The payload will contain the session_id, the final score, and other relevant data.
The JavaScript within the game will use a client-side secret (provided at session start) to create a signature (e.g., HMAC-SHA256) of the game result.
The backend must validate this signature to ensure the result is from a legitimate session and has not been tampered with. This is the primary anti-cheat mechanism.
L3 Integration: If a validated score meets the criteria for a challenge, the service triggers a transaction to the L3 to mint the earned XP or NTC reward to the player's account.
Rust Application Logic (/client/rust-app-logic/src/nexus/arcade_handler.rs):
Handlers: Implement #[cxx_qt::qinvokable] functions to interact with the new Arcade Service (fetchGamesList, startArcadeGameSession, etc.).
Secure Bridge Logic: Implement the Rust-side logic for the QWebChannel. This will be the only bridge between the QML/WebView and the Rust backend. It will expose a very limited set of functions to the JavaScript environment, such as submitGameResult(signedPayload) and getCurrentUser().
QML Frontend (/client/qml-ui/screens/nexus/arcade/):
ArcadePage.qml (Main Hub):
Layout: A vibrant, visually engaging page. Feature a large panel for a "Featured Game" and GridViews for "New Games" and "Popular Challenges."
Backend Interaction: On load, calls rustBackend.fetchArcadeData() to populate the views.
GamesPage.qml (Game Browser):
Layout: A filterable grid of all available HTML5 games.
Backend Interaction: Fetches data from rustBackend.fetchGamesList(). Clicking a game's "Play" button navigates to the GameInterface.qml for that game.
ChallengesPage.qml & RewardsPage.qml:
Layout: UI lists that display available and completed challenges, and a history of rewards earned from the Arcade.
Backend Interaction: Fetches data from the Arcade Service and the Indexer (for reward history).
GameInterface.qml (The Secure Sandbox):
Layout: This will be a simple view containing primarily the WebView component.
WebView Implementation (Critical Security):
The WebView's url property will be set to the game's URL provided by the Arcade Service. URLs will be whitelisted on the backend.
Sandboxing: The WebView will be configured with the most restrictive sandbox settings possible.
Content Security Policy (CSP): The C++ Shell will inject a strict CSP header into the WebView's response, preventing it from loading untrusted scripts or making unauthorized network requests.
Communication: A QWebChannel object will be exposed to the WebView's JavaScript context. This is the only way the game can communicate with the client. It will not have direct access to the rustBackend or any local files.
Backend Interaction: Before loading the URL, the client calls rustBackend.startArcadeGameSession(gameId). The returned session_id and client-side secret are injected into the JavaScript context via the QWebChannel for the game to use.
PracticePage.qml (Placeholder):
A polished placeholder screen for a future mode where users can play games without spending Energy or earning rewards.
5. Design Rationale
The "secure WebView with a limited bridge" is the industry-standard pattern for safely integrating untrusted web content into a native application. All game logic and state are untrusted; the backend Arcade Service is the single source of truth for validating scores and issuing rewards. The signed payload from the game client provides a strong deterrent against simple cheating (e.g., manually calling the API with a high score).
6. Operational Considerations
Local-First: The Arcade Service will be added to the docker-compose.yml file. HTML5 games will be served from a simple web server container.
Cloud-Ready: In production, the curated HTML5 games will be hosted on a secure CDN. The security and integrity of these third-party game files will be an ongoing operational concern, requiring regular audits.
7. Verification & Validation Criteria
A user can navigate to the ARCADE, browse games, and launch one in the WebView.
After winning, the score is securely submitted, and a verifiable NTC/XP reward transaction appears on the L3 block explorer.
Security testing confirms the WebView sandbox is effective and cannot be escaped.
8. Testing Methodologies
Unit Tests: For the Arcade Service's score validation logic.
Integration Tests: Validate the full loop: start session -> get signed payload from mock game -> submit result -> verify L3 transaction is created.
E2E Tests: A manual test where a user plays a simple, controllable HTML5 game, achieves a winning score, and verifies the reward appears in their client.
Security Testing (Mandatory): Dedicated penetration testing of the WebView component and the QWebChannel bridge to attempt a sandbox escape or exploit the game result submission process.
9. Version Control Strategy
Branching: This feature set will be developed on a feature/nexus-arcade branch.
Commits: The Git Commit message for this task will be exactly as specified.
10. Security Audit & Compliance Checkpoints
A mandatory, exhaustive security audit of the WebView integration and the QWebChannel bridge is required before this task can be approved. This is the largest new attack surface introduced in the MVE.
The Arcade Service's score validation and anti-cheat mechanism (signed payloads) must be rigorously reviewed.
ReviewedBy: Client Lead (Rust & QML), Rust Backend Lead, L3 Smart Contracts Lead, UI/UX Lead, Security Lead (mandatory reviewer for WebView).
ReviewOutcome: Approved.
ValidationMethod: All tests pass. The ARCADE reward loop is functional and secure. The WebView sandbox is proven to be robust.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 3.7: Implemented THE NEXUS - ARCADE (Functional & Secure HTML5 Game Hub)." @Phase3/