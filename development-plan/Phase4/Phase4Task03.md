Task 4.3: L3 Smart Contracts - BUNKERGUARD Robot Dynamic Progression Logic
(Full Dynamic Stat/Class/Affiliation System based on Equipped Items - Principles I, J, N, R)
Technical Reference
Finalized P4 GDDs (containing the precise mathematical formulas for all dynamic calculations).
Rust (w/ Arbitrum Stylus) Smart Contract Language Documentation.
Finalized L3 Smart Contract ABIs, particularly BunkerguardManager.rs and NftContract.rs.
Gas optimization best practices for the target EVM/WASM environment.

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
The Bunkerguard Robot, the player's core in-verse identity, currently has a static, default state. To create the deep, engaging, and economically meaningful progression system at the heart of the BUNKERVERSE, this state must become fully dynamic. This is a critical and highly complex task. It requires implementing the on-chain logic that dynamically calculates a player's stats, their BunkerClass, and their ClassAffiliation based on the specific combination of items they have equipped, their robot's level, and their reputation.
Measurable Objectives
The equip_item and unequip_item functions in the BunkerguardManager smart contract are fully implemented with the complete, GDD-defined recalculation logic.
All 12 sub-stats, 12 BunkerClasses, and 3 ClassAffiliations are correctly and dynamically calculated on-chain.
The entire recalculation process, including all state updates and event emissions, occurs within a single, atomic transaction.
The implementation is gas-optimized and passes an exhaustive security audit for logical and economic exploits.
Implementation Guidance
Action: This is a critical task. Fully implement the dynamic BUNKERGUARD Robot progression logic in the L3 Smart Contracts. All stats, all 12 BunkerClasses, and the 3 ClassAffiliations must now be dynamically calculated and updated in the on-chain AgentChainState based on all equipped items (fetching their full attributes from the canonical on-chain store), robot level, and player reputation points.

## Mathematical Formulas for Dynamic Progression

### Base Stat Calculation
```rust
// Level-based base stats (stored on-chain)
fn get_base_stats(level: u32) -> CoreStatsProto {
    CoreStatsProto {
        // Combat stats
        firepower: 10 + (level * 2),
        precision: 8 + (level * 1.5).floor(),
        impact: 7 + (level * 1.8).floor(),
        
        // Mobility stats  
        agility: 9 + (level * 1.6).floor(),
        speed: 10 + (level * 1.4).floor(),
        evasion: 6 + (level * 1.7).floor(),
        
        // Survivability stats
        armor: 12 + (level * 2.2).floor(),
        resistance: 8 + (level * 1.9).floor(),
        vitality: 15 + (level * 2.5).floor(),
        
        // Sensor stats
        awareness: 7 + (level * 1.3).floor(),
        detection: 6 + (level * 1.2).floor(),
        analysis: 5 + (level * 1.1).floor(),
    }
}
```

### Item Stat Contribution with Condition Modifier
```rust
fn calculate_item_contribution(item: &NftDetailsProto, condition: ItemConditionProto) -> CoreStatsProto {
    let condition_multiplier = match condition {
        ItemConditionProto::PRIME_STATE => 1.25,  // 125% of base stats
        ItemConditionProto::NEW_STATE => 1.0,     // 100% of base stats
        ItemConditionProto::USED_STATE => 0.75,   // 75% of base stats
        ItemConditionProto::BROKEN_STATE => 0.25, // 25% of base stats
    };
    
    let rarity_multiplier = match item.rarity {
        ItemRarityProto::STANDARD => 1.0,
        ItemRarityProto::OPTIMIZED => 1.15,
        ItemRarityProto::ADVANCED => 1.35,
        ItemRarityProto::SUPREME => 1.6,
        ItemRarityProto::ECHELON => 2.0,
        ItemRarityProto::ETERNAL => 2.5,
    };
    
    // Apply both multipliers to base stats
    let final_multiplier = condition_multiplier * rarity_multiplier;
    item.base_stat_boosts.multiply(final_multiplier)
}
```

