//! Error types for BUNKERVERSE Platform
//! Comprehensive error handling with structured error information

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Result type alias for BUNKERVERSE operations
pub type Result<T> = std::result::Result<T, BunkerVerseError>;

/// Main error type for BUNKERVERSE Platform
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BunkerVerseError {
    /// Validation errors for input data
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// Authentication and authorization errors
    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),

    /// Database and persistence errors
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    /// Network and service communication errors
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    /// Blockchain and smart contract errors
    #[error("Blockchain error: {0}")]
    Blockchain(#[from] BlockchainError),

    /// Payment processing errors
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),

    /// Game logic and business rule errors
    #[error("Game logic error: {0}")]
    GameLogic(#[from] GameLogicError),

    /// External service integration errors
    #[error("External service error: {0}")]
    ExternalService(#[from] ExternalServiceError),

    /// Internal system errors
    #[error("Internal error: {0}")]
    Internal(#[from] InternalError),
}

/// Validation errors for input data and format checking
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationError {
    /// Invalid format for a field
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Value out of acceptable range
    #[error("Value out of range for field '{field}': {value} (expected: {min:?} to {max:?})")]
    OutOfRange {
        field: String,
        value: String,
        min: Option<String>,
        max: Option<String>,
    },

    /// Required field is missing
    #[error("Required field missing: {field}")]
    RequiredField { field: String },

    /// Invalid length for a string or collection
    #[error("Invalid length for field '{field}': {actual} (expected: {min} to {max})")]
    InvalidLength {
        field: String,
        actual: usize,
        min: usize,
        max: usize,
    },

    /// Invalid enum value
    #[error("Invalid enum value for field '{field}': {value}")]
    InvalidEnumValue { field: String, value: String },

    /// Invalid UUID format
    #[error("Invalid UUID format: {value}")]
    InvalidUuid { value: String },

    /// Invalid Ethereum address format
    #[error("Invalid Ethereum address: {address}")]
    InvalidEthereumAddress { address: String },

    /// Invalid timestamp
    #[error("Invalid timestamp: {timestamp} (reason: {reason})")]
    InvalidTimestamp { timestamp: i64, reason: String },

    /// Custom validation error
    #[error("Validation failed: {message}")]
    Custom { message: String },
}

/// Authentication and authorization errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthenticationError {
    /// Invalid credentials
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// Expired token
    #[error("Token expired at {expired_at}")]
    TokenExpired { expired_at: i64 },

    /// Invalid token format or signature
    #[error("Invalid token: {reason}")]
    InvalidToken { reason: String },

    /// Missing authentication
    #[error("Authentication required")]
    AuthenticationRequired,

    /// Insufficient permissions
    #[error("Insufficient permissions: required {required}, has {current}")]
    InsufficientPermissions { required: String, current: String },

    /// Account locked or suspended
    #[error("Account locked: {reason}")]
    AccountLocked { reason: String },

    /// OAuth flow errors
    #[error("OAuth error: {provider} - {message}")]
    OAuthError { provider: String, message: String },

    /// Session errors
    #[error("Session error: {reason}")]
    SessionError { reason: String },
}

/// Database and persistence errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DatabaseError {
    /// Connection failed
    #[error("Database connection failed: {reason}")]
    ConnectionFailed { reason: String },

    /// Query execution failed
    #[error("Query failed: {query} - {reason}")]
    QueryFailed { query: String, reason: String },

    /// Record not found
    #[error("Record not found: {table}.{id}")]
    NotFound { table: String, id: String },

    /// Constraint violation
    #[error("Constraint violation: {constraint} - {details}")]
    ConstraintViolation { constraint: String, details: String },

    /// Transaction failed
    #[error("Transaction failed: {reason}")]
    TransactionFailed { reason: String },

    /// Migration error
    #[error("Database migration failed: {version} - {reason}")]
    MigrationFailed { version: String, reason: String },

    /// Timeout
    #[error("Database operation timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
}

/// Network and service communication errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NetworkError {
    /// HTTP request failed
    #[error("HTTP request failed: {status} - {url}")]
    HttpError { status: u16, url: String },

    /// Connection timeout
    #[error("Connection timeout: {url} after {timeout_ms}ms")]
    Timeout { url: String, timeout_ms: u64 },

    /// DNS resolution failed
    #[error("DNS resolution failed for: {host}")]
    DnsResolution { host: String },

    /// TLS/SSL error
    #[error("TLS error: {reason}")]
    TlsError { reason: String },

    /// Service unavailable
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },

    /// Rate limited
    #[error("Rate limited: {service} - retry after {retry_after_seconds}s")]
    RateLimited {
        service: String,
        retry_after_seconds: u64,
    },

    /// gRPC error
    #[error("gRPC error: {code} - {message}")]
    GrpcError { code: String, message: String },
}

