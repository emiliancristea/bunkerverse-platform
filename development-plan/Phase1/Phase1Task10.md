Task 1.10: Control Center Client - All Other Subpage Stubs
(Placeholders - Verified & Schema-Aware for Full Schemas - Principles P, R)
Technical Reference
* QML Language Documentation
* CXX-Qt Documentation
* Finalized v0.1 Protobuf Schemas (from Task 0.3)
* docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
The client's core navigation framework was established in Task 0.8, and the HOMESCREEN was implemented in Task 1.9. However, the destinations for all other primary navigation links are currently non-existent. To create a complete-feeling application shell, establish the full information architecture, and prepare for future feature development in a structured way, we need to create placeholder screens and corresponding data-type stubs for all remaining main UI sections of the MVE.
Measurable Objectives
* Basic QML placeholder files for all main UI sections (THE NEXUS, THE PLAZA, THE CONSTRUCTS, THE GUARD, and their sub-tabs) are created.
* The main navigation components in the UI correctly navigate to these placeholder screens.
* The client's Rust application logic contains "schema-aware" placeholder data structs for each of these future features, ensuring type-safety and consistency with the backend schemas from day one.
Implementation Guidance
Action: Create the basic QML file structure for all other main UI sections of the Control Center Client as defined in the scope (THE NEXUS, THE PLAZA, THE CONSTRUCTS, THE GUARD, etc.). In Phase 1, these will be non-functional placeholders that simply display a title or a "Coming Soon" message, but their navigation will work, and their underlying Rust/QML types will be aware of the full P0.3 schemas.
* Implementation:
o QML File Creation: Create the following QML files in /client/qml-ui/screens/ and their respective subdirectories:
* NEXUS: CampScreen.qml, ArcadeScreen.qml, ArchivesScreen.qml
* PLAZA: MarketplaceScreen.qml (will contain filters for Primary/GhostMarkt), ExchangeScreen.qml (locked UI)
* CONSTRUCTS: ConstructsScreen.qml
* GUARD: GuardScreen.qml (will contain Inventory, Stats, Logs views)
* SOCIAL: SocialScreen.qml
o Placeholder Content: Each screen will contain a simple layout with a title Text element (e.g., Text { text: "Marketplace" }) and a placeholder message.
o Navigation Integration: Update the main navigation components (TabBar for VERSE, right-click menu for interfaces) to correctly navigate to these placeholder screens when their respective buttons are clicked.
o Schema-Aware Type Definitions (in Rust App Logic):
* Even though the UI is a placeholder, the client's Rust application logic (/client/rust-app-logic) will have stubbed-out modules for these features (e.g., src/marketplace/, src/guard/).
* Within these modules, define the Rust structs that will eventually hold the data for these screens (e.g., struct MarketplaceListing, struct GuardRobotState). These structs must be derived from or mapped from the full Protobuf schemas defined in P0.3.
* This ensures that as these features are built out in later phases, the data types are already correctly defined and consistent with the backend schemas.
* Testing:
* Manual UI Testing: Launch the client, log in, and click on each navigation item. Verify that the correct placeholder screen is displayed.
* Code Review: Review the Rust stub modules to ensure the placeholder data structs align with the P0.3 Protobuf schemas.
* Update docs/progress_logs/progress_phase_1.md:
o Log the creation of all placeholder QML screens and their integration into the main navigation system.
o Document the creation of schema-aware Rust data struct stubs for these future features.
Design Rationale
Creating the full navigational skeleton and schema-aware data stubs early is a crucial architectural step. It enforces the project's information architecture and prevents future integration problems. When a developer begins work on the Marketplace in a later phase, the UI file, the navigation to it, and the Rust data types they will be working with are already defined and consistent with the rest of the ecosystem. This accelerates future development and reduces errors.
Operational Considerations
These placeholder screens will be iteratively replaced with functional UIs in subsequent development phases. The schema-aware Rust structs will be fleshed out with logic to fetch and manage live data.
Verification & Validation Criteria
* A manual click-through test confirms all navigation links work correctly and display the appropriate placeholder screen.
* A formal code review confirms that the newly created Rust type stubs are consistent with the finalized Protobuf schemas from Task 0.3.
Testing Methodologies
* Manual System Testing: The primary validation method is to manually launch and navigate through the compiled client application to ensure the UI shell is complete and correct.
* Static Analysis: Code review of the Rust structs serves as a form of static analysis to ensure type consistency.
Version Control Strategy
* Branching: The placeholder UIs and Rust stubs will be developed on a feature/client-ui-placeholders branch.
* Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
* While the UI is non-functional, the Security Lead will review the stubbed Rust modules to ensure they don't contain any placeholder logic that could be insecure if accidentally enabled.
ReviewedBy: Client Lead (QML & Rust), UI/UX Lead.
ReviewOutcome: Approved.
ValidationMethod: Manual click-through test confirms all navigation links work and display the correct placeholder screen. Code review confirms Rust type stubs are consistent with Protobuf schemas.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.10: Verified CC Placeholder Stubs for All Main Screens (Full Schema-Aware Type Definitions in Rust App Logic)." @Phase1/

------------------------------------------------------------------------------------------------------------------
