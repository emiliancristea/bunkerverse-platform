//! Time handling utilities for BUNKERVERSE Platform
//! Standardized timestamp handling, formatting, and validation

use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::errors::{Result, ValidationError};
use crate::validation::validate_timestamp;

// ============================================================================
// Core Timestamp Type
// ============================================================================

/// Validated timestamp in Unix seconds
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(i64);

impl Timestamp {
    /// Minimum valid timestamp (2021-01-01 00:00:00 UTC)
    pub const MIN: i64 = 1609459200;
    
    /// Maximum future offset (1 day in seconds)
    pub const MAX_FUTURE_OFFSET_SECONDS: i64 = 86400;

    /// Create Timestamp from Unix seconds with validation
    pub fn from_unix_seconds(seconds: i64) -> Result<Self> {
        validate_timestamp(seconds)?;
        Ok(Self(seconds))
    }

    /// Create Timestamp from current system time
    pub fn now() -> Self {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        Self(seconds)
    }

    /// Create Timestamp from SystemTime
    pub fn from_system_time(time: SystemTime) -> Result<Self> {
        let seconds = time
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ValidationError::InvalidTimestamp {
                timestamp: 0,
                reason: "SystemTime before Unix epoch".to_string(),
            })?
            .as_secs() as i64;
        Self::from_unix_seconds(seconds)
    }

    /// Get Unix seconds
    pub fn unix_seconds(&self) -> i64 {
        self.0
    }

    /// Get Unix milliseconds
    pub fn unix_millis(&self) -> i64 {
        self.0 * 1000
    }

    /// Convert to SystemTime
    pub fn to_system_time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(self.0 as u64)
    }

    /// Add duration to timestamp
    pub fn add_duration(&self, duration: Duration) -> Result<Self> {
        let new_seconds = self.0 + duration.as_secs() as i64;
        Self::from_unix_seconds(new_seconds)
    }

    /// Subtract duration from timestamp
    pub fn sub_duration(&self, duration: Duration) -> Result<Self> {
        let new_seconds = self.0 - duration.as_secs() as i64;
        Self::from_unix_seconds(new_seconds)
    }

    /// Get duration since another timestamp
    pub fn duration_since(&self, other: &Timestamp) -> Duration {
        if self.0 >= other.0 {
            Duration::from_secs((self.0 - other.0) as u64)
        } else {
            Duration::from_secs(0)
        }
    }

    /// Check if timestamp is in the future
    pub fn is_future(&self) -> bool {
        self.0 > Timestamp::now().0
    }

    /// Check if timestamp is in the past
    pub fn is_past(&self) -> bool {
        self.0 < Timestamp::now().0
    }

    /// Format as ISO 8601 string (UTC)
    pub fn to_iso8601(&self) -> String {
        format_unix_timestamp(self.0)
    }

    /// Format as human-readable string
    pub fn to_human_readable(&self) -> String {
        format_human_readable(self.0)
    }

    /// Format as relative time (e.g., "2 hours ago", "in 5 minutes")
    pub fn to_relative_time(&self) -> String {
        format_relative_time(self.0)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_iso8601())
    }
}

impl From<Timestamp> for i64 {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

impl TryFrom<i64> for Timestamp {
    type Error = ValidationError;