### Category Average Calculation
```rust
fn calculate_category_averages(stats: &CoreStatsProto) -> CategoryAverages {
    CategoryAverages {
        combat_avg: ((stats.firepower + stats.precision + stats.impact) / 3.0).round(),
        mobility_avg: ((stats.agility + stats.speed + stats.evasion) / 3.0).round(),
        survivability_avg: ((stats.armor + stats.resistance + stats.vitality) / 3.0).round(),
        sensors_avg: ((stats.awareness + stats.detection + stats.analysis) / 3.0).round(),
    }
}
```

### Stat Caps (Level-Dependent)
```rust
fn apply_stat_caps(stats: &mut CoreStatsProto, level: u32) {
    let max_stat = 100 + (level * 5); // Max increases with level
    let min_stat = 1;
    
    // Apply caps to all 12 sub-stats
    stats.firepower = stats.firepower.clamp(min_stat, max_stat);
    stats.precision = stats.precision.clamp(min_stat, max_stat);
    // ... apply to all stats
}
```

### Class Assignment Logic
```rust
fn determine_class(
    dominant_category: StatCategoryProto,
    class_affinities: HashMap<BunkerClassProto, u32>,
    current_affiliation: ClassAffiliationProto
) -> BunkerClassProto {
    // Step 1: Filter classes by dominant stat category
    let category_classes = match dominant_category {
        StatCategoryProto::COMBAT => vec![
            BunkerClassProto::VANGUARD,
            BunkerClassProto::ENFORCER,
            BunkerClassProto::BREACHER,
        ],
        StatCategoryProto::MOBILITY => vec![
            BunkerClassProto::EXPLORER,
            BunkerClassProto::PATHFINDER,
            BunkerClassProto::STALKER,
        ],
        StatCategoryProto::SURVIVABILITY => vec![
            BunkerClassProto::SCAVENGER,
            BunkerClassProto::RECLAIMER,
            BunkerClassProto::OVERLORD,
        ],
        StatCategoryProto::SENSORS => vec![
            BunkerClassProto::CYBERMANCER,
            BunkerClassProto::CODEBREAKER,
            BunkerClassProto::DISRUPTOR,
        ],
    };
    
    // Step 2: Find class with highest affinity points
    let mut best_class = category_classes[0];
    let mut highest_affinity = 0;
    
    for class in category_classes {
        let affinity = class_affinities.get(&class).unwrap_or(&0);
        if *affinity > highest_affinity {
            highest_affinity = *affinity;
            best_class = class;
        }
    }
    
    // Step 3: Tie-breaker based on affiliation
    if highest_affinity == 0 {
        // No clear affinity, use affiliation-based defaults
        match current_affiliation {
            ClassAffiliationProto::LOYAL => category_classes[0],
            ClassAffiliationProto::CORRUPT => category_classes[2],
            ClassAffiliationProto::NEUTRAL => category_classes[1],
        }
    } else {
        best_class
    }
}
```

