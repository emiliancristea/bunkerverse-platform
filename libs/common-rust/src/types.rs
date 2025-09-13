//! Core domain types for BUNKERVERSE Platform
//! UUID-based identifiers, validated types, and shared data structures

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

use crate::errors::{BunkerVerseError, Result, ValidationError};
use crate::validation::{validate_ethereum_address, validate_uuid_v4};

// ============================================================================
// Core ID Types
// ============================================================================

/// Player identifier - Validated UUID v4 format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(Uuid);

impl PlayerId {
    /// Create a new random PlayerId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create PlayerId from validated UUID string
    pub fn from_string(id: &str) -> Result<Self> {
        validate_uuid_v4(id)?;
        let uuid = Uuid::from_str(id).map_err(|_| {
            BunkerVerseError::Validation(ValidationError::InvalidFormat(
                "Invalid UUID format".to_string(),
            ))
        })?;
        Ok(Self(uuid))
    }

    /// Get the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Convert to string representation
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }

    /// Validate UUID v4 format specifically
    pub fn is_valid(id: &str) -> bool {
        validate_uuid_v4(id).is_ok()
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PlayerId {
    type Err = BunkerVerseError;

    fn from_str(s: &str) -> std::result::Result<Self, BunkerVerseError> {
        let uuid = Uuid::from_str(s).map_err(|_| {
            BunkerVerseError::Validation(ValidationError::InvalidFormat(
                "Invalid UUID format".to_string(),
            ))
        })?;
        if uuid.get_version() != Some(uuid::Version::Random) {
            return Err(BunkerVerseError::Validation(
                ValidationError::InvalidFormat("Must be UUID v4".to_string()),
            ));
        }
        Ok(Self(uuid))
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}

/// NFT identifier - Validated UUID v4 format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NftId(Uuid);

impl NftId {
    /// Create a new random NftId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create NftId from validated UUID string
    pub fn from_string(id: &str) -> Result<Self> {
        validate_uuid_v4(id)?;
        let uuid = Uuid::from_str(id).map_err(|_| {
            BunkerVerseError::Validation(ValidationError::InvalidFormat(
                "Invalid UUID format".to_string(),
            ))
        })?;
        Ok(Self(uuid))
    }

    /// Get the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Convert to string representation
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }

    /// Validate UUID v4 format specifically
    pub fn is_valid(id: &str) -> bool {
        validate_uuid_v4(id).is_ok()
    }
}

impl fmt::Display for NftId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for NftId {
    type Err = BunkerVerseError;

    fn from_str(s: &str) -> std::result::Result<Self, BunkerVerseError> {
        let uuid = Uuid::from_str(s).map_err(|_| {
            BunkerVerseError::Validation(ValidationError::InvalidFormat(
                "Invalid UUID format".to_string(),
            ))
        })?;
        if uuid.get_version() != Some(uuid::Version::Random) {
            return Err(BunkerVerseError::Validation(
                ValidationError::InvalidFormat("Must be UUID v4".to_string()),
            ));
        }
        Ok(Self(uuid))
    }
}

impl Default for NftId {
    fn default() -> Self {
        Self::new()
    }
}

/// Mission identifier - Validated UUID v4 format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MissionId(Uuid);

impl MissionId {
    /// Create a new random MissionId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create MissionId from validated UUID string
    pub fn from_string(id: &str) -> Result<Self> {
        validate_uuid_v4(id)?;
        let uuid = Uuid::from_str(id).map_err(|_| {
            BunkerVerseError::Validation(ValidationError::InvalidFormat(
                "Invalid UUID format".to_string(),
            ))
        })?;
        Ok(Self(uuid))
    }

    /// Get the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Convert to string representation
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for MissionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MissionId {
    type Err = BunkerVerseError;

    fn from_str(s: &str) -> std::result::Result<Self, BunkerVerseError> {
        let uuid = Uuid::from_str(s).map_err(|_| {
            BunkerVerseError::Validation(ValidationError::InvalidFormat(
                "Invalid UUID format".to_string(),
            ))
        })?;
        if uuid.get_version() != Some(uuid::Version::Random) {
            return Err(BunkerVerseError::Validation(
                ValidationError::InvalidFormat("Must be UUID v4".to_string()),
            ));
        }
        Ok(Self(uuid))
    }
}

impl Default for MissionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Robot identifier - Validated UUID v4 format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RobotId(Uuid);

impl RobotId {
    /// Create a new random RobotId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create RobotId from validated UUID string
    pub fn from_string(id: &str) -> Result<Self> {
        validate_uuid_v4(id)?;
        let uuid = Uuid::from_str(id).map_err(|_| {
            BunkerVerseError::Validation(ValidationError::InvalidFormat(
                "Invalid UUID format".to_string(),
            ))
        })?;
        Ok(Self(uuid))
    }

    /// Get the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Convert to string representation
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for RobotId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RobotId {
    type Err = BunkerVerseError;

    fn from_str(s: &str) -> std::result::Result<Self, BunkerVerseError> {
        let uuid = Uuid::from_str(s).map_err(|_| {
            BunkerVerseError::Validation(ValidationError::InvalidFormat(
                "Invalid UUID format".to_string(),
            ))
        })?;
        if uuid.get_version() != Some(uuid::Version::Random) {
            return Err(BunkerVerseError::Validation(
                ValidationError::InvalidFormat("Must be UUID v4".to_string()),
            ));
        }
        Ok(Self(uuid))
    }
}

impl Default for RobotId {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Ethereum Address Type
// ============================================================================

/// Ethereum address - Validated hex format
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthereumAddress(String);

impl EthereumAddress {
    /// Create EthereumAddress from validated hex string
    pub fn from_string(address: &str) -> Result<Self> {
        validate_ethereum_address(address)?;
        Ok(Self(address.to_lowercase()))
    }

    /// Get the address as string
    pub fn as_string(&self) -> &str {
        &self.0
    }

    /// Get checksum address format
    pub fn as_checksum(&self) -> String {
        // Basic checksum implementation - in production use proper EIP-55
        let mut result = String::with_capacity(42);
        result.push_str("0x");

        for (i, c) in self.0[2..].chars().enumerate() {
            if c.is_ascii_digit() {
                result.push(c);
            } else {
                // Simplified checksum - use proper keccak256 in production
                if i % 2 == 0 {
                    result.push(c.to_ascii_uppercase());
                } else {
                    result.push(c);
                }
            }
        }
        result
    }

    /// Check if address is valid format
    pub fn is_valid(address: &str) -> bool {
        validate_ethereum_address(address).is_ok()
    }
}

impl fmt::Display for EthereumAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EthereumAddress {
    type Err = BunkerVerseError;

    fn from_str(s: &str) -> std::result::Result<Self, BunkerVerseError> {
        if !s.starts_with("0x") || s.len() != 42 {
            return Err(BunkerVerseError::Validation(
                ValidationError::InvalidFormat(
                    "Ethereum address must be 42 characters starting with 0x".to_string(),
                ),
            ));
        }
        let hex_part = &s[2..];
        if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(BunkerVerseError::Validation(
                ValidationError::InvalidFormat(
                    "Ethereum address must contain only hex characters".to_string(),
                ),
            ));
        }
        Ok(Self(s.to_string()))
    }
}

impl Serialize for EthereumAddress {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for EthereumAddress {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_string(&s).map_err(serde::de::Error::custom)
    }
}

// ============================================================================
// Bounded Value Types
// ============================================================================

/// Player level - Bounded between 1 and 100
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayerLevel(u32);

impl PlayerLevel {
    pub const MIN: u32 = 1;
    pub const MAX: u32 = 100;

    /// Create PlayerLevel with validation
    pub fn new(level: u32) -> Result<Self> {
        if !(Self::MIN..=Self::MAX).contains(&level) {
            return Err(BunkerVerseError::Validation(ValidationError::OutOfRange {
                field: "player_level".to_string(),
                value: level.to_string(),
                min: Some(Self::MIN.to_string()),
                max: Some(Self::MAX.to_string()),
            }));
        }
        Ok(Self(level))
    }

    /// Get the inner value
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Check if level is valid
    pub fn is_valid(level: u32) -> bool {
        (Self::MIN..=Self::MAX).contains(&level)
    }
}

impl Default for PlayerLevel {
    fn default() -> Self {
        Self(1)
    }
}

impl fmt::Display for PlayerLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Stat value - Bounded between 0 and 1000
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct StatValue(u32);

impl StatValue {
    pub const MIN: u32 = 0;
    pub const MAX: u32 = 1000;

    /// Create StatValue with validation
    pub fn new(value: u32) -> Result<Self> {
        if value > Self::MAX {
            return Err(BunkerVerseError::Validation(ValidationError::OutOfRange {
                field: "stat_value".to_string(),
                value: value.to_string(),
                min: Some(Self::MIN.to_string()),
                max: Some(Self::MAX.to_string()),
            }));
        }
        Ok(Self(value))
    }

    /// Get the inner value
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Check if value is valid
    pub fn is_valid(value: u32) -> bool {
        value <= Self::MAX
    }
}

impl fmt::Display for StatValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Currency Types
// ============================================================================

/// NTC amount in wei (smallest unit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct NtcAmount(u64);

impl NtcAmount {
    /// Wei per NTC (18 decimals)
    pub const WEI_PER_NTC: u64 = 1_000_000_000_000_000_000;

    /// Maximum supply (unlimited in practice, but set reasonable bound for validation)
    pub const MAX_SUPPLY_WEI: u64 = u64::MAX;

    /// Create NtcAmount from wei
    pub fn from_wei(wei: u64) -> Self {
        Self(wei)
    }

    /// Create NtcAmount from NTC (with 18 decimal places)
    pub fn from_ntc(ntc: f64) -> Self {
        Self((ntc * Self::WEI_PER_NTC as f64) as u64)
    }

    /// Get amount in wei
    pub fn wei(&self) -> u64 {
        self.0
    }

    /// Get amount in NTC (as float)
    pub fn ntc(&self) -> f64 {
        self.0 as f64 / Self::WEI_PER_NTC as f64
    }

    /// Format as human-readable NTC amount
    pub fn format_ntc(&self) -> String {
        format!("{:.6} NTC", self.ntc())
    }
}

impl fmt::Display for NtcAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_ntc())
    }
}

