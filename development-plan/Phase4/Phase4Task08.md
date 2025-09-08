Task 4.8: C++ Control Center Client - Guard UI Implementation
(Full Dynamic BUNKERGUARD Management, Rich 3D Preview, Secure Rich NAR Narratives - Principles H, J, K, P, O, R)
Technical Reference
Finalized P4 L3 Smart Contract ABIs (BunkerguardManager, NftContract)
CXX-Qt, QML (including Scene3D module) Documentation
Finalized v0.1 API Contracts (account_service.proto, indexer_service.proto)
docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md (for THE GUARD and its sub-screens)

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
The core of the BUNKERVERSE's in-verse progression is the dynamic Bunkerguard system, whose logic was implemented on-chain in Task 4.3. However, the client still has a placeholder UI for this system. To allow users to engage with this core feature, we must build the complete, fully functional "Guard" UI. This UI must leverage the live, dynamic data from the Indexer to provide a rich and responsive experience for managing the Bunkerguard, including a live 3D preview, a detailed stats panel, and a comprehensive inventory, all while featuring rich, contextual NAR narratives for all actions.
Measurable Objectives
The "Guard" UI section and its sub-screens (Appearance, Inventory, Logs) are fully implemented and functional.
The UI provides a live 3D preview of the player's Bunkerguard that accurately reflects all equipped items and cosmetic skins.
The UI displays the player's full, dynamic on-chain state (stats, class, affiliation) and updates in near real-time.
All Guard actions (equip, unequip, level up, repair) are functional and securely submit the correct transactions to the L3 smart contracts.
Implementation Guidance
Action: Implement the complete, fully functional "Guard" UI section (QML UI and Rust Application Logic). This UI must leverage the functional Rust Indexer to allow for full dynamic BUNKERGUARD management and display rich, contextual NAR narratives.
Implementation Details:
o ##### Rust Application Logic (/client/rust-app-logic/src/guard/handler.rs):
Implement methods exposed to QML for all Guard actions: fetch_full_robot_state, link_robot, unlink_robot, equip_item(nft_id, slot), unequip_item(slot), level_up_robot, repair_item(nft_id).
These methods will perform client-side validation, submit the appropriate transaction intents to the L3 sequencer, and query the functional Rust Indexer (via the Account Service) for the player's full, live ActiveBunkerguardDataProto and their full owned NFT inventory.
After a successful L3 transaction, it will trigger the Rust NAR with the rich context of the change (e.g., new class, new stats).
o ##### 3D Rendering Integration (C++ Shell / QML Scene3D):
The rustBackend QObject will expose the player's equipped_items list to QML.
The QML Scene3D component will have logic to dynamically load and attach 3D models corresponding to the NftId or ItemType of each equipped item. It will also apply CosmeticSkin materials. This provides a live 3D preview of the player's customized robot.
o ##### QML Frontend (/client/qml-ui/screens/Guard/ sub-screens):
AppearancePage.qml:
Displays the dynamic 3D preview.
Displays the robot's name, level, and current, dynamically calculated BunkerClass and ClassAffiliation (fetched from the Indexer via rustBackend).
Displays a detailed stats panel showing all 12 live sub-stats and 4 category averages, which update in real-time as gear is changed.
InventoryPage.qml:
Displays a comprehensive, filterable, and sortable list of all NFTs owned by the player, fetched from the live Indexer.
"Equip" and "Unequip" buttons are fully functional for all valid item slots, calling the respective rustBackend methods.
"View Details" opens a modal showing an item's full GDD-defined attributes.
A "Repair Item" button will be active for BrokenState items.
LogsPage.qml: A detailed log of all Guard-related activities (items equipped, class changes, level ups) with the corresponding rich, contextual NAR narrative for each entry, fetched from the Indexer.
o Real-time Updates: The entire Guard UI will use WebSocket signals to listen for changes from the Indexer, ensuring that if a stat/class/item change happens via another source (e.g., a game reward), the UI updates automatically.
Testing:
o E2E Tests (Critical - Staging with full Rust backend running in the local Docker Compose simulation):
Perform a full BUNKERGUARD lifecycle test for several of the 12 BunkerClasses.
Start with a new player. Equip a full set of GDD-defined items (e.g., "Corrupt Vanguard" gear set) one by one.
After each equip action, verify that the Guard UI correctly updates the 3D preview, all 12 sub-stats, the 4 category stats, and eventually the displayed BunkerClass and ClassAffiliation.
Verify a rich, class/affiliation/item-specific NAR narrative is generated for each significant change.
Test the repair and level-up functions and verify their XP costs and state changes are correct on-chain.
Update docs/progress_logs/progress_phase_4.md:
o Log the implementation of the full dynamic Guard UI. Detail the 3D preview integration. Document NAR prompts for all Guard actions, classes, and affiliations.
Design Rationale
The Guard UI is the primary interface for the core "in-verse" progression loop. A rich, responsive, and data-driven interface is essential for making this progression feel meaningful and engaging. The live 3D preview provides immediate visual feedback for personalization, while the real-time updates of on-chain stats give players a tangible sense of their Bunkerguard's power and identity.
Operational Considerations
Local-First: The 3D models and assets will be part of the client build. The entire UI and its interactions with the backend will be tested in the local Docker Compose simulation.
Cloud-Ready: The performance of the Indexer queries that back this UI (especially the full inventory fetch) will need to be monitored in production. Caching strategies in the Account Service will be crucial for maintaining UI responsiveness.
Verification & Validation Criteria
E2E tests confirm that the Guard UI provides full, dynamic BUNKERGUARD management for all 12 classes/affiliations.
All data displayed is driven by and consistent with the live L3/Indexer data.
The 3D previews are accurate.
NAR narratives are highly specific and contextually relevant.
Testing Methodologies
A combination of unit tests for the Rust handler logic and extensive, scripted manual E2E tests are required. The E2E tests are critical as they validate the complex interplay between the UI, the backend services, the on-chain logic, and the NAR.
Version Control Strategy
Branching: The Guard UI is a major feature set and will be developed on a feature/guard-ui-dynamic branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A security review of all transaction-creating functions in the Rust client logic is mandatory to ensure no exploits are possible (e.g., equipping an item to an invalid slot, repairing an item without sufficient XP).
The logic that fetches and renders 3D models must be reviewed to ensure it's not vulnerable to path traversal attacks if asset IDs are ever constructed from unsanitized data.
ReviewedBy: Client Lead, UI/UX Lead, L3 Smart Contracts Lead, Indexer Lead, AI Lead, Game Design Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: E2E tests confirm that the Guard UI provides full, dynamic BUNKERGUARD management, driven by live data, with accurate 3D previews and highly specific NAR narratives.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 4.8: Implemented CC Guard UI (Full Dynamic BUNKERGUARD System - All 12 Classes, All Item Details & Stats, Rich 3D Preview, Secure Rich NAR Narratives, Rust Backend Integration)." @Phase4/