### Affiliation Assignment Formula
```rust
fn calculate_affiliation(
    loyal_item_count: u32,
    corrupt_item_count: u32,
    reputation_loyal: i64,
    reputation_corrupt: i64
) -> ClassAffiliationProto {
    // Weight items and reputation
    let item_weight = 10; // Each item trait counts as 10 points
    let rep_weight = 1;   // Each reputation point counts as 1
    
    let loyal_score = (loyal_item_count * item_weight) as i64 + (reputation_loyal * rep_weight);
    let corrupt_score = (corrupt_item_count * item_weight) as i64 + (reputation_corrupt * rep_weight);
    
    // Threshold for neutral (within 20% of each other)
    let difference = (loyal_score - corrupt_score).abs();
    let total = loyal_score.abs() + corrupt_score.abs();
    let neutral_threshold = (total as f64 * 0.2) as i64;
    
    if difference <= neutral_threshold {
        ClassAffiliationProto::NEUTRAL
    } else if loyal_score > corrupt_score {
        ClassAffiliationProto::LOYAL
    } else {
        ClassAffiliationProto::CORRUPT
    }
}
```
Implementation Details (in BunkerguardManager.rs L3 Smart Contract):
ItemEquipIntent / ItemUnequipIntent Processing: When a public function like equip_item is called, the following internal Recalculation Logic is triggered after validating ownership and slot compatibility:
i. Fetch All Equipped Item Details: The smart contract logic now makes read-calls to its own canonical, global NftDetailsProto and NftMutableStateProto storage mappings for every single item in the player's equipped_items list.
ii. Full Stat Recalculation:
* Start with the robot's base stats (from its level, based on a GDD table stored on-chain).
* Iterate through all equipped items. For each item:
o Get its current_condition from its NftMutableStateProto.
o Apply the GDD-defined condition modifier to the base_stat_boosts from its NftDetailsProto.
o Add the condition-modified stat boosts to the running totals for all 12 sub-stats, using a SafeMath library.
* Calculate the 4 main StatCategory averages from the 12 sub-stat totals.
* Apply the GDD-defined stat caps.
* Compare the newly calculated CoreStatsProto with the one in the player's current state. If different, prepare a RobotStatsUpdated event.
iii. Full Class Assignment:
* Using the newly calculated final_stats, identify the dominant StatCategory.
* Sum all class_affinity points from all equipped items' NftDetailsProto.
* Apply the GDD's class assignment logic (which will be implemented as a pure function or lookup table within the contract), using the dominant stat category, class affinity points, and current ClassAffiliation to determine the specific BunkerClassProto.
* If the new class differs from the player's current class, prepare a ClassAssigned event.
iv. Full Affiliation Assignment:
* Count the Loyal vs. Corrupt traits from all equipped items.
* Factor in the player's reputation_points_loyal and reputation_points_corrupt from their current state.
* Apply the GDD's affiliation logic.
* If the new affiliation differs from the current one, prepare an AffiliationAssigned event.
v. Atomic Event Generation: Atomically emit the ItemEquipped/ItemUnequipped event, plus any RobotStatsUpdated, ClassAssigned, and AffiliationAssigned events that resulted from the recalculation.
Security Code Review:
o Focus on the logic for fetching and using NftDetails during calculations to prevent any re-entrancy or data manipulation exploits.
o Review the stat, class, and affiliation rule implementations for integrity and to prevent unintended "super states" not foreseen by the GDD.
Update docs/progress_logs/progress_phase_4.md:
o Log the full implementation of the dynamic stat/class/affiliation system.
o Document the GDD rules/formulas as they were implemented in the smart contract code.
Design Rationale
Placing this complex logic on-chain is the ultimate expression of the "On-Chain in Substance" doctrine. It makes a player's identity-their very class and power level-a trustless, verifiable, and canonical fact on the ledger. This creates an incredibly deep and strategic "meta-game" around item collection and builds, where the on-chain state is the definitive record of a player's mastery of the system.
Operational Considerations
Gas Costs: This will be one of the most gas-intensive transactions in the MVE. The logic must be heavily optimized to minimize loops and storage reads/writes. The gas cost for equipping a full set of gear will be a key performance metric to track.

### Gas Optimization Targets
- Single item equip: < 150,000 gas
- Full gear set (6 items) equip: < 500,000 gas
- Stat recalculation only: < 50,000 gas
- Class reassignment: < 30,000 gas