/// Blockchain and smart contract errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockchainError {
    /// Transaction failed
    #[error("Transaction failed: {hash} - {reason}")]
    TransactionFailed { hash: String, reason: String },

    /// Insufficient gas
    #[error("Insufficient gas: required {required}, provided {provided}")]
    InsufficientGas { required: u64, provided: u64 },

    /// Insufficient balance
    #[error("Insufficient balance: required {required} {currency}, available {available}")]
    InsufficientBalance {
        required: String,
        available: String,
        currency: String,
    },

    /// Contract call failed
    #[error("Contract call failed: {contract} - {method} - {reason}")]
    ContractCallFailed {
        contract: String,
        method: String,
        reason: String,
    },

    /// Invalid chain ID
    #[error("Invalid chain ID: expected {expected}, got {actual}")]
    InvalidChainId { expected: u64, actual: u64 },

    /// Block not found
    #[error("Block not found: {block_number}")]
    BlockNotFound { block_number: u64 },

    /// Event parsing failed
    #[error("Event parsing failed: {event_type} - {reason}")]
    EventParsingFailed { event_type: String, reason: String },

    /// Network congestion
    #[error("Network congested: estimated wait time {estimated_minutes} minutes")]
    NetworkCongested { estimated_minutes: u32 },
}

/// Payment processing errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentError {
    /// Payment declined
    #[error("Payment declined: {reason}")]
    PaymentDeclined { reason: String },

    /// Insufficient funds
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: String, available: String },

    /// Invalid payment method
    #[error("Invalid payment method: {payment_method_id}")]
    InvalidPaymentMethod { payment_method_id: String },

    /// Payment processor error
    #[error("Payment processor error: {processor} - {code} - {message}")]
    ProcessorError {
        processor: String,
        code: String,
        message: String,
    },

    /// Currency conversion error
    #[error("Currency conversion failed: {from} to {to}")]
    CurrencyConversion { from: String, to: String },

    /// Refund failed
    #[error("Refund failed: {transaction_id} - {reason}")]
    RefundFailed {
        transaction_id: String,
        reason: String,
    },

    /// Fraud detection
    #[error("Transaction flagged by fraud detection: {reason}")]
    FraudDetection { reason: String },
}

/// Game logic and business rule errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameLogicError {
    /// Player not found
    #[error("Player not found: {player_id}")]
    PlayerNotFound { player_id: String },

    /// NFT not owned by player
    #[error("NFT not owned by player: {nft_id} not owned by {player_id}")]
    NftNotOwned { nft_id: String, player_id: String },

    /// Item not equippable
    #[error("Item not equippable: {item_id} cannot be equipped to {slot}")]
    ItemNotEquippable { item_id: String, slot: String },

    /// Mission not available
    #[error("Mission not available: {mission_id} - {reason}")]
    MissionNotAvailable { mission_id: String, reason: String },

    /// Mission already completed
    #[error("Mission already completed: {mission_id} by {player_id}")]
    MissionAlreadyCompleted {
        mission_id: String,
        player_id: String,
    },

    /// Invalid class change
    #[error("Invalid class change: {from} to {to} - {reason}")]
    InvalidClassChange {
        from: String,
        to: String,
        reason: String,
    },

    /// Marketplace listing error
    #[error("Marketplace listing error: {reason}")]
    MarketplaceListing { reason: String },

    /// Trading restriction
    #[error("Trading restricted: {reason}")]
    TradingRestricted { reason: String },
}

/// External service integration errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExternalServiceError {
    /// IPFS error
    #[error("IPFS error: {operation} - {reason}")]
    IpfsError { operation: String, reason: String },

    /// NAR AI generation error
    #[error("NAR AI error: {reason}")]
    NarError { reason: String },

    /// OAuth provider error
    #[error("OAuth provider error: {provider} - {error_code} - {description}")]
    OAuthProvider {
        provider: String,
        error_code: String,
        description: String,
    },

    /// Email service error
    #[error("Email service error: {reason}")]
    EmailService { reason: String },

    /// Analytics service error
    #[error("Analytics error: {service} - {reason}")]
    Analytics { service: String, reason: String },

    /// CDN error
    #[error("CDN error: {operation} - {reason}")]
    CdnError { operation: String, reason: String },
}

