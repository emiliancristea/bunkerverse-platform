Task 0.1: Project Setup, Version Control & Developer Onboarding
(Security-Aware Monorepo & Devs, CXX-Qt Client Structure - Principles A, B, C, D, R)
Technical Reference
Git Documentation
Docker & Docker Compose Documentation
Cargo.toml & Cargo Workspaces Specification
CMakeLists.txt Specification
rust-toolchain.toml Specification
docs/PROJECT_GOVERNANCE_AND_WORKFLOWS.md
Context/Problem Statement
With our core governance framework established in Task 0.0, we must now create the physical digital workspace (the monorepo) and the training program for our developers. An unstructured repository and inconsistent development environments are a primary source of bugs, security risks, and development friction. This task sets up the entire project's directory structure, version control policies, and the comprehensive, security-first onboarding process that every developer must follow. It ensures a clean, stable, and secure foundation before any significant code is written.
Measurable Objectives
A functional Git monorepo with the complete, approved directory structure is initialized and pushed to the remote repository.
A new developer can successfully clone the repository, follow the DEVELOPMENT_ENVIRONMENT.md guide, build all initial stubs, and run pre-commit hooks.
The entire founding team has completed the initial security awareness and secure coding training session.
Initial ADR drafts for all core architectural components have been created with the mandatory sections.
Implementation Guidance
Action: Initialize the Git monorepo, define the complete directory structure for all MVE components, create initial project files (Cargo.toml, CMakeLists.txt, QML project files, rust-toolchain.toml, etc.), set up Git hooks for pre-commit checks, define the branching strategy, decide on monorepo management tools, and establish a comprehensive developer onboarding process. This setup must fully account for the chosen technology stack (Rust services, Rust/CXX-Qt/QML client, UE5 Game Server stubs for post-MVE) and the project's architecture from day one, with a strong emphasis on security.
Monorepo Initialization & Directory Structure:
Initialize a Git monorepo at the project root.
Create a rust-toolchain.toml file at the root with the content: [toolchain]\nchannel = "1.80.0" to pin the Rust version.
Define and create the top-level directory structure:
/protos: For all Protocol Buffer definitions (shared across Rust backend and potentially C++ game servers).
/contracts: For all BUNKERVERSE Netchain L3 smart contracts (Rust/WASM, via Arbitrum Stylus).
/services: For all Rust platform microservices.
/services/identity: Rust Identity Service (zkLogin).
/services/account: Rust Player Account Service.
/services/marketplace: Rust Marketplace Service (Axum-based).
/services/payment: Rust Payment Service (Stripe integration).
/services/mission: Rust Mission Service.
/services/social: Rust Social Service.
/services/ai_data: Rust AI Data Service.
/services/feedback: Rust Feedback Service (stub for MVE).
/indexer: For the Rust Decentralized Global Indexing Layer (Elasticsearch/Typesense client).
/libs: For shared libraries.
/libs/common-rust: Shared Rust utility code, common Protobuf types.
/libs/nar-rust-wrapper-for-llama-cpp: Rust NAR library wrapping llama.cpp.
/libs/rt-shared-cpp: Shared C++ real-time libraries for game servers.
/client: For the Control Center Client.
/client/rust-app-logic: Core Rust application logic for the client.
/client/qml-ui: QML files for the user interface.
/client/cpp-shell: C++ Qt application shell integrating Rust logic (via CXX-Qt) and QML.
/client/assets: UI assets (images, fonts for QML).
/games: For C++ UE5 game server projects and the Netchain UE5 Plugin.
/games/ue5-plugin: Netchain plugin for Unreal Engine 5.
/games/constructs/arena_site_mvp, /games/constructs/frontier_island_mvp, /games/constructs/synthesis_citadel_mvp: UE5 projects for post-MVE MVP game servers.
/games/constructs/stubs: Placeholder UE5 projects for the other 6 constructs.
/tools: For development and utility scripts.
/infra: For Infrastructure-as-Code (Terraform, Kubernetes manifests, Dockerfiles).
/docs: All project documentation (ADRs, governance, design, user guides, etc.).
Initialize Rust projects (cargo init) in /contracts, /services/*, /indexer, /libs/common-rust, /libs/nar-rust-wrapper-for-llama-cpp, /client/rust-app-logic.
Initialize C++ projects (CMakeLists.txt shells) in /libs/rt-shared-cpp, /client/cpp-shell, /games/ue5-plugin, /games/constructs/*.
Initialize QML project structure in /client/qml-ui.
Add README.md shells to each top-level and key sub-directory.
Add .gitignore configured for Rust, C++, Node.js (if any build tools), QML, UE5, and common OS files.
Git Hooks & Branching Strategy:
Implement pre-commit Git hooks using a tool like husky (if Node.js is used for tooling) or custom scripts. Hooks must run:
Code formatters (rustfmt for Rust, clang-format for C++, potentially a QML formatter).
Linters (clippy -- -D warnings -A clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::cargo for Rust, clang-tidy or compiler warnings for C++, qmllint if chosen).
Basic security checks (cargo audit --deny warnings for Rust).
Define and document branching strategy (e.g., Gitflow: main, develop, feature/, release/, hotfix/). main branch must be protected, requiring PRs and passing CI checks.
Monorepo Management Strategy Decision:
Evaluate and decide on tools/strategies for managing the monorepo if needed (e.g., cargo workspaces for Rust, CMake for C++ inter-dependencies, top-level build scripts). For MVE, cargo workspaces and a top-level CMake orchestrating C++ builds might suffice initially. Document the chosen strategy.
Submodule Integration:
Add llama.cpp as a Git submodule, referenced by the /libs/nar-rust-wrapper-for-llama-cpp build.
0.1.a. Comprehensive Developer Onboarding Documentation & Security Training Plan:
Action: Create/Update docs/DEVELOPMENT_ENVIRONMENT.md and integrate its setup guide with the onboarding section in docs/PROJECT_GOVERNANCE_AND_WORKFLOWS.MD.
Content for docs/DEVELOPMENT_ENVIRONMENT.md:
Detailed setup instructions for the target development OS (Windows 11).
Pinned versions and installation methods for:
Rust toolchain (via rust-toolchain.toml, which specifies 1.80.0 stable).
C++ compilers (MSVC - specific versions from COMPATIBILITY_MATRIX.md).
Node.js (for any build tooling, via .nvmrc).
CMake, Protocol Buffer Compiler (protoc), language-specific gRPC plugins.
Qt 6.9.2 SDK (MSVC 2022 64-bit): The standardized installation path is D:\DOCUMENTS\Qt\6.9.2\msvc2022_64. Build scripts will reference this path.
CXX-Qt crate setup instructions, noting that version 0.7.2 is pinned in the project's Cargo.toml files.
Unreal Engine 5 setup instructions (for post-MVE development).
Docker Desktop for Windows setup, configured for WSL2.
Mandatory setup of local security tools: IDE plugins for SAST (SonarLint, Rust Analyzer with Clippy integration). Configuration of local dependency vulnerability checkers (cargo audit).
Content for docs/PROJECT_GOVERNANCE_AND_WORKFLOWS.MD Onboarding Section:
Step-by-step guide for new developers to clone the monorepo, install all prerequisites, build all components, and run the local Docker Compose simulation of the backend.
Explanation of the Git branching strategy, PR process, and mandatory PR review checklists.
Detailed instructions on how to report vulnerabilities internally.
Guidelines for developing features considering the dual-mode architecture.
Action: Conduct an initial, mandatory security awareness and secure coding training session for the entire Phase 0 team. This session will cover:
OWASP Top 10 Web Application Security Risks (and their relevance to backend services).
Common language-specific security pitfalls: C++ (memory corruption), Rust (unsafe block usage, FFI security), QML (injection vulnerabilities).
Secure FFI practices (for NAR C-FFI and Rust-QML CXX-Qt).
Input validation techniques at all boundaries.
Authentication and Authorization best practices.
Dependency management hygiene.
Introduction to Threat Modeling (STRIDE).
Secure handling of secrets.
ADRs - Drafts Created (with Mandatory Security Sections & Dual-Mode Analysis):
Create initial ADR files (e.g., docs/adr/0001-l3-architecture-overview.md, docs/adr/0002-indexer-choice.md, 0003-client-tech-stack.md, 0004-nar-integration.md, 0005-zklogin-flow.md, 0006-marketplace-service-design.md).
Each ADR draft must include:
A mandatory "Security Considerations & Threat Model Outline" section.
A mandatory "Dual-Mode (Off-Chain/On-Chain) Design Considerations" section.
Update docs/progress_logs/progress_phase_0.md:
Log all actions: Git repo initialization, directory structure creation, rust-toolchain.toml creation, project initializations (including Rust/CXX-Qt/QML client), Git hooks setup, branching strategy definition, monorepo management decision, submodule additions. Detail how Principle R is embedded in the developer environment setup (e.g., "Git pre-commit hooks now include cargo audit --deny warnings and clippy for stricter Rust checks."). Document the completion of the initial security training session and link to training materials. Detail ADR draft creation with specified sections.
ReviewedBy: Tech Lead (Rust), Tech Lead (C++/Client), DevOps Lead, Lead Architect, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: Successful clone, build, and execution of basic "hello world" or stub functionality by a new team member following DEVELOPMENT_ENVIRONMENT.md. Onboarding session completed by all P0 team members, confirmed by attendance list. Initial security training session completed, materials reviewed. ADR drafts contain the required sections.
Git Commit: "Phase 0.1: Initial Monorepo & Project Structures (incl. Rust/CXX-Qt/QML Client), Security-Aware Dev Onboarding & Training, ADR Drafts with Security & Dual-Mode Sections."
Design Rationale
A clean monorepo structure is essential for managing the complexity of a multi-language project, simplifying dependency management and build orchestration. Enforced pre-commit hooks catch errors and security issues at the earliest possible stage, reducing CI load and preventing flawed code from ever being committed. A single, unified development environment document eliminates "works on my machine" issues. The directory structure is updated to remove the proprietary "NLS" and replace it with a standard /contracts directory, reflecting our L3-based architecture and improving clarity for future Web3-native developers.
Operational Considerations
The pre-commit hooks and protected branch strategy will be the primary gatekeepers of code quality for the project's entire lifecycle. The local Docker Compose setup defined in the onboarding docs will be the standard development environment, ensuring consistency. All components are designed to be built within this local-first context but are structured for future cloud deployment (e.g., separation of services, IaC in /infra).
Verification & Validation Criteria
A new developer can successfully clone the repo, follow the DEVELOPMENT_ENVIRONMENT.md guide, build all components, and run the local Docker Compose simulation of the backend. Pre-commit hooks are functional and prevent commits with formatting/linting errors. The initial security training session is completed by all, confirmed by attendance.
Testing Methodologies
Unit/Integration: N/A for this task.
System: A developer successfully runs the "hello world" tests or stubs, confirming the environment is correctly configured.
Performance: N/A for this task.
Version Control Strategy
Branching: Gitflow (main, develop, feature/, etc.). main and develop are protected.
Commits: All initial project skeletons, directory structures, and documentation are committed to the develop branch. The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security Lead must review and approve the content of the security training materials.
The Security Lead must review and approve the security-related pre-commit hooks (cargo audit).
The creation of ADRs with mandatory security sections is a required checkpoint.