/// Credits amount (fiat-backed currency for MVE mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct CreditAmount(u64);

impl CreditAmount {
    /// Credits are stored as integer cents to avoid floating point precision issues
    pub const CENTS_PER_CREDIT: u64 = 100;

    /// Maximum credits a player can hold
    pub const MAX_CREDITS: u64 = 100_000_000; // 1 million credits

    /// Create CreditAmount from cents
    pub fn from_cents(cents: u64) -> Result<Self> {
        if cents > Self::MAX_CREDITS {
            return Err(BunkerVerseError::Validation(ValidationError::OutOfRange {
                field: "credit_amount".to_string(),
                value: cents.to_string(),
                min: Some("0".to_string()),
                max: Some(Self::MAX_CREDITS.to_string()),
            }));
        }
        Ok(Self(cents))
    }

    /// Create CreditAmount from credits (with 2 decimal places)
    pub fn from_credits(credits: f64) -> Result<Self> {
        let cents = (credits * Self::CENTS_PER_CREDIT as f64) as u64;
        Self::from_cents(cents)
    }

    /// Get amount in cents
    pub fn cents(&self) -> u64 {
        self.0
    }

    /// Get amount in credits (as float)
    pub fn credits(&self) -> f64 {
        self.0 as f64 / Self::CENTS_PER_CREDIT as f64
    }

