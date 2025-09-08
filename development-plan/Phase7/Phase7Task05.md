Task 7.5: Feedback Triage, Prioritization & Iterative Hotfixing for Staging
(Based on External Tester Feedback - High Urgency for P0/P1 - Principles F, R)
Technical Reference
Project Issue Tracker (e.g., Jira)
Staging CI/CD Deployment Pipeline (.github/workflows/staging-deploy.yml)
docs/INCIDENT_RESPONSE_PLAN_DRAFT.md (for security vulnerability response)
Context/Problem Statement
The external closed test is generating a continuous stream of bug reports, performance feedback, and potential security concerns. To maximize the value of this testing phase, we must have a rapid and disciplined process for collecting, triaging, prioritizing, and fixing the most critical issues. This task defines the operational loop for converting external feedback into validated, iterative updates deployed to the staging environment, ensuring that we actively improve the product based on the invaluable data our testers provide.
Measurable Objectives
A daily triage process for all externally reported issues is successfully maintained.
All identified P0 (Blocker) bugs are fixed and deployed to staging within a target 24-48 hour turnaround.
All identified P1 (Critical/High) bugs are fixed and deployed to staging in bundled updates between testing cycles.
All potential security vulnerabilities are handled with the highest priority according to the defined response protocol.
Implementation Guidance
Action: Implement a rapid, continuous cycle of collecting, triaging, prioritizing, and fixing P0 (Blocker) and P1 (Critical/High) bugs reported by external testers. The highest urgency will be placed on issues affecting core platform functionality, economic integrity, security, client stability, or Rust NAR usability/performance on common hardware. Validated fixes will be deployed iteratively to the staging environment.
Process Details:
o Daily Triage Meetings (Cross-Functional): Continue the daily triage meetings with QA, Community, Product, and Dev Leads. The primary input is now the externally reported bugs from the bug tracker and Discord.
o Prioritization Strategy (Strict & Externally Focused):
P0 (Blocker/Emergency Hotfix): Issues that must be fixed immediately (target <24-48h turnaround) to unblock the testing process. Examples:
Widespread client crashes, especially those related to the Rust NAR FFI or llama.cpp.
Inability for a significant segment of testers to install, launch, or complete zkLogin.
An exploit allowing item/currency duplication in the L3 Smart Contracts.
Any confirmed security vulnerability reported by a tester.
Core platform loops (marketplace, guard, archives) being non-functional.
P1 (High/Next Staging Update): Major issues that significantly degrade the experience. Examples:
Significant L3 Smart Contract data inconsistencies that are visible to the user via the Indexer.
Frequent generation of poor, irrelevant, or nonsensical Rust NAR narratives.
Severe NAR performance issues (e.g., >30-second freezes) on common tester hardware.
Major UI bugs blocking access to important features.
o Deployment of Fixes to Staging:
Use the CI/CD pipeline to build and deploy hotfixes (for P0 bugs) or bundled updates (for P1 bugs) to the staging environment.
Communicate clearly with testers via the private Discord, providing clear patch notes that highlight which of their reported issues have been fixed. Request specific re-testing of the fixed issues.
o Narrative Iteration (Final Tuning): This is the last major opportunity to tune the Rust NAR system based on widespread external feedback. This involves tweaking prompt templates in the client's Rust logic and potentially adjusting llama.cpp generation parameters to address feedback on narrative quality and performance. Any reported offensive or nonsensical outputs must be addressed immediately as a P1 bug.
o Security Vulnerability Response: Any potential security vulnerability reported by testers is immediately escalated to the Security Lead, investigated with the highest priority, and a patch is developed and deployed to staging as a P0 hotfix if confirmed. The process follows the responsible disclosure protocol.
Testing:
o Internal QA must rigorously test all bug fixes (including regression testing the affected area) before they are deployed to the staging environment for external testers to validate.
Update docs/progress_logs/progress_phase_7.md:
o Maintain a detailed log of all P0/P1 bugs reported by external testers, linking to the issue tracker, summarizing the fix, and documenting the verification process.
o Document the final iterations on Rust NAR prompts based on tester feedback.
o Log any security vulnerabilities reported and their remediation.
Design Rationale
An iterative hotfixing cycle is the most effective way to manage a closed test. It allows the team to be highly responsive to critical issues, keeping the testers unblocked and engaged. It also provides an opportunity to validate fixes with the same users who discovered the problems, which is a powerful form of regression testing. Prioritizing based on severity ensures that engineering effort is always focused on the most impactful issues first.
Operational Considerations
This process requires tight coordination between the Community, QA, and Development teams. The CI/CD pipeline for the staging environment must be robust and efficient to support rapid hotfix deployments. Clear and consistent communication with the tester cohort is essential to manage expectations and direct re-testing efforts.
Verification & Validation Criteria
The bug tracker shows P0/P1 tester-reported bugs moving from "Open" to "Resolved" (fixed by dev) and then to "Verified" (confirmed fixed by QA and/or the original reporter).
Positive feedback trends are observed from testers on the fixed issues in the private Discord.
NAR quality and performance improvements are validated by tester feedback through surveys and direct commentary.
Testing Methodologies
Regression Testing: Every bug fix must be accompanied by a new automated test (if possible) and must be manually tested by internal QA to ensure it resolves the issue and does not introduce new ones.
User Acceptance Testing (UAT): External testers re-testing the deployed fixes serves as the final UAT for each patch.
Version Control Strategy
Branching: Fixes will be developed on hotfix/ or bugfix/ branches based off the current staging branch. These are then merged into staging for deployment. Periodically, staging is merged back into develop.
Commits & Tagging: Use regular commits for fixes. Tag staging builds deployed to testers (e.g., v0.9.9-staging-beta2) to track versions.
Security Audit & Compliance Checkpoints
The Security Lead is a mandatory participant in the daily triage process and has final authority on the prioritization and handling of any security-related bug.
The responsible disclosure protocol for confirmed vulnerabilities must be strictly followed.
ReviewedBy: QA Lead, Product Owner, Dev Leads, Narrative Designer, Security Lead.
ReviewOutcome: Iterative Fixes Deployed to Staging, Critical External Tester Issues Addressed.
ValidationMethod: The bug tracker shows P0/P1 tester-reported bugs moving to "Resolved" and "Verified" states. Positive feedback trends are observed from testers on the fixed issues. NAR quality and performance improvements are validated by tester feedback.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 7.5: Iterative Bug Fixing and Staging Updates from External Closed Testing Cycle [N]." @Phase7/