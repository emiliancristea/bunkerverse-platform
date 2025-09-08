Task 3.1: Control Center Client - Core UI & Navigation Overhaul
(The Imperial Interface - Principles P, R)
Technical Reference
* CXX-Qt and QML Documentation
* docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md (from Task 0.10)
* Finalized Style.qml singleton (from Task 0.8)

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
The Control Center Client, as it exists at the end of Phase 2, is a collection of functional but disjointed screens. To fulfill the vision of an "all-in-one" hub, we must implement the final, grand navigational structure. This task involves a significant refactoring of the client's root UI to build the complete navigational framework, including the four main "VERSE" tabs and the right-click logo menu for switching client interfaces. This creates the final application shell into which all other platform features will be integrated.
Measurable Objectives
* The main client UI is refactored to include the persistent horizontal navigation bar with four functional tabs: THE NEXUS, THE PLAZA, THE CONSTRUCTS, THE GUARD.
* The right-click menu on the BUNKERVERSE logo is implemented, allowing users to switch between the default "VERSE" interface and a placeholder "SOCIAL" interface.
* The "STREAMING" and "HARDWARE" interface options are present in the menu but are visually locked/disabled.
* The entire navigation system is functional, routing users to the correct placeholder or existing screen.
Implementation Guidance
Action: Refactor the MVE client's core navigation to implement the final, grand vision: the main horizontal navigation bar with the four primary tabs and the right-click logo menu for switching client interfaces.
* Core UI Structure (QML - main.qml):
o Refactor the root QML item to be the main application window that contains the persistent header.
o In the header, implement the clickable BUNKERVERSE logo.
* Left-Click Logic: Implement a Connections block so that a left-click on the logo always navigates the main view back to a designated "Home" screen (which can be HomeScreen.qml for now).
* Right-Click Logic: Implement a MouseArea that detects a right-click and opens a custom ContextMenu QML component.
o Interface Switcher (ContextMenu.qml):
* This menu will contain four items: "VERSE," "SOCIAL," "STREAMING," and "HARDWARE."
* Clicking an item will call a new #[cxx_qt::qinvokable] method on the rustBackend, e.g., rustBackend.switchInterface("VERSE").
* The "STREAMING" and "HARDWARE" items will be visually disabled (enabled: false).
* Main Navigation (VerseInterface.qml):
* Create a new root component for the VERSE interface. This will be the main view loaded by default.
* Implement a TabBar or similar component at the top to represent the four main tabs: THE NEXUS, THE PLAZA, THE CONSTRUCTS, THE GUARD.
* Use a StackLayout or SwipeView as the main content area, which will display the content for the currently active main tab.
* Initially, each tab will navigate to its respective placeholder screen created in Phase 1 (e.g., MarketplaceScreen.qml, GuardScreen.qml).
* Social Interface (SocialInterface.qml):
o Create a new root component for the SOCIAL interface. For this task, it can be a simple placeholder screen with a "Social Interface - Coming Soon" message.
* Rust Application Logic (rustBackend):
* Implement the switchInterface method. For now, this will simply change an internal state variable and emit a signal (interfaceChanged).
* The root QML logic will listen for this signal and dynamically switch the main content loader's source between VerseInterface.qml and SocialInterface.qml.
Design Rationale
Building the final, complex navigation structure early, even with placeholder content, is a crucial architectural step. It establishes the application's information architecture and provides a stable "skeleton" for all subsequent feature development. This prevents major UI refactoring later in the project and ensures a consistent user experience from the outset. The right-click menu for switching entire client interfaces is a power-user feature that establishes the client as more than just a game launcher-it's a multi-purpose application shell.
Operational Considerations
This refactoring will be a significant change to the client's root component. All subsequent UI tasks in the plan will now build upon this new, finalized structure. The performance of the interface switching will be monitored, although with only two simple interfaces for now, it should be instantaneous.
Verification & Validation Criteria
* The compiled client application launches into the "VERSE" interface by default.
* The four main tabs are visible and clickable, correctly navigating to their respective placeholder screens.
* Right-clicking the logo successfully opens the interface switcher menu.
* Selecting "VERSE" or "SOCIAL" from the menu correctly switches the main view of the application.
* The "STREAMING" and "HARDWARE" options are correctly displayed as disabled.
Testing Methodologies
* Manual UI Testing: The primary validation method is a thorough manual click-through of all new navigational elements to ensure they function as expected.
* Component/Unit Tests (QML): Unit tests will be written for the ContextMenu component and the main TabBar to verify their internal logic and signal emissions.
* Integration Tests (CXX-Qt): An integration test will verify that calling the rustBackend.switchInterface() method correctly emits the interfaceChanged signal.
Version Control Strategy
* Branching: This major UI refactor will be developed on a feature/imperial-interface-shell branch.
* Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
* The Security Lead will review the new ContextMenu and interface switching logic to ensure it doesn't introduce any new attack surfaces or state management vulnerabilities.
ReviewedBy: Client Lead (Rust & QML), UI/UX Lead, Lead Architect, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: Manual click-through and code review confirm the complete navigational structure is implemented as specified and is functional.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 3.1: Implemented Core Control Center UI & Navigation Overhaul." @Phase3/

------------------------------------------------------------------------------------------------------------------