Task 0.13: Initial Automated Testing Setup & Integration ("Smart Stubs")
(Finalized Frameworks & Initial Security-Aware Tests for Rust Backend & Rust/CXX-Qt/QML Client - Principle O, R)
Technical Reference
The Rust Book (Testing Chapter)
mockall crate documentation
testcontainers-rs crate documentation
Qt Test Framework Documentation
GoogleTest/Catch2 Documentation
docs/TESTING_STRATEGY.md (from Task 0.0)
Context/Problem Statement
Manual testing is not scalable, repeatable, or sufficient for a project of this complexity. To ensure code quality, prevent regressions, and validate security, we must establish a comprehensive automated testing strategy from the outset. This task involves selecting and setting up the appropriate testing frameworks for each component of our stack (Rust backend, NAR library, C++/Rust/QML client) and implementing an initial suite of security-aware unit and integration tests. These initial tests will run against the "smart stub" versions of their dependencies, allowing us to validate logic and security at the boundaries before full end-to-end integration.
Measurable Objectives
Finalized testing frameworks for Rust, C++, and QML are selected and integrated into the CI/CD pipeline (Task 0.4).
Initial sets of unit and integration tests are implemented for all MVE components, running against "smart stub" dependencies.
A foundational suite of security negative test cases is implemented, validating input validation and error handling at all critical API and FFI boundaries.
Initial test coverage reports are generated and published in CI.
Implementation Guidance
Action: Finalize the selection and setup of testing frameworks for all MVE components. Implement initial sets of unit and integration tests that run against the "smart stub" versions of dependencies. These initial tests must include basic security negative test cases to validate input validation and error handling at API/FFI boundaries.
Testing Frameworks & Setup:
Rust Backend (Indexer, Services including Axum Marketplace):
Unit Tests: Standard Rust #[test] functions within each crate. Use mocking libraries like mockall where necessary to isolate units.
Integration Tests: Standard Rust #[test] functions, often in a tests/ directory. These will involve:
Spinning up instances of the service being tested (e.g., an Axum test server for Marketplace Service).
Using gRPC clients (tonic) to interact with the service.
Using Dockerized dependencies (e.g., a test instance of Redis for Marketplace service tests, a mock gRPC server for the L3 sequencer for Indexer tests) managed by crates like testcontainers-rs or manually orchestrated for CI via the local Docker Compose simulation.
Rust NAR Library (/libs/nar-rust-wrapper-for-llama-cpp):
Unit Tests: Test Rust wrapper logic, prompt construction, parameter validation. Mock llama.cpp FFI calls.
FFI Integration Tests (Rust calling C-FFI stubs): Test the actual llama.cpp interaction for model loading and basic inference if feasible in CI without excessive resource use (otherwise, focus on FFI boundary with stubs).
Control Center Client (Rust Application Logic, CXX-Qt, QML UI, C++ Shell):
Rust Application Logic (/client/rust-app-logic):
Unit Tests: Standard Rust #[test] functions. Mock gRPC clients (for backend service calls) and NAR FFI calls.
CXX-Qt FFI Integration Tests (Rust side): Test functions exposed to QML, ensuring they handle inputs correctly and emit signals as expected.
QML UI (/client/qml-ui):
Unit/Component Tests: Utilize Qt Test framework (QTest module) for QML. Write test cases in QML or C++ to verify individual QML component behavior, property bindings, and signal emissions.
Focus on testing UI logic, basic interactions, and integration with a mocked version of the rustBackend QObject.
C++ Shell (/client/cpp-shell):
Unit Tests: Use a C++ testing framework (e.g., GoogleTest) to test any C++ specific logic in the shell, including its interaction with the Rust libraries. Mock the Rust libraries for isolated C++ shell tests.
Test Configuration & CI Integration:
Ensure all tests can be run via cargo test for Rust components and standard C++/Qt test runners for C++/QML.
Integrate test execution into the CI/CD pipelines (Task 0.4). CI jobs must fail if any tests fail.
Generate test coverage reports (e.g., using grcov for Rust) and publish them in CI.
Initial Test Case Implementation (Against Smart Stubs):
Rust Backend Integration Tests (Service-Level):
For each service (Marketplace, Identity, Account, etc.): Write integration tests that call its gRPC API (as exposed by its smart stub) with valid inputs and verify the expected schema-compliant stubbed responses.
Security Negative Tests (Backend):
Send gRPC requests with malformed or oversized payloads to smart stubs; verify stubs return appropriate error codes (e.g., INVALID_ARGUMENT) and do not crash.
Attempt to call gRPC endpoints (on stubs that simulate auth) without mock authentication tokens; verify stubs return UNAUTHENTICATED or PERMISSION_DENIED.
Send inputs designed to test input validation rules defined in P0.3; verify stubs reject them correctly.
Rust Client Application Logic Integration Tests (Interacting with Stubs):
Test Rust client app logic functions that call backend service stubs (via gRPC) and the Rust NAR FFI stub. Verify correct request construction, response parsing, and error handling.
Security Negative Tests (Client Logic FFI/API Consumption):
Test Rust client app logic's handling of error codes returned by Rust NAR FFI stubs (e.g., ensure NarError is correctly propagated).
Test Rust client app logic's handling of gRPC errors returned by backend service stubs.
QML UI Integration Tests (Interacting with Rust App Logic Stubs via CXX-Qt):
Write Qt Test cases that simulate user interactions in QML (e.g., button clicks that trigger rustBackend methods).
Verify that calling stubbed Rust methods results in the expected UI updates.
Security Negative Tests (QML -> Rust FFI Boundary):
From QML tests, attempt to call #[cxx_qt::qinvokable] Rust methods with invalid parameters (e.g., out-of-range numbers). Verify the Rust Application Logic handles these gracefully, returns appropriate error indicators to QML, and does not crash the client.
C++ Shell FFI Consumption Tests (for NAR):
If the C++ shell directly calls the NAR C-FFI (rather than Rust app logic doing it), C++ unit tests will verify:
Correct marshalling of data to nar_ffi.h C-structs.
Correct invocation of free_nar_generated_text_ffi for memory returned by NAR FFI stubs.
Robust handling of NarResultCode errors.
Security Negative Tests (C++ -> NAR FFI Boundary): Send invalid parameters (null pointers, overly long strings) to Rust NAR FFI stubs; verify C++ code handles error codes robustly and there are no crashes.
Update docs/progress_logs/progress_phase_0.md:
Document the chosen testing frameworks for each component type.
List key initial unit and integration test suites created.
Detail specific security negative test cases implemented for backend stubs and client FFI boundaries (e.g., "TestRustIdentityStub_InvalidJWT_ReturnsAuthError", "TestClientRustLogic_CallNarFfiWithNullPrompt_ReturnsNarErrorInvalidInput").
Report initial test coverage metrics if available.
Design Rationale
Establishing a comprehensive testing culture and framework from the outset is critical for maintaining quality and development velocity. The "Testing Pyramid" principle (Principle O) guides our focus on a large base of fast unit tests, a smaller set of integration tests, and a minimal set of E2E tests. Security negative tests (Principle R) are essential for hardening the system's most vulnerable points-its external boundaries-and must be included from the very beginning.
Operational Considerations
The test suites will be run automatically by the CI/CD pipeline (Task 0.4) on every pull request. This ensures that no code that breaks existing functionality or fails security checks can be merged into the main development branch. These tests will be the project's primary defense against regressions.
Verification & Validation Criteria
All defined P0 scope unit and integration tests pass in the CI pipeline.
This includes the successful execution of all basic security negative test cases against the smart stubs.
Test coverage reports show initial, reasonable coverage for key business logic and security-critical modules.
Testing Methodologies
Unit Testing: Use #[test] in Rust, GoogleTest in C++, Qt Test in QML. Mocking is used to isolate components.
Integration Testing: Use Rust integration tests (tests/ directory) and the local Docker Compose simulation to test service-level interactions against smart stubs.
Version Control Strategy
Branching: All test code is developed alongside the application code in feature/ branches.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security Lead must review and approve the suite of security negative test cases to ensure it provides adequate coverage for the defined API and FFI boundaries.
The integration of testing frameworks and security-aware tests into the CI pipeline is a mandatory security gate.
ReviewedBy: QA Lead, Security Lead, Lead Rust Developer, Lead C++ Developer, Client Lead (Rust & QML).
ReviewOutcome: Phase 0 Automated Testing Frameworks & Initial Security-Aware "Smart Stub" Integration Tests Approved.
ValidationMethod: All defined P0 scope unit and integration tests pass in CI, including basic security negative test cases against smart stubs. Test coverage reports show initial coverage for key modules.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 0.13: Finalized Automated Testing Frameworks & Initial 'Smart Stub' Integration Tests (incl. Security Negative Tests for Rust Backend & Rust/CXX-Qt/QML Client)." @Phase0/