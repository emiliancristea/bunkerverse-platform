//! Validation utilities for BUNKERVERSE Platform
//! Centralized validation logic for UUIDs, addresses, and other common formats

use regex::Regex;
use std::sync::OnceLock;

use crate::errors::{Result, ValidationError};

// ============================================================================
// Validation Constants
// ============================================================================

/// UUID v4 validation regex pattern
const UUID_V4_PATTERN: &str = r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$";

/// Ethereum address validation regex pattern
const ETHEREUM_ADDRESS_PATTERN: &str = r"^0x[a-fA-F0-9]{40}$";

/// IPFS CID validation regex pattern (basic QM format)
const IPFS_CID_PATTERN: &str = r"^Qm[1-9A-HJ-NP-Za-km-z]{44}$";

/// Email validation regex pattern (basic)
const EMAIL_PATTERN: &str = r"^[^\s@]+@[^\s@]+\.[^\s@]+$";

/// Username/BunkerTag validation regex pattern
const USERNAME_PATTERN: &str = r"^[a-zA-Z0-9_-]{3,32}$";

// ============================================================================
// Regex Compilation (Thread-Safe Lazy Static)
// ============================================================================

static UUID_REGEX: OnceLock<Regex> = OnceLock::new();
static ETHEREUM_REGEX: OnceLock<Regex> = OnceLock::new();
static IPFS_CID_REGEX: OnceLock<Regex> = OnceLock::new();
static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
static USERNAME_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_uuid_regex() -> &'static Regex {
    UUID_REGEX.get_or_init(|| Regex::new(UUID_V4_PATTERN).unwrap())
}

fn get_ethereum_regex() -> &'static Regex {
    ETHEREUM_REGEX.get_or_init(|| Regex::new(ETHEREUM_ADDRESS_PATTERN).unwrap())
}

fn get_ipfs_cid_regex() -> &'static Regex {
    IPFS_CID_REGEX.get_or_init(|| Regex::new(IPFS_CID_PATTERN).unwrap())
}

fn get_email_regex() -> &'static Regex {
    EMAIL_REGEX.get_or_init(|| Regex::new(EMAIL_PATTERN).unwrap())
}

fn get_username_regex() -> &'static Regex {
    USERNAME_REGEX.get_or_init(|| Regex::new(USERNAME_PATTERN).unwrap())
}

// ============================================================================
// Core Validation Functions
// ============================================================================

/// Validate UUID v4 format
pub fn validate_uuid_v4(uuid: &str) -> Result<()> {
    if !get_uuid_regex().is_match(uuid) {
        return Err(ValidationError::invalid_uuid(uuid));
    }
    Ok(())
}

/// Validate Ethereum address format
pub fn validate_ethereum_address(address: &str) -> Result<()> {
    if !get_ethereum_regex().is_match(address) {
        return Err(ValidationError::invalid_ethereum_address(address));
    }
    Ok(())
}

/// Validate IPFS CID format (basic QM format)
pub fn validate_ipfs_cid(cid: &str) -> Result<()> {
    if !get_ipfs_cid_regex().is_match(cid) {
        return Err(ValidationError::InvalidFormat(format!(
            "Invalid IPFS CID format: {}",
            cid
        )));
    }
    Ok(())
}

/// Validate email format
pub fn validate_email(email: &str) -> Result<()> {
    if !get_email_regex().is_match(email) {
        return Err(ValidationError::InvalidFormat(format!(
            "Invalid email format: {}",
            email
        )));
    }
    Ok(())
}

/// Validate username/BunkerTag format
pub fn validate_username(username: &str) -> Result<()> {
    if !get_username_regex().is_match(username) {
        return Err(ValidationError::InvalidFormat(
            format!("Invalid username format: {} (must be 3-32 chars, alphanumeric, underscore, hyphen only)", username)
        ));
    }
    Ok(())
}

/// Validate string length
pub fn validate_string_length(field: &str, value: &str, min: usize, max: usize) -> Result<()> {
    let length = value.len();
    if length < min || length > max {
        return Err(ValidationError::InvalidLength {
            field: field.to_string(),
            actual: length,
            min,
            max,
        });
    }
    Ok(())
}

/// Validate numeric range
pub fn validate_numeric_range<T>(field: &str, value: T, min: T, max: T) -> Result<()>
where
    T: PartialOrd + std::fmt::Display + Copy,
{
    if value < min || value > max {
        return Err(ValidationError::OutOfRange {
            field: field.to_string(),
            value: value.to_string(),
            min: Some(min.to_string()),
            max: Some(max.to_string()),
        });
    }
    Ok(())
}