    fn try_from(seconds: i64) -> std::result::Result<Self, ValidationError> {
        if seconds < Self::MIN {
            return Err(ValidationError::OutOfRange {
                field: "timestamp".to_string(),
                value: seconds.to_string(),
                min: Some(Self::MIN.to_string()),
                max: Some("current time + 1 day".to_string()),
            });
        }
        
        let current_time = current_unix_timestamp();
        if seconds > current_time + Self::MAX_FUTURE_OFFSET_SECONDS {
            return Err(ValidationError::OutOfRange {
                field: "timestamp".to_string(),
                value: seconds.to_string(),
                min: Some(Self::MIN.to_string()),
                max: Some((current_time + Self::MAX_FUTURE_OFFSET_SECONDS).to_string()),
            });
        }

        Ok(Self(seconds))
    }
}

// ============================================================================
// Time Utilities
// ============================================================================

/// Get current Unix timestamp in seconds
pub fn current_unix_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Get current Unix timestamp in milliseconds
pub fn current_unix_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

/// Convert Unix timestamp to ISO 8601 string (UTC)
pub fn format_unix_timestamp(timestamp: i64) -> String {
    // Basic ISO 8601 formatting - in production use chrono for proper formatting
    let datetime = UNIX_EPOCH + Duration::from_secs(timestamp as u64);

    // Simple formatting - this is a placeholder for proper datetime formatting
    format!("{}T00:00:00Z", timestamp / 86400 + 719163) // Rough approximation
}

/// Format timestamp as human-readable string
pub fn format_human_readable(timestamp: i64) -> String {
    let datetime = UNIX_EPOCH + Duration::from_secs(timestamp as u64);

    // Simple formatting - in production use chrono
    let days_since_epoch = timestamp / 86400;
    let year = 1970 + (days_since_epoch / 365);
    let day_of_year = days_since_epoch % 365;
    let month = (day_of_year / 30) + 1;
    let day = (day_of_year % 30) + 1;

    format!("{}-{:02}-{:02}", year, month.min(12), day.min(31))
}

/// Format timestamp as relative time
pub fn format_relative_time(timestamp: i64) -> String {
    let current = current_unix_timestamp();
    let diff = current - timestamp;

    if diff < 0 {
        // Future time
        let future_diff = -diff;
        if future_diff < 60 {
            return "in a few seconds".to_string();
        } else if future_diff < 3600 {
            return format!("in {} minutes", future_diff / 60);
        } else if future_diff < 86400 {
            return format!("in {} hours", future_diff / 3600);
        } else {
            return format!("in {} days", future_diff / 86400);
        }
    }

    // Past time
    if diff < 60 {
        "a few seconds ago".to_string()
    } else if diff < 3600 {
        format!("{} minutes ago", diff / 60)
    } else if diff < 86400 {
        format!("{} hours ago", diff / 3600)
    } else if diff < 604800 {
        format!("{} days ago", diff / 86400)
    } else if diff < 2629746 {
        format!("{} weeks ago", diff / 604800)
    } else if diff < 31556952 {
        format!("{} months ago", diff / 2629746)
    } else {
        format!("{} years ago", diff / 31556952)
    }
}

/// Check if timestamp is today (UTC)
pub fn is_today(timestamp: i64) -> bool {
    let current = current_unix_timestamp();
    let current_day = current / 86400;
    let timestamp_day = timestamp / 86400;
    current_day == timestamp_day
}

/// Check if timestamp is this week (UTC, week starts Monday)
pub fn is_this_week(timestamp: i64) -> bool {
    let current = current_unix_timestamp();
    let current_week = (current / 86400 + 4) / 7; // +4 to make Monday week start
    let timestamp_week = (timestamp / 86400 + 4) / 7;
    current_week == timestamp_week
}

/// Get start of day timestamp (00:00:00 UTC)
pub fn start_of_day(timestamp: i64) -> i64 {
    (timestamp / 86400) * 86400
}

/// Get end of day timestamp (23:59:59 UTC)
pub fn end_of_day(timestamp: i64) -> i64 {
    start_of_day(timestamp) + 86399
}

/// Get start of week timestamp (Monday 00:00:00 UTC)
pub fn start_of_week(timestamp: i64) -> i64 {
    let days_since_epoch = timestamp / 86400;
    let days_since_monday = (days_since_epoch + 4) % 7; // +4 to make Monday = 0
    let monday_days = days_since_epoch - days_since_monday;
    monday_days * 86400
}

/// Get end of week timestamp (Sunday 23:59:59 UTC)
pub fn end_of_week(timestamp: i64) -> i64 {
    start_of_week(timestamp) + (7 * 86400) - 1
}

// ============================================================================
// Duration Utilities
// ============================================================================

/// Convert duration to human-readable string
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    if total_seconds < 60 {
        format!("{}s", total_seconds)
    } else if total_seconds < 3600 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        if seconds == 0 {
            format!("{}m", minutes)
        } else {
            format!("{}m {}s", minutes, seconds)
        }
    } else if total_seconds < 86400 {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        if minutes == 0 {
            format!("{}h", hours)
        } else {
            format!("{}h {}m", hours, minutes)
        }
    } else {
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        if hours == 0 {
            format!("{}d", days)
        } else {
            format!("{}d {}h", days, hours)
        }
    }
}

