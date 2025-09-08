Task 7.1: External Tester Recruitment, Onboarding & Secure Environment Access
(Final Cohort - Principles M, R)
Technical Reference
Finalized v0.9.9-stable client installers (from Task 6.2)
bunkerverse-admin-cli tool (for asset provisioning)
docs/legal/NDA_v1.pdf, docs/legal/Test_Agreement_v1.pdf
Context/Problem Statement
The MVE has been hardened and validated internally, but its true performance, stability, and usability can only be assessed by exposing it to a diverse group of external users with varied hardware and interaction patterns. This task covers the entire logistical process of finalizing the list of testers, legally onboarding them, securely distributing the client, and provisioning their accounts with the necessary assets to begin testing effectively.
Measurable Objectives
A final, diverse cohort of 50-200 external testers is confirmed and vetted.
100% of participating testers have electronically signed the required legal agreements (NDA, Test Agreement).
All onboarded testers successfully install the client, complete the zkLogin flow, and receive their standardized starting assets.
A mandatory kickoff session is successfully conducted for all participants.
Implementation Guidance
Action: Finalize recruitment, vet, and securely onboard the selected cohort of external testers. Provide them with secure access to the production-like staging environment (hosted on the local simulation machine) and all necessary materials, ensuring their data and the environment's integrity are protected.
1. Implementation Details:
a. Recruitment & Vetting: Confirm the final list of 50-200 testers. Ensure the cohort includes a diverse representation of hardware profiles (low-end to high-end PCs, different OSes, GPU vendors) and technical skill levels to test the platform and NAR performance under real-world conditions. Re-confirm their availability and commitment for the testing period.
b. Legal & NDAs: Ensure every external tester has electronically signed and returned a Non-Disclosure Agreement (NDA) and a Closed Test Participation Agreement. Use a secure digital signature service, and store the executed agreements securely.
c. Onboarding Materials & Secure Distribution:
* Distribute the finalized MVE overview document for testers, which clearly outlines the scope of the test, key platform features to focus on, and a link to the "Known Issues" list (from P6).
* Provide secure, unique download links for the platform-specific C++ Control Center Client production-candidate staging builds (signed/notarized Windows .exe, macOS .dmg, Linux .AppImage).
* The accompanying installation guide must include:
* Clear instructions for the Rust NAR's Gemma3 1B GGUF model download, which is automatically triggered by the client's nar-model-handler.
* Finalized minimum and recommended hardware specifications.
* A comprehensive troubleshooting FAQ for installation, NAR model problems, and zkLogin difficulties.
* Provide detailed instructions on using the designated feedback channels (private Discord, web-based bug tracker).
* Reiterate instructions for collecting and securely submitting client logs and system information.
d. Test Account & Asset Provisioning (on Staging Environment):
* Testers will use the live zkLogin flow on the staging environment with their own personal OAuth provider accounts.
* After a tester successfully registers, an automated script (or a community manager using the Admin CLI) will grant a standardized, identical set of starting test assets to their account by submitting transactions to the L3 Smart Contracts. This package will include:
* A fixed amount of test Credits for marketplace testing.
* A basic BunkerguardRobot NFT.
* A diverse starter set of Item NFTs across different rarities and types with varied but pre-defined GDD stats, traits, and affinities. This is crucial to allow testers to immediately experiment with the dynamic BunkerGuard class/affiliation system.
* This standard provisioning set must be meticulously documented.
e. Kickoff & Support Session for Testers: Host a mandatory live kickoff Discord session. This will include a detailed MVE walkthrough, a clear explanation of testing goals (functional testing, UI/UX feedback, NAR quality/performance assessment), and a demo of the feedback/bug reporting process.
2. Update docs/progress_logs/progress_phase_7.md:
Log the final tester count and demographics.
Document the versions and checksums of the client installers and NAR model GGUF used for the test.
Detail the standard asset provisioning package.
Summarize the kickoff session. Note adherence to Principle R for the secure handling of tester PII and NDAs.
Design Rationale
A structured and professional onboarding process is critical for a successful closed test. Providing clear documentation, standardized assets, and a live kickoff session ensures that all testers start from the same baseline and understand the goals of the test. This maximizes the quality and relevance of the feedback received. Securely distributing the client and handling legal agreements protects the project's intellectual property.
Operational Considerations
Local-First Simulation: The "staging environment" is the full MVE stack running in Docker Compose on the designated Windows 11 host machine. The network for this machine must be configured to allow secure, public access to the required service endpoints for the testers.
Asset Provisioning: The use of the Admin CLI to grant assets is a crucial test of the tool's utility in a live management scenario.
Verification & Validation Criteria
Confirmation from all testers of successful client installation, NAR model setup, zkLogin, and receipt of starter assets.
All testers are present in the designated private Discord channels.
The kickoff session is successfully conducted and recorded.
Testing Methodologies
N/A (Process execution and validation).
Version Control Strategy
Branching: All documentation (onboarding guides, etc.) will be finalized on a feature/external-test-prep branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The process for distributing client installers must be secure to prevent unauthorized access or distribution.
The handling and storage of personally identifiable information (PII) from testers and their signed legal agreements must comply with relevant data protection regulations (e.g., GDPR). This is a mandatory review checkpoint with Legal Counsel.
ReviewedBy: Community Lead, QA Lead, Legal Counsel, Project Manager, Security Lead.
ReviewOutcome: External Tester Onboarding Successfully Completed.
ValidationMethod: Confirmation from all testers of successful client installation, NAR model setup, zkLogin, receipt of starter assets, and active presence in the designated feedback channels. The kickoff session is successfully conducted and recorded.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 7.1: External Tester Recruitment & Onboarding Completed; Secure Staging Access & Materials Distributed." @Phase7/