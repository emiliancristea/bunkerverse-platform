Task 7.3: Execution of Guided & Exploratory Closed Testing by External Testers
(Iterative Cycles with Staging Updates & Full Platform Validation - Principles O, P, R)
Technical Reference
* docs/testing/closed_alpha/guided_scenarios.md
* The project bug tracker and private Discord feedback channels
* The "Known Issues" document (from Task 6.5)
Context/Problem Statement
The external testers are now fully onboarded and have access to the staging environment. The primary objective of Phase 7 is to leverage this cohort to validate all MVE features under real-world conditions. This task outlines the execution of the testing itself, which is structured into iterative cycles. It combines structured, guided scenarios to ensure full feature coverage with unstructured, exploratory testing to uncover edge cases and unexpected issues that internal QA might miss. The focus is on validating all platform features, especially the full BunkerGuard dynamic system and its impact on the user experience and NAR narratives.
Measurable Objectives
* Achieve a high completion rate (>80%) for all guided scenarios by the tester cohort.
* Generate a significant volume of structured bug reports and qualitative feedback in the bug tracker.
* Successfully complete at least one iterative cycle, where feedback is collected, critical bugs are fixed and deployed to staging, and testers validate the fixes.
* Gather comprehensive performance data and narrative quality feedback across a diverse range of hardware.
Implementation Guidance
Action: The external tester cohort will execute comprehensive guided scenarios and perform extensive exploratory testing on the production-like local simulation staging environment. The testing will be structured into iterative cycles (e.g., one or two 1-week cycles), with staging updates deployed between cycles to address critical feedback. The focus is on validating all MVE platform features, especially the full BunkerGuard class/item/stat system and its impact on the NAR narratives.
* Guided Scenario Execution (by External Testers):
* Testers will be tasked with systematically executing all user flows defined in a new document: docs/testing/closed_alpha/guided_scenarios.md. This will ensure full feature coverage. Scenarios include:
* Onboarding: Successful zkLogin with at least two different OAuth providers.
* Mission System: Accept an in-verse mission (e.g., "Post a guide"), meet the objective in the ARCHIVES, and claim the on-chain reward in the CAMP.
* BUNKERGUARD Management (Guard UI):
* Using the provided starter items, equip a full set of gear to achieve a specific target BunkerClass.
* Verify that the UI correctly displays the new class, affiliation, and the dynamically calculated stats.
* Test the item repair flow for an item that degrades in condition.
* Marketplace (Full Flow):
* List one of their starter items for sale.
* Use the search and detailed filters to find a specific type of item.
* Purchase an item from another test player.
* Test the Stripe credit purchase flow using provided test credit card numbers.
* Social UI: Add at least two other testers as on-chain friends and exchange messages in the real-time chat.
* Rust NAR Narrative Evaluation (Deep Dive): For all the above activities, testers will be asked to specifically rate and comment on the local Rust NAR narratives they receive. They will assess relevance, quality, variety, and performance on their specific hardware.
* Exploratory Testing & Edge Case Hunting:
o After completing the guided scenarios, testers will be encouraged to perform unscripted, exploratory testing.
o The goal is to push the system's boundaries, test unusual item/class combinations, and attempt to find economic exploits that were not caught by internal testing.
* Hardware & Platform Variety Feedback: Testers will be explicitly asked to report on client performance, Rust NAR inference speed, and overall stability, and to mention their OS, CPU, and GPU in such reports.
* Regular Tester Feedback Sessions & Surveys (Targeted):
o At the end of each testing cycle, conduct a live feedback session on Discord and send out a structured survey.
o The survey will ask targeted questions to gather quantitative and qualitative data:
* "On a scale of 1-5, how intuitive did you find the Marketplace UI?"
* "Did the dynamic BunkerClass system feel intuitive and rewarding? Why or why not?"
* "Please provide an example of a NAR narrative you found particularly good or bad."
* "Did you experience any performance issues with the client or the AI narratives? If so, please describe them and your PC specs."
* Update docs/progress_logs/progress_phase_7.md:
o For each completed external testing cycle, log its duration, number of active testers, and a summary of the scenarios covered. Link to a high-level report summarizing the key bugs, UI/UX feedback themes, and narrative quality assessments.
Design Rationale
A combination of guided and exploratory testing provides the best of both worlds. Guided scenarios ensure that every critical feature is tested by the entire cohort, providing a broad set of data. Exploratory testing leverages the creativity and diverse perspectives of the testers to find unexpected issues. Iterative cycles allow us to be responsive to critical feedback, deploying fixes and having them validated by the same users who found the problems, which is a highly effective quality assurance loop.
Operational Considerations
The core team (especially QA, Community, and SRE/DevOps) will need to be on high alert during the testing cycles to provide support, monitor the environment, and manage the feedback channels. The staging environment, running on the local Windows 11 machine, must be monitored closely for performance degradation under the load of ~50-200 concurrent users.
Verification & Validation Criteria
* High completion rate of guided scenarios by testers.
* A significant volume of structured bugs and qualitative feedback is submitted to the bug tracker.
* A high response rate on end-of-cycle surveys, providing actionable data.
* At least one iterative hotfix cycle is successfully completed.
Testing Methodologies
* System Testing: The entire MVE platform is tested by external users in a production-like environment.
* Usability Testing: Feedback on UI/UX is a primary goal.
* Performance Testing: Performance data is collected from a wide variety of real-world hardware configurations.
Version Control Strategy
* Branching: Bug fixes identified during this phase will be developed in hotfix/ branches and merged into a dedicated staging branch for deployment.
* Commits: The Git Commit message for this task will be exactly as specified, tagged with the cycle number.
Security Audit & Compliance Checkpoints
* All security-related bug reports submitted through the secure channel (Task 7.2) will be treated as the highest priority (P0) for the iterative fixing cycle (Task 7.5).
* The Security Lead will monitor for any reports of potential economic or system exploits found during exploratory testing.
ReviewedBy: QA Lead, Product Owner, All Dev Leads, Game Design Lead, Narrative Designer.
ReviewOutcome: External Testing Cycle [N] Completed, Rich Feedback on Full Platform Mechanics & Performance Gathered.
ValidationMethod: High completion rate of guided scenarios by testers. A significant volume of bugs and feedback submitted to the tracker. High response rate on end-of-cycle surveys.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 7.3: External Closed Testing Cycle [N] Execution and Rich Platform Mechanics Feedback Collection Completed." @Phase7/

------------------------------------------------------------------------------------------------------------------

