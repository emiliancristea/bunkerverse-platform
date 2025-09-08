use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported OAuth providers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OAuthProvider {
    Google,
    GitHub,
    Discord,
    Microsoft,
}

/// OAuth ID token claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdTokenClaims {
    pub iss: String,           // Issuer
    pub sub: String,           // Subject (user ID)
    pub aud: String,           // Audience
    pub exp: i64,              // Expiration time
    pub iat: i64,              // Issued at time
    pub email: String,         // User email
    pub email_verified: bool,  // Email verification status
    pub name: Option<String>,  // User's full name
    pub picture: Option<String>, // Profile picture URL
    pub nonce: Option<String>, // Nonce for replay protection
}

/// OAuth validation result
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub claims: Option<IdTokenClaims>,
    pub error: Option<String>,
    pub provider: OAuthProvider,
    pub validation_time: DateTime<Utc>,
}

/// OAuth provider configuration
#[derive(Debug, Clone)]
struct ProviderConfig {
    pub issuer: String,
    pub jwks_uri: String,
    pub expected_audience: String,
}

/// OAuth validation service
pub struct OAuthValidation {
    provider: OAuthProvider,
    config: ProviderConfig,
    mock_mode: bool,
}

impl OAuthValidation {
    /// Create new OAuth validator for provider
    pub async fn new(provider: OAuthProvider) -> Result<Self> {
        let config = match provider {
            OAuthProvider::Google => ProviderConfig {
                issuer: "https://accounts.google.com".to_string(),
                jwks_uri: "https://www.googleapis.com/oauth2/v3/certs".to_string(),
                expected_audience: "bunkerverse-client-id".to_string(),
            },
            OAuthProvider::GitHub => ProviderConfig {
                issuer: "https://token.actions.githubusercontent.com".to_string(),
                jwks_uri: "https://token.actions.githubusercontent.com/.well-known/jwks".to_string(),
                expected_audience: "bunkerverse-github".to_string(),
            },
            OAuthProvider::Discord => ProviderConfig {
                issuer: "https://discord.com".to_string(),
                jwks_uri: "https://discord.com/api/oauth2/token".to_string(),
                expected_audience: "bunkerverse-discord".to_string(),
            },
            OAuthProvider::Microsoft => ProviderConfig {
                issuer: "https://login.microsoftonline.com".to_string(),
                jwks_uri: "https://login.microsoftonline.com/common/discovery/v2.0/keys".to_string(),
                expected_audience: "bunkerverse-microsoft".to_string(),
            },
        };

        println!("✅ OAuth validator initialized for {:?}", provider);
        
        Ok(Self {
            provider,
            config,
            mock_mode: true, // Enable mock mode for PoC
        })
    }

    /// Validate OAuth ID token
    pub async fn validate_token(&self, id_token: &str, expected_nonce: &str) -> Result<IdTokenClaims> {
        if self.mock_mode {
            return self.mock_validate_token(id_token, expected_nonce).await;
        }

        // Real validation would involve:
        // 1. Fetch JWKS from provider
        // 2. Verify JWT signature
        // 3. Validate claims (iss, aud, exp, iat)
        // 4. Check nonce
        
        // For now, return mock validation
        self.mock_validate_token(id_token, expected_nonce).await
    }

    /// Mock token validation for PoC
    async fn mock_validate_token(&self, id_token: &str, expected_nonce: &str) -> Result<IdTokenClaims> {
        if id_token.is_empty() {
            return Err(anyhow!("Empty ID token"));
        }

        // Simulate processing delay
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Generate mock claims based on provider
        let claims = match self.provider {
            OAuthProvider::Google => self.create_google_mock_claims(expected_nonce),
            OAuthProvider::GitHub => self.create_github_mock_claims(expected_nonce),
            OAuthProvider::Discord => self.create_discord_mock_claims(expected_nonce),
            OAuthProvider::Microsoft => self.create_microsoft_mock_claims(expected_nonce),
        };

        Ok(claims)
    }

    fn create_google_mock_claims(&self, nonce: &str) -> IdTokenClaims {
        let now = Utc::now().timestamp();
        IdTokenClaims {
            iss: "https://accounts.google.com".to_string(),
            sub: "google_user_123456789".to_string(),
            aud: "bunkerverse-client-id".to_string(),
            exp: now + 3600, // 1 hour
            iat: now,
            email: "user@gmail.com".to_string(),
            email_verified: true,
            name: Some("Bunkerverse User".to_string()),
            picture: Some("https://lh3.googleusercontent.com/mock".to_string()),
            nonce: Some(nonce.to_string()),
        }
    }

    fn create_github_mock_claims(&self, nonce: &str) -> IdTokenClaims {
        let now = Utc::now().timestamp();
        IdTokenClaims {
            iss: "https://token.actions.githubusercontent.com".to_string(),
            sub: "github_user_987654321".to_string(),
            aud: "bunkerverse-github".to_string(),
            exp: now + 3600,
            iat: now,
            email: "developer@github.local".to_string(),
            email_verified: true,
            name: Some("GitHub Developer".to_string()),
            picture: Some("https://avatars.githubusercontent.com/mock".to_string()),
            nonce: Some(nonce.to_string()),
        }
    }