/// Validate timestamp is reasonable (> 2021-01-01, < current + 1 day)
pub fn validate_timestamp(timestamp: i64) -> Result<()> {
    const MIN_TIMESTAMP: i64 = 1609459200; // 2021-01-01 00:00:00 UTC
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let max_timestamp = current_time + 86400; // Current time + 1 day

    if timestamp < MIN_TIMESTAMP {
        return Err(ValidationError::InvalidTimestamp {
            timestamp,
            reason: "Timestamp too old (before 2021)".to_string(),
        });
    }

    if timestamp > max_timestamp {
        return Err(ValidationError::InvalidTimestamp {
            timestamp,
            reason: "Timestamp too far in future (> 1 day)".to_string(),
        });
    }

    Ok(())
}

/// Validate required field is not empty
pub fn validate_required_string(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(ValidationError::required_field(field));
    }
    Ok(())
}

/// Validate required field is present
pub fn validate_required_field<T>(field: &str, value: &Option<T>) -> Result<()> {
    if value.is_none() {
        return Err(ValidationError::required_field(field));
    }
    Ok(())
}

// ============================================================================
// Composite Validators
// ============================================================================

/// Validate player ID (UUID v4 + not empty)
pub fn validate_player_id(player_id: &str) -> Result<()> {
    validate_required_string("player_id", player_id)?;
    validate_uuid_v4(player_id)?;
    Ok(())
}

/// Validate NFT ID (UUID v4 + not empty)
pub fn validate_nft_id(nft_id: &str) -> Result<()> {
    validate_required_string("nft_id", nft_id)?;
    validate_uuid_v4(nft_id)?;
    Ok(())
}

/// Validate BunkerTag (username format + length)
pub fn validate_bunker_tag(bunker_tag: &str) -> Result<()> {
    validate_required_string("bunker_tag", bunker_tag)?;
    validate_string_length("bunker_tag", bunker_tag, 3, 32)?;
    validate_username(bunker_tag)?;
    Ok(())
}

/// Validate display name (length + not empty)
pub fn validate_display_name(display_name: &str) -> Result<()> {
    validate_required_string("display_name", display_name)?;
    validate_string_length("display_name", display_name, 1, 64)?;
    Ok(())
}

/// Validate bio text (length)
pub fn validate_bio(bio: &str) -> Result<()> {
    validate_string_length("bio", bio, 0, 256)?;
    Ok(())
}

/// Validate message content (length + not empty)
pub fn validate_message_content(content: &str) -> Result<()> {
    validate_required_string("message_content", content)?;
    validate_string_length("message_content", content, 1, 2000)?;
    Ok(())
}

/// Validate player level (1-100)
pub fn validate_player_level(level: u32) -> Result<()> {
    validate_numeric_range("player_level", level, 1, 100)?;
    Ok(())
}

/// Validate stat value (0-1000)
pub fn validate_stat_value(stat_name: &str, value: u32) -> Result<()> {
    validate_numeric_range(stat_name, value, 0, 1000)?;
    Ok(())
}

/// Validate XP amount (0 to MAX_XP)
pub fn validate_xp_amount(xp: u64) -> Result<()> {
    const MAX_XP: u64 = 1_000_000_000; // 1 billion
    validate_numeric_range("xp", xp, 0, MAX_XP)?;
    Ok(())
}

/// Validate NTC amount in wei (reasonable bounds)
pub fn validate_ntc_amount_wei(amount_wei: u64) -> Result<()> {
    // Max reasonable amount: ~18 ETH equivalent (max u64)
    const MAX_REASONABLE_WEI: u64 = u64::MAX;
    validate_numeric_range("ntc_amount", amount_wei, 0, MAX_REASONABLE_WEI)?;
    Ok(())
}

/// Validate credit amount in cents
pub fn validate_credit_amount_cents(amount_cents: u64) -> Result<()> {
    const MAX_CREDITS_CENTS: u64 = 1_000_000_00; // 1 million credits in cents
    validate_numeric_range("credit_amount", amount_cents, 0, MAX_CREDITS_CENTS)?;
    Ok(())
}

/// Validate gas price (reasonable bounds)
pub fn validate_gas_price(gas_price: u64) -> Result<()> {
    const MIN_GAS_PRICE: u64 = 100_000_000; // 0.1 gwei
    const MAX_GAS_PRICE: u64 = 1_000_000_000_000; // 1000 gwei
    validate_numeric_range("gas_price", gas_price, MIN_GAS_PRICE, MAX_GAS_PRICE)?;
    Ok(())
}

/// Validate gas limit (reasonable bounds)
pub fn validate_gas_limit(gas_limit: u64) -> Result<()> {
    const MIN_GAS_LIMIT: u64 = 21_000; // Basic transfer
    const MAX_GAS_LIMIT: u64 = 30_000_000; // Block gas limit
    validate_numeric_range("gas_limit", gas_limit, MIN_GAS_LIMIT, MAX_GAS_LIMIT)?;
    Ok(())
}

