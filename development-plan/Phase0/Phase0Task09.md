Task 0.9: Control Center Client - C++ Qt Shell & Rust Application Logic Backend Setup
(Finalized "Smart Stubs" for Secure FFI to Rust NAR & Secure gRPC Calls to Backend Service Stubs via Rust App Logic - Principles G, P, R)
Technical Reference
C++, Qt 6.9.2, CMake Documentation
CXX-Qt Documentation
tonic gRPC Client Documentation
nar_ffi.h (from Task 0.7)
All v0.1 gRPC API Contracts (from Task 0.3)
Context/Problem Statement
The client is composed of three tightly integrated parts: the C++ shell (the native application host), the Rust application logic (the "brain"), and the QML UI (the "face"). In Task 0.8, we built the face. Now, we must finalize the skull and the brain and ensure they are perfectly connected. This task involves finalizing the C++ shell's structure and fully implementing the Rust application logic's backend-facing responsibilities, which for this phase means securely consuming the "smart stubs" for both the NAR library and all backend services.
Measurable Objectives
The C++ Qt Application Shell successfully compiles, links the Rust application logic library, and can launch the QML UI.
The Rust application logic can successfully consume all methods on the Rust NAR C-FFI stub.
The Rust application logic can successfully make gRPC calls to all backend service stubs running in the local Docker Compose environment.
Security hardening for FFI consumption and API interactions is implemented and validated via unit tests.
Implementation Guidance
Action: Finalize the C++ Qt Application Shell structure and the embedded Rust Application Logic "backend". This Rust logic will handle all client-side business operations, including consuming "smart stubs" for the Rust NAR library (via C-FFI) and all backend Rust services (via gRPC). Security hardening of FFI consumption and API interactions is paramount.
C++ Qt Application Shell Implementation (/client/cpp-shell):
main.cpp:
Initialize the Qt Application (QApplication or QGuiApplication).
Instantiate the main Rust QObject (from /client/rust-app-logic, bridged by CXX-Qt).
Create a QQmlApplicationEngine.
Set the Rust QObject instance as a root context property for the QML engine (e.g., engine.setContextProperty("rustBackend", &rust_object_instance);).
Load the main QML file (e.g., qrc:/qml/main.qml).
Start the Qt event loop (app.exec()).
Native OS Integration (Minimal for MVE): Handle any essential OS-level interactions not directly managed by Qt or the Rust app logic (e.g., custom URL scheme registration for OAuth callbacks if not handled by Qt).
Build System (CMake): Ensure CMake correctly builds the C++ shell, links against Qt libraries, and links against the compiled Rust Application Logic library (produced by Cargo and CXX-Qt).
Rust Application Logic Backend (within Client - /client/rust-app-logic):
This Rust library contains the core client-side business logic, state management, and interaction with external libraries/services. It is exposed to QML via CXX-Qt.
NAR FFI Consumer (Rust Logic):
Implement Rust functions (e.g., fn generate_narrative_for_event(...)) that are invokable from QML (via CXX-Qt wrapper).
These functions internally call the Rust NAR library's C-FFI functions from nar_ffi.h.
Secure FFI Handling:
Rigorously validate all inputs before constructing C-structs/strings for the C-FFI call.
Correctly manage memory for C strings passed to and received from the NAR C-FFI (using CString::new for inputs, and CString::from_raw then free_nar_generated_text_ffi for outputs).
Robustly handle NarResultCode error codes returned from the FFI, translating them into Rust NarError enums that can be propagated to QML.
Wrap C-FFI calls in unsafe blocks with clear justifications.
zkLogin Flow Handler (Rust Logic):
Implement Rust functions callable from QML for each step of the zkLogin flow (e.g., fn initiate_google_login(), fn process_oauth_callback(...)).
These functions will use a Rust gRPC client (tonic) to communicate with the Rust Identity Service API stubs running in the local Docker Compose simulation.
Securely manage client-side state needed for OAuth (e.g., state parameter, PKCE code verifier). Store ephemeral secrets securely in memory and zeroize after use.
Handle errors from the Identity Service stubs gracefully.
Backend Platform Service API Consumers (Rust Logic - gRPC to Stubs):
For each backend Rust service (Account, Marketplace, Indexer, etc.), implement a Rust gRPC client using tonic.
Create Rust functions (exposed to QML via CXX-Qt) to call the API methods on these service stubs (e.g., fn fetch_player_profile(), fn list_marketplace_item(...)).
These functions will construct appropriate Protobuf request messages, send them to the service stubs running in the local Docker Compose simulation, and parse the Protobuf responses.
Implement robust error handling for gRPC calls.
Handle JWTs for authenticated calls: Securely store the platform session JWT (obtained from zkLogin) in memory (e.g., within the rustBackend QObject). Attach it as a bearer token to gRPC requests requiring authentication. Implement logic for JWT expiry and refresh (stubbed for P0, full logic later).
State Management (Rust Logic): The Rust application logic will hold the client-side representation of the application state (e.g., current user profile, inventory, NAR narratives). This state will be updated based on responses from backend service stubs or NAR library calls. Changes to this state will trigger signals (via CXX-Qt) to notify QML for UI updates.
Update docs/progress_logs/progress_phase_0.md:
Detail the C++ Qt Shell's main.cpp structure and its role in initializing Rust/QML.
Describe the architecture of the Rust Application Logic library, including its modules for NAR interaction, zkLogin flow, and backend API consumption.
Document the specific Rust functions exposed to QML via CXX-Qt.
Detail the secure FFI consumption patterns for the Rust NAR C-FFI.
Explain how gRPC client calls to backend service stubs are made from Rust, including error handling and placeholder JWT management.
Design Rationale
Placing all client-side business logic, FFI consumption, and API calls within the Rust library centralizes complexity and leverages Rust's safety and performance. The C++ shell's role is minimized to being a lightweight, secure host for the Rust and QML runtimes. This clear separation of concerns makes the application more secure, maintainable, and testable.
Operational Considerations
Local-First: All gRPC calls from the Rust logic will target the DNS service names defined in the docker-compose.yml file (e.g., http://identity-service:50051). This ensures the client works seamlessly within the local simulation.
Cloud-Ready: Because communication uses DNS service names rather than localhost, no code changes will be required when the backend services are deployed to Kubernetes. The client's configuration will simply be updated with the public endpoint for the new environment.
Verification & Validation Criteria
Successful compilation of the C++ shell linking the Rust application logic library.
Unit tests for Rust client app logic methods that call NAR FFI stubs and backend service gRPC stubs (including tests for error handling, data marshalling, and memory management for FFI) pass.
A basic QML UI can successfully invoke a stubbed Rust function (e.g., rustBackend.getNarStubNarrative("test")) and display a placeholder result.
Testing Methodologies
Unit Tests: Each module within the Rust application logic will have extensive unit tests, using mocking libraries (e.g., mockall) to isolate dependencies on gRPC clients and FFI calls.
Integration Tests: A small suite of integration tests will be developed to verify that the compiled Rust library can successfully make gRPC calls to the backend stubs running in the Docker Compose environment.
Version Control Strategy
Branching: All client backend code will be developed on feature/ branches off of develop.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory security code review of the Rust logic that handles session JWTs is required.
The secure FFI handling patterns (memory management, error handling, input validation) for the NAR consumer logic must be reviewed and signed off by the Security Lead.
The handling of OAuth ephemeral secrets (e.g., PKCE verifier) and their zeroization from memory is a critical security checkpoint.
ReviewedBy: Client Lead (C++ & Rust & QML), Rust NAR Lead, Backend Service Stubs Lead, Security Lead.
ReviewOutcome: Control Center C++ Shell & Rust Application Logic Backend Framework with Secure "Smart" FFI/API Stub Consumption Approved.
ValidationMethod: Successful compilation of C++ shell linking Rust app logic. Unit tests for Rust client app logic methods that call NAR FFI stubs and backend service gRPC stubs pass. Basic QML UI can invoke a stubbed Rust function and display a placeholder result.
Git Commit: "Phase 0.9: Finalized CC C++ Shell & Rust App Logic Backend Framework with Secure 'Smart' FFI/API Stub Consumption."