    /// Format as human-readable credits amount
    pub fn format_credits(&self) -> String {
        format!("{:.2} Credits", self.credits())
    }
}

impl fmt::Display for CreditAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_credits())
    }
}

/// Experience points
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct ExperiencePoints(u64);

impl ExperiencePoints {
    /// Maximum XP a player can have
    pub const MAX_XP: u64 = 1_000_000_000; // 1 billion XP

    /// Create ExperiencePoints with validation
    pub fn new(xp: u64) -> Result<Self> {
        if xp > Self::MAX_XP {
            return Err(BunkerVerseError::Validation(ValidationError::OutOfRange {
                field: "experience_points".to_string(),
                value: xp.to_string(),
                min: Some("0".to_string()),
                max: Some(Self::MAX_XP.to_string()),
            }));
        }
        Ok(Self(xp))
    }

    /// Get XP value
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Calculate player level from XP (basic formula)
    pub fn to_player_level(&self) -> PlayerLevel {
        // Simple level calculation: level = floor(sqrt(xp / 1000)) + 1
        let level = ((self.0 as f64 / 1000.0).sqrt() as u32 + 1).min(PlayerLevel::MAX);
        PlayerLevel::new(level).unwrap_or_default()
    }
}

impl fmt::Display for ExperiencePoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} XP", self.0)
    }
}