/// Validate signature format (65 bytes as hex string)
pub fn validate_signature(signature: &str) -> Result<()> {
    if signature.len() != 130 {
        // 65 bytes * 2 (hex) = 130 chars
        return Err(ValidationError::InvalidFormat(format!(
            "Invalid signature length: expected 130 chars, got {}",
            signature.len()
        )));
    }

    // Check if it's valid hex
    if !signature.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::InvalidFormat(
            "Signature must be valid hexadecimal".to_string(),
        ));
    }

    Ok(())
}

/// Validate transaction hash format
pub fn validate_transaction_hash(tx_hash: &str) -> Result<()> {
    if !tx_hash.starts_with("0x") || tx_hash.len() != 66 {
        return Err(ValidationError::InvalidFormat(format!(
            "Invalid transaction hash format: {}",
            tx_hash
        )));
    }

    let hex_part = &tx_hash[2..];
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::InvalidFormat(
            "Transaction hash must be valid hexadecimal".to_string(),
        ));
    }

    Ok(())
}

/// Validate URL format (basic check)
pub fn validate_url(url: &str) -> Result<()> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ValidationError::InvalidFormat(format!(
            "Invalid URL format: must start with http:// or https://"
        )));
    }

    if url.len() > 2048 {
        return Err(ValidationError::InvalidLength {
            field: "url".to_string(),
            actual: url.len(),
            min: 1,
            max: 2048,
        });
    }

    Ok(())
}

// ============================================================================
// Enum Validators
// ============================================================================

/// Validate enum values by name
pub fn validate_enum_value(field: &str, value: &str, valid_values: &[&str]) -> Result<()> {
    if !valid_values.contains(&value) {
        return Err(ValidationError::InvalidEnumValue {
            field: field.to_string(),
            value: value.to_string(),
        });
    }
    Ok(())
}

/// Valid BunkerClass enum values
pub const VALID_BUNKER_CLASSES: &[&str] = &[
    "EXPLORER",
    "PATHFINDER",
    "CYBERMANCER",
    "VANGUARD",
    "ENFORCER",
    "SCAVENGER",
    "STALKER",
    "DISRUPTOR",
    "CODEBREAKER",
    "OVERLORD",
    "BREACHER",
    "RECLAIMER",
];

/// Valid ClassAffiliation enum values
pub const VALID_CLASS_AFFILIATIONS: &[&str] = &["LOYAL", "CORRUPT", "NEUTRAL"];

/// Valid ItemRarity enum values
pub const VALID_ITEM_RARITIES: &[&str] = &[
    "STANDARD",
    "OPTIMIZED",
    "ADVANCED",
    "SUPREME",
    "ECHELON",
    "ETERNAL",
];

/// Valid ItemType enum values
pub const VALID_ITEM_TYPES: &[&str] = &[
    "HEAD",
    "TORSO",
    "GEAR",
    "ACCESSORY",
    "PERK",
    "BADGE",
    "BUNKERGUARD_ROBOT",
    "ROBOT_ITEM",
    "COSMETIC_SKIN",
];

/// Validate BunkerClass enum value
pub fn validate_bunker_class(class: &str) -> Result<()> {
    validate_enum_value("bunker_class", class, VALID_BUNKER_CLASSES)
}

/// Validate ClassAffiliation enum value
pub fn validate_class_affiliation(affiliation: &str) -> Result<()> {
    validate_enum_value("class_affiliation", affiliation, VALID_CLASS_AFFILIATIONS)
}

/// Validate ItemRarity enum value
pub fn validate_item_rarity(rarity: &str) -> Result<()> {
    validate_enum_value("item_rarity", rarity, VALID_ITEM_RARITIES)
}

/// Validate ItemType enum value
pub fn validate_item_type(item_type: &str) -> Result<()> {
    validate_enum_value("item_type", item_type, VALID_ITEM_TYPES)
}

// ============================================================================
// Batch Validators
// ============================================================================

/// Validate list of player IDs
pub fn validate_player_id_list(player_ids: &[String]) -> Result<()> {
    for player_id in player_ids {
        validate_player_id(player_id)?;
    }
    Ok(())
}

/// Validate list of NFT IDs
pub fn validate_nft_id_list(nft_ids: &[String]) -> Result<()> {
    for nft_id in nft_ids {
        validate_nft_id(nft_id)?;
    }
    Ok(())
}