### Optimization Strategies
```rust
// Cache storage reads
let mut equipped_cache: HashMap<String, NftDetailsProto> = HashMap::new();
for (slot, item_id) in player.equipped_items.iter() {
    if let Some(details) = self.nft_details.get(item_id) {
        equipped_cache.insert(slot.clone(), details);
    }
}

// Use packed structs to minimize storage slots
#[packed]
struct PackedStats {
    // Pack 12 u16 stats into 3 storage slots (256 bits each)
    combat_stats: u256,      // firepower, precision, impact
    mobility_stats: u256,     // agility, speed, evasion
    survival_stats: u256,     // armor, resistance, vitality
    sensor_stats: u256,       // awareness, detection, analysis
}

// Batch event emissions
let mut events = Vec::new();
if stats_changed { events.push(Event::StatsUpdated(new_stats)); }
if class_changed { events.push(Event::ClassAssigned(new_class)); }
if affiliation_changed { events.push(Event::AffiliationChanged(new_affiliation)); }
self.emit_batch(events); // Single storage write for all events
```
GDD Updates: Any future changes to the stat calculation formulas or class assignment rules will require a governed smart contract upgrade.
Verification & Validation Criteria
Extensive integration tests simulating players equipping various full sets of GDD-defined items dynamically and correctly trigger each of the 12 BunkerClasses and 3 Affiliations.
The final on-chain state (AgentChainState) is verified to be 100% correct according to the GDD formulas after each change.
The transaction gas cost is within acceptable limits for a "gasless" experience sponsored by the Paymaster.
Testing Methodologies
Unit Tests (Smart Contracts): Exhaustive tests for the recalculation logic using a Rust-compatible framework like Foundry. Create test cases for every BunkerClass and Affiliation, providing the exact set of items needed to trigger them and asserting that the final calculated state is correct. Test all edge cases (empty inventory, stat caps, etc.).

### Comprehensive Test Cases

```rust
#[test]
fn test_vanguard_class_assignment() {
    // Setup: Create items that boost combat stats
    let items = vec![
        create_item(ItemType::HEAD, rarity: SUPREME, combat_boost: 50),
        create_item(ItemType::TORSO, rarity: ADVANCED, combat_boost: 35),
        create_item(ItemType::GEAR, class_affinity: VANGUARD, points: 25),
    ];
    
    let player = equip_items(player, items);
    assert_eq!(player.current_class, BunkerClassProto::VANGUARD);
    assert!(player.stats.combat_avg > 150);
    assert!(total_gas_used() < 500_000);
}

#[test]
fn test_affiliation_neutral_threshold() {
    // Setup: Balanced loyal and corrupt items
    let items = vec![
        create_item(trait: LOYAL),
        create_item(trait: LOYAL),
        create_item(trait: CORRUPT),
        create_item(trait: CORRUPT),
    ];
    
    let player = equip_items(player, items);
    player.reputation_loyal = 15;
    player.reputation_corrupt = 18;
    
    let affiliation = calculate_affiliation(player);
    assert_eq!(affiliation, ClassAffiliationProto::NEUTRAL);
}

#[test]
fn test_condition_degradation() {
    let item = create_item(base_firepower: 100, condition: PRIME_STATE);
    assert_eq!(calculate_boost(item), 125); // 100 * 1.25
    
    item.condition = BROKEN_STATE;
    assert_eq!(calculate_boost(item), 25); // 100 * 0.25
}

#[test]
fn test_stat_cap_enforcement() {
    let mut stats = CoreStatsProto {
        firepower: 999,
        precision: 999,
        // ...
    };
    
    apply_stat_caps(&mut stats, level: 20);
    assert_eq!(stats.firepower, 200); // 100 + (20 * 5)
    assert_eq!(stats.precision, 200);
}
```
Integration Tests: Use the Admin CLI to mint full sets of gear to a test player's account. Then, use the client to log in and equip the items one by one. After each equip transaction, use the CLI's state-derive command to query the on-chain state and compare it against a pre-calculated expected result.
Version Control Strategy
Branching: The dynamic progression logic will be developed on a feature/l3-dynamic-progression branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory, in-depth security audit of the BunkerguardManager contract, with a specific focus on the recalculation logic, is required before this phase can be completed.
The audit must scrutinize the logic for potential economic exploits, integer overflows/underflows, re-entrancy, and logical flaws that could lead to unintended "god-mode" gear combinations. The Game Design Lead must be part of this review to validate the implementation against their economic model.
ReviewedBy: L3 Smart Contracts Lead, Game Design Lead (mandatory for rules/formulas), Security Lead, Backend Lead.
ReviewOutcome: Approved.
ValidationMethod: Extensive integration tests simulating players equipping various full sets of GDD-defined items dynamically trigger each of the 12 BunkerClasses and 3 Affiliations. The on-chain state is verified to be correct after each change.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 4.3: Implemented L3 Smart Contract Full Dynamic Stat/Class/Affiliation System (All 12 Classes, Based on Live Equipped Item Details)." @Phase4/