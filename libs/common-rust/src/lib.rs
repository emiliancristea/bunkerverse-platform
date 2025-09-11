//! BUNKERVERSE Platform - Common Rust Types & Utilities
//! Core domain types, error handling, and shared validation logic

// Re-export for common use

// Re-export modules
pub mod errors;
pub mod time;
pub mod types;
pub mod validation;

pub use errors::*;
pub use time::*;
pub use types::*;
pub use validation::*;

/// Version information for the common library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
    }
}
