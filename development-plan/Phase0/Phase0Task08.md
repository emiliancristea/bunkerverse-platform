Task 0.8: Control Center Client - QML UI Framework Setup
(Finalized Structure & Review using CXX-Qt with Rust App Logic, Styling, Core Components - Principle R for Dependencies & Secure Practices)
Technical Reference
Qt 6 & QML Documentation
CXX-Qt Documentation
Finalized 0.7 Shared Libraries and FFI Stubs
Context/Problem Statement
The Control Center is the primary window through which users will interact with the BUNKERVERSE. Before we can build any specific screens, we need a robust, scalable, and secure framework for the entire UI. This includes the final navigation structure, a consistent styling system, a set of common components, and the core state management pattern that securely bridges the QML frontend with our Rust application logic backend. This task lays that foundational groundwork.
Measurable Objectives
A core QML application shell is created that implements the final navigation structure (VERSE interface with four main tabs, right-click logo menu).
A consistent styling and theming system, based on a QML singleton, is established.
A library of basic, reusable QML components is created.
The Rust-driven state management pattern (QML binds to Rust properties exposed via CXX-Qt) is implemented and validated.
Implementation Guidance
Action: Finalize the Control Center Client's UI framework using QML, with CXX-Qt bridging to the Rust application logic. This includes setting up the core application structure, defining a consistent styling approach, implementing common reusable QML components, and establishing state management patterns driven by the Rust backend. Security best practices for QML development and dependency management must be applied.
Core QML Application Structure & CXX-Qt Integration:
Main Application Window (main.qml): Define the root QML item (e.g., ApplicationWindow). Set up initial window properties (title, size, visibility).
CXX-Qt Root Object Instantiation: In main.cpp (C++ shell), instantiate the root Rust QObject (defined in /client/rust-app-logic and exposed via CXX-Qt) and set it as a context property for the QML engine, making it accessible globally in QML as rustBackend.
Core Navigation System (QML): Implement the final MVE navigation system. This will involve:
A main TabBar or RowLayout for the four primary VERSE interface tabs: THE NEXUS, THE PLAZA, THE CONSTRUCTS, THE GUARD.
A StackLayout to display the content for the currently active main tab.
Logic for the right-click logo menu to switch between the main client interfaces (VERSE, SOCIAL, and the disabled STREAMING, HARDWARE placeholders).
Layout Structure (QML): Define reusable QML components for common layouts (e.g., Header.qml, SidePanel.qml, MainContentArea.qml) using Qt Quick Layouts or manual positioning.
Styling and Theming (QML):
Decide on a styling approach:
Utilize and customize Qt Quick Controls (e.g., Button, TextField) for a native feel while applying custom styles.
Singleton for Theme/Style Constants (Style.qml): Create a QML singleton to define global style constants (colors, font sizes, spacing units). Components can then reference these constants (e.g., color: Style.primaryColor). This allows for easier theme changes later.
Define a basic visual theme (color palette, typography, iconography approach) consistent with BUNKERVERSE branding.
Common Reusable QML Components:
Develop a small library of common custom QML components:
StyledButton.qml: A button with consistent project styling.
StyledTextInput.qml: A text input field with validation state indicators.
LoadingSpinner.qml: An animated loading indicator.
ErrorMessage.qml: A component to display error messages.
ListItem.qml: A generic list item component for displaying data in lists.
These components should be designed for reusability across different screens.
State Management (Rust-Driven, Exposed to QML via CXX-Qt):
The primary source of truth for application state (user profile, inventory, marketplace listings, etc.) resides in the Rust application logic (/client/rust-app-logic).
Rust structs representing state are exposed to QML as QObject properties via CXX-Qt (e.g., rustBackend.currentUserProfile, rustBackend.marketplaceListingsModel).
QML UI components bind to these Rust-owned properties.
When state changes in Rust (e.g., after an API call to a backend service returns new data), Rust emits signals (defined with CXX-Qt, e.g., currentUserProfileChanged()). QML property bindings automatically update the UI.
User interactions in QML invoke #[cxx_qt::qinvokable] Rust methods on the rustBackend object. These Rust methods then handle the logic (e.g., calling a service via gRPC) and update the Rust-side state, which in turn updates the QML UI.
Security Considerations for QML UI Framework:
Qt/QML Versioning: Ensure the chosen Qt version, 6.9.2, is actively maintained and patched for security vulnerabilities. Pin this version in the build system and COMPATIBILITY_MATRIX.md.
Dynamic QML Loading: Avoid loading QML code dynamically from untrusted sources. All QML for MVE will be bundled with the client.
JavaScript in QML: Be cautious with complex JavaScript embedded within QML. Prefer driving logic from Rust where possible. Avoid using eval() in QML JavaScript.
Input Sanitization/Validation (Display): While primary input validation happens in Rust, if QML directly constructs display strings from multiple sources, ensure proper encoding and sanitization to prevent UI injection or display issues. For MVE, most displayed data comes from trusted backend sources via Rust.
Context Property Security: Be mindful of what is exposed via QML context properties. The rustBackend object should only expose necessary functions and properties, adhering to the principle of least interface.
WebView Usage (If Any - for ARCADE tab):
The ARCADE tab will require a WebView. This component must be carefully secured.
Review NPM packages for any such WebView's HTML/JS content via npm audit --production.
Implement security headers (Content Security Policy, HSTS) via the C++ shell that hosts the WebView.
Strictly avoid dangerouslySetInnerHTML or equivalent.
Update docs/progress_logs/progress_phase_0.md:
Detail the final QML application structure, including the four main tabs and the right-click menu logic.
Describe the chosen styling approach and the initial theme definition.
List key reusable QML components created.
Explain the Rust-driven state management pattern and its integration with QML.
Document the security considerations addressed for QML and the planned secure implementation of the WebView for the ARCADE tab.
Design Rationale
A Rust-driven state management pattern is critical for security and performance. It keeps all complex business logic out of the QML/JavaScript layer, reducing the attack surface and leveraging Rust's strengths. Implementing the final, complex navigation structure now, even with placeholder content, ensures that the core UX is established early and all future features can be built into a stable and consistent shell.
Operational Considerations
The reusable component library and styling singleton will be critical for maintaining UI consistency and development speed throughout the project. The security of the WebView component for the ARCADE tab will require ongoing vigilance and adherence to web security best practices.
Verification & Validation Criteria
Code review of the QML framework structure, CXX-Qt integration points, and initial components is completed.
A basic shell application renders successfully with the final main navigation bar (NEXUS, PLAZA, CONSTRUCTS, GUARD) and the right-click logo menu.
Navigation between a few stubbed QML screens, driven by the Rust application logic stub, is functional.
Security considerations are documented and reviewed by the Security Lead.
Testing Methodologies
Unit Tests: Unit tests will be created for any complex QML component logic using Qt Test.
Integration Tests: A basic integration test will launch the client shell and verify that the rustBackend object is successfully exposed to the QML context.
Version Control Strategy
Branching: All client framework code will be developed on feature/ branches.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security Lead must review and approve the state management pattern, particularly the boundary between QML and Rust.
The security plan for the ARCADE tab's WebView component is a mandatory checkpoint.
ReviewedBy: Client Lead (Rust & QML), UI/UX Lead, Security Lead, Lead Architect.
ReviewOutcome: Control Center Client QML UI Framework Structure & Initial Components Approved.
ValidationMethod: Code review of the QML framework structure, CXX-Qt integration points, and initial components. Successful rendering of a basic shell application with navigation between a few stubbed QML screens, driven by the Rust application logic stub. Security considerations documented and reviewed.
Git Commit: "Phase 0.8: Finalized Control Center Client QML UI Framework Structure (CXX-Qt with Rust App Logic, Styling, Core Components Reviewed, Secure Practices Applied)."