// ============================================================================
// Core Stats Structure
// ============================================================================

/// Core stats for robots and items with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreStats {
    // Combat stats
    pub damage: StatValue,
    pub accuracy: StatValue,
    pub critical_chance: StatValue,
    pub armor_piercing: StatValue,

    // Mobility stats
    pub speed: StatValue,
    pub agility: StatValue,
    pub stealth: StatValue,
    pub evasion: StatValue,

    // Survivability stats
    pub health: StatValue,
    pub shield: StatValue,

    // Sensors stats
    pub detection: StatValue,
    pub range: StatValue,
}

impl CoreStats {
    /// Create new CoreStats with all zeros
    pub fn new() -> Self {
        Self {
            damage: StatValue::default(),
            accuracy: StatValue::default(),
            critical_chance: StatValue::default(),
            armor_piercing: StatValue::default(),
            speed: StatValue::default(),
            agility: StatValue::default(),
            stealth: StatValue::default(),
            evasion: StatValue::default(),
            health: StatValue::default(),
            shield: StatValue::default(),
            detection: StatValue::default(),
            range: StatValue::default(),
        }
    }

    /// Calculate combat average
    pub fn combat_average(&self) -> StatValue {
        let sum = self.damage.value()
            + self.accuracy.value()
            + self.critical_chance.value()
            + self.armor_piercing.value();
        StatValue::new(sum / 4).unwrap_or_default()
    }

    /// Calculate mobility average
    pub fn mobility_average(&self) -> StatValue {
        let sum =
            self.speed.value() + self.agility.value() + self.stealth.value() + self.evasion.value();
        StatValue::new(sum / 4).unwrap_or_default()
    }

    /// Calculate survivability average
    pub fn survivability_average(&self) -> StatValue {
        let sum = self.health.value() + self.shield.value();
        StatValue::new(sum / 2).unwrap_or_default()
    }

    /// Calculate sensors average
    pub fn sensors_average(&self) -> StatValue {
        let sum = self.detection.value() + self.range.value();
        StatValue::new(sum / 2).unwrap_or_default()
    }