/// Parse duration from human-readable string (basic implementation)
pub fn parse_duration(duration_str: &str) -> Result<Duration> {
    let duration_str = duration_str.trim().to_lowercase();

    if duration_str.ends_with("s") {
        let num_str = &duration_str[..duration_str.len() - 1];
        let seconds: u64 = num_str.parse().map_err(|_| {
            ValidationError::InvalidFormat(format!("Invalid duration format: {}", duration_str))
        })?;
        return Ok(Duration::from_secs(seconds));
    }

    if duration_str.ends_with("m") {
        let num_str = &duration_str[..duration_str.len() - 1];
        let minutes: u64 = num_str.parse().map_err(|_| {
            ValidationError::InvalidFormat(format!("Invalid duration format: {}", duration_str))
        })?;
        return Ok(Duration::from_secs(minutes * 60));
    }

    if duration_str.ends_with("h") {
        let num_str = &duration_str[..duration_str.len() - 1];
        let hours: u64 = num_str.parse().map_err(|_| {
            ValidationError::InvalidFormat(format!("Invalid duration format: {}", duration_str))
        })?;
        return Ok(Duration::from_secs(hours * 3600));
    }

    if duration_str.ends_with("d") {
        let num_str = &duration_str[..duration_str.len() - 1];
        let days: u64 = num_str.parse().map_err(|_| {
            ValidationError::InvalidFormat(format!("Invalid duration format: {}", duration_str))
        })?;
        return Ok(Duration::from_secs(days * 86400));
    }

    // Try parsing as raw seconds
    let seconds: u64 = duration_str.parse().map_err(|_| {
        ValidationError::InvalidFormat(format!("Invalid duration format: {}", duration_str))
    })?;
    Ok(Duration::from_secs(seconds))
}

// ============================================================================
// Time Constants
// ============================================================================

pub const SECOND: Duration = Duration::from_secs(1);
pub const MINUTE: Duration = Duration::from_secs(60);
pub const HOUR: Duration = Duration::from_secs(3600);
pub const DAY: Duration = Duration::from_secs(86400);
pub const WEEK: Duration = Duration::from_secs(604800);
pub const MONTH: Duration = Duration::from_secs(2629746); // Average month (30.44 days)
pub const YEAR: Duration = Duration::from_secs(31556952); // Average year (365.24 days)

// ============================================================================
// Game-Specific Time Functions
// ============================================================================

/// Check if it's time for daily reset (assumes UTC 00:00 reset)
pub fn is_daily_reset_time() -> bool {
    let current = current_unix_timestamp();
    let seconds_in_day = current % 86400;
    seconds_in_day < 300 // Within 5 minutes of midnight UTC
}

/// Check if it's time for weekly reset (assumes Monday 00:00 UTC reset)
pub fn is_weekly_reset_time() -> bool {
    let current = current_unix_timestamp();
    let days_since_epoch = current / 86400;
    let day_of_week = (days_since_epoch + 4) % 7; // +4 to make Monday = 0
    let seconds_in_day = current % 86400;

    day_of_week == 0 && seconds_in_day < 300 // Monday within 5 minutes of midnight
}

/// Get next daily reset timestamp
pub fn next_daily_reset() -> i64 {
    let current = current_unix_timestamp();
    let current_day = current / 86400;
    (current_day + 1) * 86400 // Next day at 00:00 UTC
}

/// Get next weekly reset timestamp
pub fn next_weekly_reset() -> i64 {
    let current = current_unix_timestamp();
    let days_since_epoch = current / 86400;
    let day_of_week = (days_since_epoch + 4) % 7; // +4 to make Monday = 0
    let days_until_monday = (7 - day_of_week) % 7;
    let next_monday_days = days_since_epoch
        + if days_until_monday == 0 {
            7
        } else {
            days_until_monday
        };
    next_monday_days * 86400
}

