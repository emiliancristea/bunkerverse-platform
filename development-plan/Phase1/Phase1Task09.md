Task 1.9: C++ Control Center Client - Initial HOMESCREEN
(Data from Functional Rust Account Svc & Functional Rust Indexer API Stubs with Full Schema Defaults, Secure API Calls via Rust App Logic - Principles H, J, K, P, R)
Technical Reference
* CXX-Qt and QML Documentation
* docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md (for HOMESCREEN layout)
* Finalized v0.1 API Contracts for Account Service and AI Data Service
* Finalized v0.1 Protobuf schemas (PlayerProfileResponseProto, etc.)

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
Following a successful zkLogin in Task 1.8, the user is authenticated, but the client application must display their initial profile and state. This task involves implementing the HOMESCREEN UI itself and the underlying Rust application logic required to securely fetch the user's default data from the backend. For Phase 1, this data will originate from the functional-but-static Indexer API stubs, flowing through the Account and AI Data services.
Measurable Objectives
* The initial HOMESCREEN UI is implemented in QML according to the UI Definition document.
* The client's Rust logic can securely fetch the default player profile and AI context data after a successful login.
* The HOMESCREEN successfully displays the full-schema default data for a new user in the MVE.
* The entire flow, from login signal to data display, is validated by end-to-end tests.
Implementation Guidance
Action: Implement the initial HOMESCREEN UI in QML. This screen will display a new user's default profile data for the MVE. The data will be fetched by the client's Rust Application Logic from the functional Rust backend services (Account Service, AI Data Service), which in turn get the hardcoded, full-schema default data from the Indexer API stubs. All API calls must be secure.
* Rust Application Logic (/client/rust-app-logic/src/data_fetcher.rs - Functional & Secure):
* Implement an #[cxx_qt::qinvokable] async fn fetch_initial_player_data(&self) method in the rustBackend QObject.
* This method will be called by the QML UI after a successful login (e.g., in response to the loginSucceeded signal).
* Secure API Calls:
* The method will retrieve the stored platform session JWT from the secure OS keychain via the keyring crate.
* It will create tonic gRPC clients for the Rust Player Account Service and the Rust Server-Side AI Data Service.
* It will make two concurrent gRPC calls using the JWT for authentication:
* Call the functional Rust Player Account Service's (P1.5) GetPlayerProfile endpoint.
* Call the functional Rust AI Data Service's (P1.6) GetAgentContextForNAR endpoint.
* State Update:
* Upon successful completion of both calls, the method will receive the PlayerProfileResponseProto and AIAgentInputDataContextProto.
* It will validate the responses (e.g., check for expected fields).
* It will update the internal state of the rustBackend QObject with this new data (e.g., update self.player_profile and self.ai_context properties, which are exposed to QML via CXX-Qt).
* This state update will automatically trigger the corresponding ...Changed signals (e.g., playerProfileChanged, aiContextChanged) that QML is listening to.
* Error Handling: Robustly handle errors from the services (e.g., UNAUTHENTICATED if JWT is bad, UNAVAILABLE if a service is down). Emit a specific QML error signal (e.g., dataFetchFailed(errorMessage)) so the UI can display a user-friendly error message.
* QML Frontend (/client/qml-ui/screens/HomeScreen.qml - Displaying Full Schema Defaults):
o Data Fetch Trigger: On this screen's Component.onCompleted or in response to the rustBackend.loginSucceeded signal, it will call rustBackend.fetchInitialPlayerData().
o UI State: The screen will display a loading indicator while data is being fetched. It will listen for rustBackend.dataFetchFailed to show an error message.
o Data Binding: The QML components on the HOMESCREEN will directly bind to the properties of the rustBackend QObject. For example:
* Text { text: rustBackend.playerProfile.bunkerTag }
* Text { text: "XP: " + rustBackend.playerProfile.xpBalance }
* Text { text: "Credits: " + rustBackend.playerProfile.creditsBalance } (Will display '0' for MVE)
* A component to display the default ActiveBunkerguardDataProto details:
* Text { text: "Level: " + rustBackend.playerProfile.activeBunkerguardData.level } // Displays '1'
* Text { text: "Class: " + (rustBackend.playerProfile.activeBunkerguardData.currentClass || "Unclassed") } // Displays 'Unclassed'
* Text { text: "Affiliation: " + rustBackend.playerProfile.activeBunkerguardData.currentAffiliation } // Displays 'Neutral'
* A grid or list to display the 12 sub-stats from rustBackend.playerProfile.activeBunkerguardData.finalStats.
o NAR Narrative Placeholder: A dedicated area (e.g., NarDisplayArea.qml) will be present on the HOMESCREEN. For P1, it might display a default welcome message or a placeholder text like "Awaiting narrative events...".
o AI Context Summary Placeholder: A debug panel (visible only in dev builds) could display a summary of the received AIAgentInputDataContextProto to help validate the data flow.
* Testing:
o Component/Unit Tests (Rust App Logic & QML):
* Test the fetch_initial_player_data method in Rust, mocking the Account and AI Data Service gRPC clients. Verify correct concurrent calls and state updates.
* Test the QML HomeScreen.qml component using Qt Test, providing a mock rustBackend object to verify property bindings and UI updates.
o E2E Tests:
* This test runs immediately after the successful zkLogin E2E test (P1.8).
* After login, the client automatically navigates to the HOMESCREEN.
* The test will verify that the QML UI makes the call to rustBackend.fetch_initial_player_data.
* It will verify that the client's Rust app logic makes secure, authenticated gRPC calls to the functional Account and AI Data service stubs running in the local Docker Compose simulation.
* It will verify that the HOMESCREEN UI correctly displays the full-schema default data returned by the Indexer API stubs (via the services). This includes checking that Level is 1, Class is "Unclassed", Affiliation is "Neutral", and all 12 sub-stats are displayed with the value 10.
* Verify that if a service stub is configured to return an error, the UI displays a proper error message.
* Update docs/progress_logs/progress_phase_1.md:
o Log the implementation of the data_fetcher.rs module in the client's Rust app logic and its secure gRPC calls.
o Detail the HomeScreen.qml implementation, including its data bindings to the Rust rustBackend QObject.
o Explicitly list the default MVE data points that are being displayed on the HOMESCREEN (Level 1, Unclassed, Neutral, 12x sub-stats at 10, etc.).
o Document adherence to First Principles H, J, K, P, R.
Design Rationale
This task establishes the primary data flow pattern for the entire client: QML triggers an async Rust function, Rust securely fetches data from the backend, Rust updates its internal state, and QML automatically reacts via property bindings. This is a robust, performant, and secure pattern that keeps complex logic in Rust. Making concurrent calls for profile and AI context data is a performance optimization that speeds up the initial screen load time.
Operational Considerations
The performance of this initial data fetch will be a key metric to monitor post-launch, as it directly impacts the user's perception of the application's speed. Caching strategies in the Account Service (Task 1.5) are the first line of defense against performance issues here.
Verification & Validation Criteria
* All unit and E2E tests pass.
* After a successful zkLogin, the HOMESCREEN successfully fetches and accurately displays the full-schema default player profile data for the MVE, using secure and authenticated API calls to the backend stubs.
Testing Methodologies
A combination of mocked unit tests for the Rust logic and QML components, and a full E2E test that validates the entire client-to-backend data fetching pipeline in the local Docker Compose environment.
Version Control Strategy
* Branching: The HOMESCREEN implementation will be developed on a feature/ branch (e.g., feature/client-homescreen).
* Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
* The Security Lead must review the fetch_initial_player_data function to ensure that the JWT is handled securely and is correctly attached as a bearer token for all authenticated gRPC calls.
* The QML UI code will be reviewed to ensure it only displays data from the rustBackend and does not contain complex logic that could be a security risk.
ReviewedBy: Client Lead (Rust & QML), Rust Account Service Lead, Rust AI Data Service Lead, Security Lead (for JWT usage in API calls), UI/UX Lead.
ReviewOutcome: Approved.
ValidationMethod: All unit and E2E tests pass. After a successful zkLogin, the HOMESCREEN successfully fetches and accurately displays the full-schema default player profile data for the MVE, using secure and authenticated API calls.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.9: Implemented CC HOMESCREEN (Full Schema Default Data for MVE from Functional Rust Account Svc via Secure Functional Rust Indexer API Stubs)." @Phase1/

------------------------------------------------------------------------------------------------------------------