    /// Add stats from another CoreStats (for equipment bonuses)
    pub fn add(&self, other: &CoreStats) -> Result<CoreStats> {
        Ok(CoreStats {
            damage: StatValue::new(self.damage.value() + other.damage.value())?,
            accuracy: StatValue::new(self.accuracy.value() + other.accuracy.value())?,
            critical_chance: StatValue::new(
                self.critical_chance.value() + other.critical_chance.value(),
            )?,
            armor_piercing: StatValue::new(
                self.armor_piercing.value() + other.armor_piercing.value(),
            )?,
            speed: StatValue::new(self.speed.value() + other.speed.value())?,
            agility: StatValue::new(self.agility.value() + other.agility.value())?,
            stealth: StatValue::new(self.stealth.value() + other.stealth.value())?,
            evasion: StatValue::new(self.evasion.value() + other.evasion.value())?,
            health: StatValue::new(self.health.value() + other.health.value())?,
            shield: StatValue::new(self.shield.value() + other.shield.value())?,
            detection: StatValue::new(self.detection.value() + other.detection.value())?,
            range: StatValue::new(self.range.value() + other.range.value())?,
        })
    }
}

impl Default for CoreStats {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_id_creation() {
        let id = PlayerId::new();
        assert!(!id.as_string().is_empty());

        let id_str = "550e8400-e29b-41d4-a716-446655440000";
        let id2 = PlayerId::from_string(id_str).unwrap();
        assert_eq!(id2.as_string(), id_str);
    }

    #[test]
    fn test_player_id_validation() {
        // Valid UUID v4
        assert!(PlayerId::is_valid("550e8400-e29b-41d4-a716-446655440000"));

        // Invalid format
        assert!(!PlayerId::is_valid("invalid-uuid"));
        assert!(!PlayerId::is_valid("550e8400-e29b-41d4-a716"));
    }

    #[test]
    fn test_ethereum_address() {
        let addr =
            EthereumAddress::from_string("0x742d35cc6678cf92dbf4d9f2c9c6f5e9e5c6f7db").unwrap();
        assert_eq!(
            addr.as_string(),
            "0x742d35cc6678cf92dbf4d9f2c9c6f5e9e5c6f7db"
        );

        // Invalid format
        assert!(EthereumAddress::from_string("invalid").is_err());
        assert!(EthereumAddress::from_string("0x123").is_err());
    }

    #[test]
    fn test_player_level_bounds() {
        assert!(PlayerLevel::new(0).is_err());
        assert!(PlayerLevel::new(1).is_ok());
        assert!(PlayerLevel::new(100).is_ok());
        assert!(PlayerLevel::new(101).is_err());
    }

    #[test]
    fn test_stat_value_bounds() {
        assert!(StatValue::new(0).is_ok());
        assert!(StatValue::new(1000).is_ok());
        assert!(StatValue::new(1001).is_err());
    }

    #[test]
    fn test_ntc_amount() {
        let amount = NtcAmount::from_ntc(1.5);
        assert_eq!(amount.wei(), 1_500_000_000_000_000_000);
        assert!((amount.ntc() - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_credit_amount() {
        let amount = CreditAmount::from_credits(10.50).unwrap();
        assert_eq!(amount.cents(), 1050);
        assert!((amount.credits() - 10.50).abs() < f64::EPSILON);
    }

    #[test]
    fn test_core_stats_averages() {
        let mut stats = CoreStats::new();
        stats.damage = StatValue::new(100).unwrap();
        stats.accuracy = StatValue::new(200).unwrap();
        stats.critical_chance = StatValue::new(300).unwrap();
        stats.armor_piercing = StatValue::new(400).unwrap();

        assert_eq!(stats.combat_average().value(), 250);
    }

    #[test]
    fn test_experience_to_level() {
        let xp = ExperiencePoints::new(0).unwrap();
        assert_eq!(xp.to_player_level().value(), 1);

        let xp = ExperiencePoints::new(1000).unwrap();
        assert_eq!(xp.to_player_level().value(), 2);

        let xp = ExperiencePoints::new(4000).unwrap();
        assert_eq!(xp.to_player_level().value(), 3);
    }
}