/// Calculate mission expiry time based on type
pub fn calculate_mission_expiry(mission_type: &str, start_time: i64) -> i64 {
    match mission_type.to_lowercase().as_str() {
        "daily" => next_daily_reset(),
        "weekly" => next_weekly_reset(),
        "event" => start_time + (7 * 86400), // 7 days for events
        "tutorial" => start_time + (30 * 86400), // 30 days for tutorials
        _ => start_time + (365 * 86400),     // 1 year for permanent missions
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_creation() {
        let ts = Timestamp::now();
        assert!(ts.unix_seconds() > Timestamp::MIN);

        let ts2 = Timestamp::from_unix_seconds(1609459200).unwrap();
        assert_eq!(ts2.unix_seconds(), 1609459200);

        // Invalid timestamp
        assert!(Timestamp::from_unix_seconds(1000000000).is_err()); // Too old
    }

    #[test]
    fn test_timestamp_operations() {
        let ts = Timestamp::from_unix_seconds(1609459200).unwrap();
        let ts2 = ts.add_duration(Duration::from_secs(3600)).unwrap();

        assert_eq!(ts2.unix_seconds(), 1609459200 + 3600);

        let duration = ts2.duration_since(&ts);
        assert_eq!(duration, Duration::from_secs(3600));
    }

    #[test]
    fn test_time_formatting() {
        let current = current_unix_timestamp();
        let past_time = current - 3600; // 1 hour ago
        let future_time = current + 7200; // 2 hours in future

        assert!(format_relative_time(past_time).contains("hour"));
        assert!(format_relative_time(future_time).contains("in"));

        assert!(!format_human_readable(current).is_empty());
    }

    #[test]
    fn test_duration_formatting() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m");
        assert_eq!(format_duration(Duration::from_secs(86401)), "1d 0h");
    }

    #[test]
    fn test_duration_parsing() {
        assert_eq!(parse_duration("30s").unwrap(), Duration::from_secs(30));
        assert_eq!(parse_duration("5m").unwrap(), Duration::from_secs(300));
        assert_eq!(parse_duration("2h").unwrap(), Duration::from_secs(7200));
        assert_eq!(parse_duration("1d").unwrap(), Duration::from_secs(86400));

        assert!(parse_duration("invalid").is_err());
    }

    #[test]
    fn test_day_week_calculations() {
        let ts = 1609459200; // 2021-01-01 00:00:00 UTC (Friday)

        let start_day = start_of_day(ts + 3661); // Add 1h 1m 1s
        assert_eq!(start_day, ts);

        let end_day = end_of_day(ts);
        assert_eq!(end_day, ts + 86399);

        // Test week calculations
        let week_start = start_of_week(ts);
        assert!(week_start <= ts);

        let week_end = end_of_week(ts);
        assert!(week_end > ts);
    }

    #[test]
    fn test_game_time_functions() {
        let daily_reset = next_daily_reset();
        let weekly_reset = next_weekly_reset();
        let current = current_unix_timestamp();

        assert!(daily_reset > current);
        assert!(weekly_reset >= current);

        // Test mission expiry calculations
        let expiry = calculate_mission_expiry("daily", current);
        assert!(expiry > current);
    }

    #[test]
    fn test_timestamp_validation() {
        // Valid timestamp
        assert!(Timestamp::from_unix_seconds(current_unix_timestamp()).is_ok());

        // Too old
        assert!(Timestamp::from_unix_seconds(1000000000).is_err());

        // Too far in future
        let far_future = current_unix_timestamp() + 100000;
        assert!(Timestamp::from_unix_seconds(far_future).is_err());
    }

    #[test]
    fn test_timestamp_comparisons() {
        let ts1 = Timestamp::from_unix_seconds(1609459200).unwrap();
        let ts2 = Timestamp::from_unix_seconds(1609459260).unwrap(); // 1 minute later

        assert!(ts2 > ts1);
        assert!(ts1 < ts2);
        assert_eq!(ts2.duration_since(&ts1), Duration::from_secs(60));
    }
}
