Task 4.1: L3 Smart Contracts - Token Ledger Implementation
(Finalized, Secure & Robust for All Marketplace & BUNKERGUARD Costs - Principles I, J, N, R)
Technical Reference
Rust (w/ Arbitrum Stylus) Smart Contract Language Documentation
OpenZeppelin ERC20 and SafeMath library implementations
Finalized P4 GDDs for XP costs and Credit-based fees
Finalized L3 Smart Contract ABIs

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
The core economic loops of the MVE (marketplace trades, leveling up, item repair) rely on the secure and robust management of two key non-NTC assets: XP (earned via gameplay) and Credits (purchased with fiat). The logic for handling these token balances within the L3 smart contracts must be finalized and hardened against all potential economic exploits. This task focuses on implementing the full debit/credit logic for these tokens to support all defined costs from the GDDs.
Measurable Objectives
The token management logic within the L3 smart contracts is finalized and hardened.
The Credits token logic supports multi-party transfers for marketplace trades.
The XP token logic supports robust validation and burning for leveling and item repair.
All token-related logic passes a mandatory security code review for economic exploits.
Implementation Guidance
Action: Finalize and harden the token management logic within the L3 Smart Contracts. The logic to debit/credit Credits and XP will be fully implemented and tested.
Implementation Details (in BunkerguardManager.rs and Marketplace.rs):
o CreditsUpdated Logic: Finalize the internal _transfer_credits function.
This logic must support multi-party transfers for marketplace trades (debit buyer, credit seller, credit BUNKER_CORP fee account) within a single atomic transaction.
It must use a SafeMath library for all arithmetic to prevent integer overflow/underflow exploits.
It must include checks to prevent negative balances (require(balance >= amount)).
o XpBurned Logic: Finalize the internal _burn_xp function.
i. This function will robustly validate and burn XP for:
Robot Leveling: Based on a GDD-defined XP curve table that is stored on-chain in the smart contract. The level_up_robot transaction will check the required XP from this table.
Item Repair: Based on a GDD-defined formula (e.g., XP_Cost = BaseCost * RarityMultiplier * ConditionDegradationFactor). This formula will be implemented as a pure function within the smart contract.
o Transaction Validation (Hardened): When processing execute_trade, level_up_robot, repair_item transactions, the stateful validation must atomically verify sufficient Credits/XP based on the GDD-defined costs and fees before any state is changed.
Security Code Review:
o Review all token ledger smart contract code for potential race conditions, integer overflow/underflow exploits, and economic exploits (e.g., fee evasion, free XP/Credit generation through rounding errors). This is a critical security gate.
Update docs/progress_logs/progress_phase_4.md:
o Log refinements to the on-chain token ledger logic, including the implementation of fee calculations and XP cost formulas from the GDD.
o Document the outcomes of the security code review and any tests for economic exploits.
Design Rationale
Placing all token management logic directly within the L3 smart contracts is the only way to ensure the economic rules of the universe are truly canonical and immune to off-chain manipulation. Using a standard, audited SafeMath library is a non-negotiable security best practice for all financial calculations on-chain. Storing GDD-defined cost tables directly on-chain ensures that the rules of progression are as transparent and immutable as the ledger itself.
Operational Considerations
Local-First: All development and testing will be performed against the L3 smart contracts deployed to the local Arbitrum Orbit simulation.
Cloud-Ready: Smart contract logic is inherently cloud-ready. The GDD-defined cost tables will be deployed as part of the initial contract setup. Any future changes to these economic parameters (e.g., rebalancing XP costs) will require a formal, governed contract upgrade.
Verification & Validation Criteria
All unit and integration tests for the token logic pass successfully.
Complex multi-party marketplace trades correctly debit and credit all three parties (buyer, seller, treasury) in a single atomic transaction.
XP expenditures for leveling up and item repair are correctly calculated based on the on-chain GDD data and are debited from the user's balance.
Testing Methodologies
Unit Tests (Smart Contracts): Write exhaustive tests for the _transfer_credits and _burn_xp functions using a framework like Foundry or another Rust-compatible framework. Test all edge cases, including insufficient funds, zero-value transfers, and potential rounding errors.
Integration Tests: Create complex E2E test scripts (using the Admin CLI or a test harness) that simulate a full marketplace trade and verify that all three parties' Credit balances are correct afterward. Simulate a player earning XP and then spending it on leveling and repair, verifying the final XP balance is correct.
Version Control Strategy
Branching: The token ledger logic will be developed on a feature/l3-token-ledger branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory, exhaustive security code review of all new token-related smart contract logic is required. This is one of the highest-priority security reviews in the entire MVE.
The review must focus on preventing any possible economic exploit, no matter how minor. The Security Lead and the Game Economy Designer must both sign off.
ReviewedBy: L3 Smart Contracts Lead, Security Lead, Game Economy Designer.
ReviewOutcome: Approved.
ValidationMethod: Unit and integration tests pass for complex multi-party token exchanges and all defined XP expenditures.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 4.1: Finalized & Hardened L3 Smart Contract Token Ledger for All Marketplace & BUNKERGUARD XP/Credit Costs." @Phase4/