/// Internal system errors
#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InternalError {
    /// Configuration error
    #[error("Configuration error: {setting} - {reason}")]
    Configuration { setting: String, reason: String },

    /// Serialization/deserialization error
    #[error("Serialization error: {format} - {reason}")]
    Serialization { format: String, reason: String },

    /// Thread pool error
    #[error("Thread pool error: {reason}")]
    ThreadPool { reason: String },

    /// Memory allocation error
    #[error("Memory allocation failed: {size} bytes")]
    MemoryAllocation { size: usize },

    /// File system error
    #[error("File system error: {path} - {operation} - {reason}")]
    FileSystem {
        path: String,
        operation: String,
        reason: String,
    },

    /// Unexpected error
    #[error("Unexpected error: {message}")]
    Unexpected { message: String },
}

// ============================================================================
// Error Code Mappings for API Responses
// ============================================================================

/// Error codes for API responses (maps to protobuf ErrorCodeProto)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    Unknown = 1,
    NotFound = 2,
    Unauthorized = 3,
    InvalidInput = 4,
    FeatureDisabled = 5,
    RateLimited = 6,
    InternalError = 7,
    InsufficientBalance = 8,
    TransactionFailed = 9,
}

impl BunkerVerseError {
    /// Convert error to API error code
    pub fn to_error_code(&self) -> ErrorCode {
        match self {
            BunkerVerseError::Validation(_) => ErrorCode::InvalidInput,
            BunkerVerseError::Authentication(auth_err) => match auth_err {
                AuthenticationError::InvalidCredentials
                | AuthenticationError::TokenExpired { .. }
                | AuthenticationError::InvalidToken { .. }
                | AuthenticationError::AuthenticationRequired => ErrorCode::Unauthorized,
                AuthenticationError::InsufficientPermissions { .. }
                | AuthenticationError::AccountLocked { .. } => ErrorCode::Unauthorized,
                _ => ErrorCode::Unknown,
            },
            BunkerVerseError::Database(DatabaseError::NotFound { .. }) => ErrorCode::NotFound,
            BunkerVerseError::Database(_) => ErrorCode::InternalError,
            BunkerVerseError::Network(net_err) => match net_err {
                NetworkError::RateLimited { .. } => ErrorCode::RateLimited,
                NetworkError::ServiceUnavailable { .. } => ErrorCode::InternalError,
                _ => ErrorCode::InternalError,
            },
            BunkerVerseError::Blockchain(bc_err) => match bc_err {
                BlockchainError::InsufficientBalance { .. } => ErrorCode::InsufficientBalance,
                BlockchainError::TransactionFailed { .. } => ErrorCode::TransactionFailed,
                _ => ErrorCode::InternalError,
            },
            BunkerVerseError::Payment(PaymentError::InsufficientFunds { .. }) => {
                ErrorCode::InsufficientBalance
            }
            BunkerVerseError::Payment(_) => ErrorCode::InternalError,
            BunkerVerseError::GameLogic(game_err) => match game_err {
                GameLogicError::PlayerNotFound { .. } => ErrorCode::NotFound,
                GameLogicError::NftNotOwned { .. } => ErrorCode::Unauthorized,
                _ => ErrorCode::InvalidInput,
            },
            _ => ErrorCode::InternalError,
        }
    }

    /// Convert error to user-safe message (no internal details)
    pub fn to_user_message(&self) -> String {
        match self {
            BunkerVerseError::Validation(val_err) => match val_err {
                ValidationError::InvalidFormat(_) => "Invalid input format".to_string(),
                ValidationError::OutOfRange { field, .. } => {
                    format!("Value for {} is out of acceptable range", field)
                }
                ValidationError::RequiredField { field } => {
                    format!("Required field {} is missing", field)
                }
                ValidationError::InvalidLength { field, .. } => {
                    format!("Invalid length for {}", field)
                }
                _ => "Invalid input provided".to_string(),
            },
            BunkerVerseError::Authentication(_) => {
                "Authentication required or insufficient permissions".to_string()
            }
            BunkerVerseError::Database(DatabaseError::NotFound { .. }) => {
                "Requested resource not found".to_string()
            }
            BunkerVerseError::Network(NetworkError::RateLimited { .. }) => {
                "Request rate limit exceeded, please try again later".to_string()
            }
            BunkerVerseError::Blockchain(BlockchainError::InsufficientBalance { .. }) => {
                "Insufficient balance for this operation".to_string()
            }
            BunkerVerseError::GameLogic(GameLogicError::PlayerNotFound { .. }) => {
                "Player not found".to_string()
            }
            BunkerVerseError::GameLogic(GameLogicError::NftNotOwned { .. }) => {
                "You do not own this item".to_string()
            }
            _ => "An error occurred while processing your request".to_string(),
        }
    }

