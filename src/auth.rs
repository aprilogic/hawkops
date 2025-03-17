use crate::error::{HawkOpsError, HawkOpsResult};
use crate::config::HawkOpsConfig;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use std::sync::Arc;
use base64::Engine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    expires_in: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct AuthManager {
    config: Arc<RwLock<HawkOpsConfig>>,
    client: Client,
}

impl AuthManager {
    pub fn new(config: HawkOpsConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config: Arc::new(RwLock::new(config)),
            client,
        }
    }

    pub async fn authenticate(&self) -> HawkOpsResult<String> {
        let config = self.config.read().await;
        
        if let Some(api_key) = &config.api.api_key {
            let url = format!("{}/api/v1/auth/login", config.api.base_url);
            
            let response = self.client
                .get(&url)
                .header("X-ApiKey", api_key)
                .send()
                .await
                .map_err(|e| HawkOpsError::AuthError(format!("Failed to authenticate: {}", e)))?;

            if !response.status().is_success() {
                return Err(HawkOpsError::AuthError(format!(
                    "Authentication failed: {}",
                    response.status()
                )));
            }

            let auth_response: AuthResponse = response
                .json()
                .await
                .map_err(|e| HawkOpsError::AuthError(format!("Failed to parse auth response: {}", e)))?;

            // Update config with new tokens
            drop(config); // Release the read lock
            let mut config = self.config.write().await;
            config.update_auth_tokens(auth_response.token.clone(), auth_response.refresh_token)?;

            Ok(auth_response.token)
        } else {
            Err(HawkOpsError::AuthError("API key not found in configuration".to_string()))
        }
    }

    pub async fn get_valid_token(&self) -> HawkOpsResult<String> {
        let config = self.config.read().await;
        
        if let Some(token) = &config.auth.access_token {
            if !Self::is_token_expired(token) {
                return Ok(token.clone());
            }
            
            if config.auth.auto_refresh {
                drop(config);
                return self.refresh_token().await;
            }
        }
        
        drop(config);
        self.authenticate().await
    }

    async fn refresh_token(&self) -> HawkOpsResult<String> {
        let config = self.config.read().await;
        
        let refresh_token = config.auth.refresh_token.as_ref()
            .ok_or_else(|| HawkOpsError::AuthError("No refresh token available".to_string()))?;

        let url = format!("{}/api/v1/auth/refresh", config.api.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", refresh_token))
            .send()
            .await
            .map_err(|e| HawkOpsError::AuthError(format!("Failed to refresh token: {}", e)))?;

        if !response.status().is_success() {
            return Err(HawkOpsError::AuthError(format!(
                "Token refresh failed: {}",
                response.status()
            )));
        }

        let auth_response: AuthResponse = response
            .json()
            .await
            .map_err(|e| HawkOpsError::AuthError(format!("Failed to parse refresh response: {}", e)))?;

        // Update config with new tokens
        drop(config);
        let mut config = self.config.write().await;
        config.update_auth_tokens(auth_response.token.clone(), auth_response.refresh_token)?;

        Ok(auth_response.token)
    }

    fn is_token_expired(token: &str) -> bool {
        // JWT token validation logic
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return true;
        }

        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1].as_bytes())
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok());

        if let Some(payload) = payload {
            if let Some(exp) = payload["exp"].as_i64() {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64;
                return exp <= now;
            }
        }

        true // If we can't validate the token, consider it expired
    }

    pub async fn get_auth_header(&self) -> HawkOpsResult<header::HeaderValue> {
        let token = self.get_valid_token().await?;
        header::HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|e| HawkOpsError::AuthError(format!("Failed to create auth header: {}", e)))
    }
}