/// Validate pagination parameters
pub fn validate_pagination(page: u32, page_size: u32) -> Result<()> {
    if page == 0 {
        return Err(crate::errors::BunkerVerseError::Validation(
            ValidationError::InvalidFormat(
                "Page number must be >= 1".to_string()
            )
        ));
    }

    if page_size == 0 || page_size > 100 {
        return Err(crate::errors::BunkerVerseError::Validation(
            ValidationError::OutOfRange {
                field: "page_size".to_string(),
                value: page_size.to_string(),
                min: Some("1".to_string()),
                max: Some("100".to_string()),
            }
        ));
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_v4_validation() {
        // Valid UUID v4
        assert!(validate_uuid_v4("550e8400-e29b-41d4-a716-446655440000").is_ok());

        // Invalid formats
        assert!(validate_uuid_v4("invalid-uuid").is_err());
        assert!(validate_uuid_v4("550e8400-e29b-41d4-a716").is_err());
        assert!(validate_uuid_v4("").is_err());
    }

    #[test]
    fn test_ethereum_address_validation() {
        // Valid addresses
        assert!(validate_ethereum_address("0x742d35cc6678cf92dbf4d9f2c9c6f5e9e5c6f7db").is_ok());
        assert!(validate_ethereum_address("0x742D35CC6678CF92DBF4D9F2C9C6F5E9E5C6F7DB").is_ok());

        // Invalid addresses
        assert!(validate_ethereum_address("742d35cc6678cf92dbf4d9f2c9c6f5e9e5c6f7db").is_err()); // Missing 0x
        assert!(validate_ethereum_address("0x742d35cc6678cf92dbf4d9f2c9c6f5e9e5c6f7d").is_err()); // Too short
        assert!(validate_ethereum_address("0x742d35cc6678cf92dbf4d9f2c9c6f5e9e5c6f7dbg").is_err());
        // Invalid hex
    }

    #[test]
    fn test_string_length_validation() {
        assert!(validate_string_length("test_field", "hello", 1, 10).is_ok());
        assert!(validate_string_length("test_field", "", 1, 10).is_err()); // Too short
        assert!(validate_string_length("test_field", "hello world long", 1, 10).is_err());
        // Too long
    }

    #[test]
    fn test_numeric_range_validation() {
        assert!(validate_numeric_range("level", 50, 1, 100).is_ok());
        assert!(validate_numeric_range("level", 0, 1, 100).is_err()); // Too low
        assert!(validate_numeric_range("level", 101, 1, 100).is_err()); // Too high
    }

    #[test]
    fn test_timestamp_validation() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        assert!(validate_timestamp(current_time).is_ok()); // Current time
        assert!(validate_timestamp(1609459200).is_ok()); // 2021-01-01
        assert!(validate_timestamp(1000000000).is_err()); // Too old (2001)
        assert!(validate_timestamp(current_time + 100000).is_err()); // Too far in future
    }

    #[test]
    fn test_bunker_tag_validation() {
        assert!(validate_bunker_tag("player123").is_ok());
        assert!(validate_bunker_tag("player_test").is_ok());
        assert!(validate_bunker_tag("player-test").is_ok());

        assert!(validate_bunker_tag("ab").is_err()); // Too short
        assert!(validate_bunker_tag("player@test").is_err()); // Invalid characters
        assert!(validate_bunker_tag("").is_err()); // Empty
    }

    #[test]
    fn test_signature_validation() {
        let valid_sig = "1234567890abcdef".repeat(8) + "12"; // 130 hex chars
        assert!(validate_signature(&valid_sig).is_ok());

        assert!(validate_signature("1234567890abcdef").is_err()); // Too short
        assert!(validate_signature(&("invalid_hex".repeat(20))).is_err()); // Invalid hex
    }

    #[test]
    fn test_enum_validation() {
        assert!(validate_bunker_class("EXPLORER").is_ok());
        assert!(validate_bunker_class("INVALID_CLASS").is_err());

        assert!(validate_item_rarity("SUPREME").is_ok());
        assert!(validate_item_rarity("INVALID_RARITY").is_err());
    }

    #[test]
    fn test_pagination_validation() {
        assert!(validate_pagination(1, 50).is_ok());
        assert!(validate_pagination(0, 50).is_err()); // Page 0 invalid
        assert!(validate_pagination(1, 0).is_err()); // Page size 0 invalid
        assert!(validate_pagination(1, 101).is_err()); // Page size too large
    }

    #[test]
    fn test_url_validation() {
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("http://example.com/path").is_ok());

        assert!(validate_url("ftp://example.com").is_err()); // Wrong protocol
        assert!(validate_url("not_a_url").is_err()); // Not a URL
    }

    #[test]
    fn test_composite_validators() {
        assert!(validate_player_id("550e8400-e29b-41d4-a716-446655440000").is_ok());
        assert!(validate_player_id("").is_err());
        assert!(validate_player_id("invalid").is_err());

        assert!(validate_display_name("John Doe").is_ok());
        assert!(validate_display_name("").is_err());
        assert!(validate_display_name(&"x".repeat(100)).is_err()); // Too long
    }
}
