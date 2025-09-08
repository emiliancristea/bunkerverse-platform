Task 0.4: CI/CD Pipeline Setup
(Comprehensive, Functional, with Integrated Security Scanning for Arbitrum Orbit Backend & Rust/CXX-Qt/QML Client - Principles D, R)
Technical Reference
GitHub Actions Documentation
Docker Documentation
cargo, cmake, Trivy, cppcheck CLI documentation
docs/PROJECT_GOVERNANCE_AND_WORKFLOWS.md (SDL Charter)
Context/Problem Statement
With our core technologies and APIs defined, we must establish an automated, repeatable, and secure process for building, testing, and packaging our software. Manual building is slow, prone to human error, and cannot scale. A comprehensive Continuous Integration and Continuous Deployment (CI/CD) pipeline is essential to enforce our quality and security standards on every single code change, providing rapid feedback to developers and ensuring the integrity of our software artifacts.
Measurable Objectives
Fully functional CI workflows for the backend services, game server stubs (post-MVE prep), and the Control Center client are implemented in GitHub Actions.
All pipelines include mandatory, automated security scanning stages (SAST, dependency, container) that will fail the build on critical findings.
Protected branch rules on GitHub are configured to require all relevant CI jobs to pass before a Pull Request can be merged into develop or main.
Initial pipeline runs are successful, and any security findings on the boilerplate code are documented and triaged.
Implementation Guidance
Action: Design, implement, and thoroughly test comprehensive CI/CD workflows using GitHub Actions. These pipelines must automate the build, test, security scanning, and packaging processes for all MVE components (Rust backend services including the Indexer and Marketplace with Axum; Rust NAR library; C++ UE5 Game Server stubs for post-MVE; Control Center Client with Rust app logic, CXX-Qt, and QML UI). Pipelines must enforce strict adherence to pinned toolchain versions and integrate automated security scanning tools at appropriate stages, failing builds on critical findings.
GitHub Actions Workflow Implementation:
netchain-backend-ci.yml (For all Rust Indexer, Services including Axum Marketplace):
Trigger: On push to develop and feature/* branches, and on PRs targeting develop.
Jobs:
Setup Environment: Install pinned Rust toolchain (from rust-toolchain.toml), protoc compiler, and any other build dependencies using fixed versions.
Lint & Format Check:
Run cargo fmt -- --check for all Rust crates.
Run cargo clippy --all-targets --all-features -- -D warnings -A clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::cargo for all Rust crates.
Build: Run cargo build --all-targets --all-features --release for all Rust backend components.
Unit & Integration Tests: Run cargo test --all-targets --all-features --release for all Rust backend components. Integration tests will use in-memory stubs or Dockerized dependencies.
Security Scans (Rust):
SAST (Dependency Vulnerabilities): Run cargo audit --deny warnings critical high to check for vulnerable crate versions.
SAST (License Compliance): Run cargo deny check license ban against an approved license list (e.g., MIT, Apache-2.0, BSD-3-Clause).
(Optional SAST - Static Analysis): If budget/time allows and a suitable tool is identified for deeper static analysis of Rust code beyond Clippy, integrate it here.
Dockerize Services: For each Rust service (Indexer nodes, Marketplace Service, Identity Service, etc.):
Build a versioned Docker image using a multi-stage Dockerfile. Base images must be minimal (e.g., alpine or distroless if compatible) and from trusted sources. Images must run services as non-root users.
Tag images with Git commit SHA and branch name.
Container Vulnerability Scan: Integrate Trivy to scan the built Docker images for OS and application-level vulnerabilities. Fail the build if critical or high severity vulnerabilities with available fixes are found (configurable threshold).
Publish Artifacts (Optional for CI, Mandatory for Release): Publish Docker images to a private container registry (e.g., AWS ECR, GitHub Container Registry) on successful develop branch builds or release tags.
netchain-gameserver-stubs-ci.yml (For C++ UE5 Game Server Stubs - MVE focus is on stubs for post-MVE prep):
Trigger: On push/PR to relevant paths in /games/.
Jobs:
Setup Environment: Install pinned C++ compiler (matching DEVELOPMENT_ENVIRONMENT.md), CMake, UE5 build prerequisites (if building stubs that link UE headers).
Build C++ UE5 Game Server Stubs: Build the basic C++ server stubs and shared RT libraries.
(Optional Security Scan - C++): Integrate basic C++ static analysis (e.g., cppcheck, or Clang Static Analyzer via build flags like -Wall -Wextra -Werror -fanalyzer).
netchain-client-ci.yml (For Control Center Client - Rust App Logic, CXX-Qt, QML UI, C++ Shell, NAR Lib):
Trigger: On push/PR to relevant paths in /client/ or /libs/nar-rust-wrapper-for-llama-cpp/.
Jobs:
Setup Environment:
Install pinned Rust toolchain (will automatically use 1.80.0 from rust-toolchain.toml).
Install pinned C++ compiler.
Install Qt 6.9.2 SDK (or use a pre-built CI image that contains it). Ensure the environment variables point to the correct installation path.
Install pinned versions of CMake, protoc, cbindgen.
Lint & Format Check:
Run cargo fmt -- --check for Rust client app logic and NAR library.
Run cargo clippy (as above) for Rust client app logic and NAR library.
Run clang-format --check for C++ shell code.
(Optional QML Lint): Integrate qmllint or similar if a suitable tool is chosen.
Build Rust NAR Library:
Build /libs/nar-rust-wrapper-for-llama-cpp as a static or dynamic library. This includes compiling its llama.cpp Git submodule (with secure CMake flags).
Run cbindgen to generate nar_ffi.h.
Build Rust Client Application Logic: Build /client/rust-app-logic as a library, ensuring CXX-Qt correctly generates the C++ bridge code.
Build C++ Client Shell: Build /client/cpp-shell, linking the Rust NAR library and the Rust Client Application Logic library.
Security Scans (Rust Client Logic & NAR):
Run cargo audit --deny warnings critical high.
Run cargo deny check license ban.
Security Scans (C++ Client Shell):
Integrate cppcheck or Clang Static Analyzer.
(Dependency Scan - C++ vcpkg/Conan): If using vcpkg/Conan for Qt or other C++ dependencies, utilize their audit features if available.
Unit Tests:
Run cargo test for Rust client app logic and Rust NAR library.
Run C++ unit tests for the C++ shell (including FFI consumer stubs).
(Optional QML Unit Tests): If a QML testing framework (e.g., Qt Test) is adopted, run QML component tests.
Package Client Installers (On Release Tags/Manual Trigger):
For Windows: Package into a signed .exe installer (using WiX or similar).
For macOS: Package into a signed and notarized .dmg or .pkg.
For Linux: Package into an .AppImage or .deb/.rpm.
Installers must bundle the C++ shell, Rust Client App Logic library, Rust NAR library, QML files, and all necessary runtime components.
CI Configuration & Best Practices:
Workflow Triggers: Define precise path filters for triggers to avoid unnecessary runs.
Caching: Implement caching for Rust dependencies (cargo build artifacts), C++ build artifacts, and Docker layers to speed up CI runs.
Matrix Builds: Use matrix strategies for building client installers for different OS (windows-latest, macos-latest, ubuntu-latest).
Secrets Management in CI: Use GitHub Actions encrypted secrets for any tokens needed during CI (e.g., container registry push credentials, code signing certificates/passwords for release builds).
PR Checks: Configure PRs to develop and main to require all relevant CI jobs to pass before merging is allowed.
Notifications: Configure notifications for CI build failures on develop and main branches.
Integration of Codegen & Security Scans:
Ensure that code generation steps happen before compilation and testing.
PR checks must include all defined security scans. Builds must fail if new, unaddressed critical or high severity issues are introduced by the PR. A process for reviewing, triaging, and formally accepting or deferring findings must be established.
Update docs/progress_logs/progress_phase_0.md:
Document the setup of each CI workflow file.
Detail specific SAST/dependency/container scan tools integrated (Trivy, cargo audit, cargo deny, cppcheck) and their configurations.
Log successful execution of initial pipeline runs for each workflow, including evidence that all security scan stages completed.
Document any initial findings from security scans on the boilerplate/stub code and how they were triaged (e.g., "Integrated cargo-audit into Rust service builds; initial scan on boilerplate found 0 vulnerabilities. Trivy scan on minimal Axum service Docker image found 2 low severity OS package vulnerabilities, triaged and deferred as base image update is pending.").
Design Rationale
A comprehensive CI/CD pipeline is the heart of a modern DevOps culture. It provides rapid feedback to developers, enforces quality gates automatically, and ensures that what is tested is what gets deployed. Integrating security scanning directly into the pipeline ("shifting left") makes security a continuous, automated process, not an afterthought. The chosen structure with separate workflows for the backend and client respects the different build and dependency complexities of each component.
Operational Considerations
These pipelines will be the sole path for code to be promoted to staging and production environments in the future. Their reliability is paramount. The Publish Artifacts step will be disabled during initial development and enabled as we approach deployment phases, pushing to a dev container registry first, then staging/production. The performance and cost of CI runners will be monitored and optimized over time.
Verification & Validation Criteria
Successful green builds for all workflows on PRs to develop. All configured security scan jobs complete successfully. The ability to view scan reports/logs from CI is confirmed. False positive rates for SAST tools are understood and initial tuning (if any) is documented.
Testing Methodologies
The pipeline's primary function is to execute the project's automated tests (Unit, Integration) as defined in Task 0.13. It serves as the automated testing harness.
Version Control Strategy
Branching: All .github/workflows/*.yml files are committed to the main branch to govern the repository's processes.
Commits: Development of workflows happens on feature/ branches and is merged to develop before being promoted to main. The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security and DevOps Leads must review and approve the CI/CD workflows, especially the configuration and failure thresholds for the security scanning tools (Trivy, cargo audit).
The process for triaging and accepting risks from security scan findings must be formally documented and approved.
ReviewedBy: DevOps Lead, Security Lead, Lead Rust Developer, Lead C++ Developer, Client Lead.
ReviewOutcome: CI/CD Pipelines with Integrated Security Scanning Approved for Phase 0 Scope.
ValidationMethod: Successful green builds for all workflows on PRs to develop. All configured security scan jobs complete successfully. False positive rates for SAST tools are understood and initial tuning (if any) is documented. Ability to see scan reports/logs from CI.
Git Commit: "Phase 0.4: Functional CI/CD Pipelines with Integrated Security Scanning (SAST, Dependency, Container for Full Rust/Axum/CXX-Qt/QML Stack)."