    /// Generate trace-friendly error details (for debugging)
    pub fn to_trace_details(&self) -> String {
        format!("{:?}", self)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::Unknown => write!(f, "UNKNOWN"),
            ErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ErrorCode::Unauthorized => write!(f, "UNAUTHORIZED"),
            ErrorCode::InvalidInput => write!(f, "INVALID_INPUT"),
            ErrorCode::FeatureDisabled => write!(f, "FEATURE_DISABLED"),
            ErrorCode::RateLimited => write!(f, "RATE_LIMITED"),
            ErrorCode::InternalError => write!(f, "INTERNAL_ERROR"),
            ErrorCode::InsufficientBalance => write!(f, "INSUFFICIENT_BALANCE"),
            ErrorCode::TransactionFailed => write!(f, "TRANSACTION_FAILED"),
        }
    }
}

// ============================================================================
// Error Construction Helpers
// ============================================================================

impl ValidationError {
    pub fn invalid_uuid(value: &str) -> Self {
        Self::InvalidUuid {
            value: value.to_string(),
        }
    }

    pub fn invalid_ethereum_address(address: &str) -> Self {
        Self::InvalidEthereumAddress {
            address: address.to_string(),
        }
    }

    pub fn required_field(field: &str) -> Self {
        Self::RequiredField {
            field: field.to_string(),
        }
    }

    pub fn out_of_range<T: fmt::Display>(
        field: &str,
        value: T,
        min: Option<T>,
        max: Option<T>,
    ) -> Self {
        Self::OutOfRange {
            field: field.to_string(),
            value: value.to_string(),
            min: min.map(|v| v.to_string()),
            max: max.map(|v| v.to_string()),
        }
    }
}

impl AuthenticationError {
    pub fn token_expired(expired_at: i64) -> Self {
        Self::TokenExpired { expired_at }
    }

    pub fn invalid_token(reason: &str) -> Self {
        Self::InvalidToken {
            reason: reason.to_string(),
        }
    }

    pub fn insufficient_permissions(required: &str, current: &str) -> Self {
        Self::InsufficientPermissions {
            required: required.to_string(),
            current: current.to_string(),
        }
    }
}

impl DatabaseError {
    pub fn not_found(table: &str, id: &str) -> Self {
        Self::NotFound {
            table: table.to_string(),
            id: id.to_string(),
        }
    }

    pub fn query_failed(query: &str, reason: &str) -> Self {
        Self::QueryFailed {
            query: query.to_string(),
            reason: reason.to_string(),
        }
    }
}

impl GameLogicError {
    pub fn player_not_found(player_id: &str) -> Self {
        Self::PlayerNotFound {
            player_id: player_id.to_string(),
        }
    }

    pub fn nft_not_owned(nft_id: &str, player_id: &str) -> Self {
        Self::NftNotOwned {
            nft_id: nft_id.to_string(),
            player_id: player_id.to_string(),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_mapping() {
        let validation_err =
            BunkerVerseError::Validation(ValidationError::InvalidFormat("test".to_string()));
        assert_eq!(validation_err.to_error_code(), ErrorCode::InvalidInput);

        let auth_err = BunkerVerseError::Authentication(AuthenticationError::InvalidCredentials);
        assert_eq!(auth_err.to_error_code(), ErrorCode::Unauthorized);
    }

    #[test]
    fn test_user_message_safety() {
        let internal_err = BunkerVerseError::Internal(InternalError::Unexpected {
            message: "Database password is 'secret123'".to_string(),
        });

        let user_msg = internal_err.to_user_message();
        assert!(!user_msg.contains("secret123"));
        assert_eq!(user_msg, "An error occurred while processing your request");
    }

    #[test]
    fn test_validation_error_constructors() {
        let err = ValidationError::invalid_uuid("not-a-uuid");
        assert!(matches!(err, ValidationError::InvalidUuid { .. }));

        let err = ValidationError::required_field("player_id");
        assert!(matches!(err, ValidationError::RequiredField { .. }));

        let err = ValidationError::out_of_range("level", 101, Some(1), Some(100));
        assert!(matches!(err, ValidationError::OutOfRange { .. }));
    }

    #[test]
    fn test_error_serialization() {
        let err = BunkerVerseError::Validation(ValidationError::InvalidFormat("test".to_string()));
        let serialized = serde_json::to_string(&err).unwrap();
        let deserialized: BunkerVerseError = serde_json::from_str(&serialized).unwrap();
        assert_eq!(err, deserialized);
    }
}