    fn create_discord_mock_claims(&self, nonce: &str) -> IdTokenClaims {
        let now = Utc::now().timestamp();
        IdTokenClaims {
            iss: "https://discord.com".to_string(),
            sub: "discord_user_555666777".to_string(),
            aud: "bunkerverse-discord".to_string(),
            exp: now + 3600,
            iat: now,
            email: "gamer@discord.gg".to_string(),
            email_verified: true,
            name: Some("Discord Gamer".to_string()),
            picture: Some("https://cdn.discordapp.com/avatars/mock.png".to_string()),
            nonce: Some(nonce.to_string()),
        }
    }

    fn create_microsoft_mock_claims(&self, nonce: &str) -> IdTokenClaims {
        let now = Utc::now().timestamp();
        IdTokenClaims {
            iss: "https://login.microsoftonline.com".to_string(),
            sub: "microsoft_user_111222333".to_string(),
            aud: "bunkerverse-microsoft".to_string(),
            exp: now + 3600,
            iat: now,
            email: "user@outlook.com".to_string(),
            email_verified: true,
            name: Some("Microsoft User".to_string()),
            picture: Some("https://graph.microsoft.com/mock/photo".to_string()),
            nonce: Some(nonce.to_string()),
        }
    }

    /// Validate token claims
    pub fn validate_claims(&self, claims: &IdTokenClaims) -> Result<ValidationResult> {
        let now = Utc::now().timestamp();
        
        // Check expiration
        if claims.exp <= now {
            return Ok(ValidationResult {
                is_valid: false,
                claims: None,
                error: Some("Token expired".to_string()),
                provider: self.provider.clone(),
                validation_time: Utc::now(),
            });
        }

        // Check issuer
        if !claims.iss.contains(&self.get_expected_issuer()) {
            return Ok(ValidationResult {
                is_valid: false,
                claims: None,
                error: Some("Invalid issuer".to_string()),
                provider: self.provider.clone(),
                validation_time: Utc::now(),
            });
        }

        // Check audience
        if claims.aud != self.config.expected_audience {
            return Ok(ValidationResult {
                is_valid: false,
                claims: None,
                error: Some("Invalid audience".to_string()),
                provider: self.provider.clone(),
                validation_time: Utc::now(),
            });
        }

        // Check if issued in future
        if claims.iat > now + 300 { // Allow 5 min clock skew
            return Ok(ValidationResult {
                is_valid: false,
                claims: None,
                error: Some("Token issued in future".to_string()),
                provider: self.provider.clone(),
                validation_time: Utc::now(),
            });
        }

        Ok(ValidationResult {
            is_valid: true,
            claims: Some(claims.clone()),
            error: None,
            provider: self.provider.clone(),
            validation_time: Utc::now(),
        })
    }

    fn get_expected_issuer(&self) -> String {
        match self.provider {
            OAuthProvider::Google => "accounts.google.com".to_string(),
            OAuthProvider::GitHub => "token.actions.githubusercontent.com".to_string(),
            OAuthProvider::Discord => "discord.com".to_string(),
            OAuthProvider::Microsoft => "login.microsoftonline.com".to_string(),
        }
    }

    /// Get provider metadata
    pub fn get_provider_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("provider".to_string(), format!("{:?}", self.provider));
        info.insert("issuer".to_string(), self.config.issuer.clone());
        info.insert("jwks_uri".to_string(), self.config.jwks_uri.clone());
        info.insert("audience".to_string(), self.config.expected_audience.clone());
        info.insert("mock_mode".to_string(), self.mock_mode.to_string());
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_oauth_validation_creation() {
        let validator = OAuthValidation::new(OAuthProvider::Google).await.unwrap();
        assert_eq!(validator.provider, OAuthProvider::Google);
        assert!(validator.mock_mode);
    }

    #[tokio::test]
    async fn test_mock_token_validation() {
        let validator = OAuthValidation::new(OAuthProvider::Google).await.unwrap();
        let nonce = "test_nonce_123";
        let claims = validator.validate_token("mock_token", nonce).await.unwrap();
        
        assert_eq!(claims.email, "user@gmail.com");
        assert_eq!(claims.nonce.as_ref().unwrap(), nonce);
        assert!(claims.email_verified);
    }

    #[tokio::test]
    async fn test_claims_validation() {
        let validator = OAuthValidation::new(OAuthProvider::Google).await.unwrap();
        let nonce = "test_nonce_456";
        let claims = validator.validate_token("mock_token", nonce).await.unwrap();
        
        let validation = validator.validate_claims(&claims).unwrap();
        assert!(validation.is_valid);
        assert!(validation.error.is_none());
    }

    #[tokio::test]
    async fn test_expired_token() {
        let validator = OAuthValidation::new(OAuthProvider::GitHub).await.unwrap();
        
        let mut claims = validator.validate_token("mock_token", "nonce").await.unwrap();
        claims.exp = Utc::now().timestamp() - 3600; // 1 hour ago
        
        let validation = validator.validate_claims(&claims).unwrap();
        assert!(!validation.is_valid);
        assert!(validation.error.is_some());
    }

    #[tokio::test] 
    async fn test_provider_info() {
        let validator = OAuthValidation::new(OAuthProvider::Discord).await.unwrap();
        let info = validator.get_provider_info();
        
        assert_eq!(info.get("provider").unwrap(), "Discord");
        assert!(info.contains_key("issuer"));
        assert!(info.contains_key("jwks_uri"));
        assert_eq!(info.get("mock_mode").unwrap(), "true");
    }
}