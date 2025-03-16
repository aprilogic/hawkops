use crate::error::{HawkOpsError, HawkOpsResult};
use crate::auth::AuthManager;
use reqwest::{Client, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    auth_manager: AuthManager,
}

impl ApiClient {
    pub fn new(auth_manager: AuthManager) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            auth_manager,
        }
    }

    pub async fn get<T>(&self, endpoint: &str) -> HawkOpsResult<T>
    where
        T: DeserializeOwned,
    {
        let auth_header = self.auth_manager.get_auth_header().await?;
        let response = self.client
            .get(endpoint)
            .header("Authorization", auth_header)
            .send()
            .await
            .map_err(|e| HawkOpsError::ApiError(format!("Request failed: {}", e)))?;

        Self::handle_response(response).await
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> HawkOpsResult<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let auth_header = self.auth_manager.get_auth_header().await?;
        let response = self.client
            .post(endpoint)
            .header("Authorization", auth_header)
            .json(body)
            .send()
            .await
            .map_err(|e| HawkOpsError::ApiError(format!("Request failed: {}", e)))?;

        Self::handle_response(response).await
    }

    pub async fn put<T, B>(&self, endpoint: &str, body: &B) -> HawkOpsResult<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let auth_header = self.auth_manager.get_auth_header().await?;
        let response = self.client
            .put(endpoint)
            .header("Authorization", auth_header)
            .json(body)
            .send()
            .await
            .map_err(|e| HawkOpsError::ApiError(format!("Request failed: {}", e)))?;

        Self::handle_response(response).await
    }

    pub async fn delete(&self, endpoint: &str) -> HawkOpsResult<()> {
        let auth_header = self.auth_manager.get_auth_header().await?;
        let response = self.client
            .delete(endpoint)
            .header("Authorization", auth_header)
            .send()
            .await
            .map_err(|e| HawkOpsError::ApiError(format!("Request failed: {}", e)))?;

        match response.status() {
            StatusCode::NO_CONTENT | StatusCode::OK => Ok(()),
            status => Err(HawkOpsError::ApiError(format!(
                "Delete request failed: {}",
                status
            ))),
        }
    }

    async fn handle_response<T>(response: reqwest::Response) -> HawkOpsResult<T>
    where
        T: DeserializeOwned,
    {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response
                    .json()
                    .await
                    .map_err(|e| HawkOpsError::ApiError(format!("Failed to parse response: {}", e)))
            }
            StatusCode::UNAUTHORIZED => {
                Err(HawkOpsError::AuthError("Authentication failed".to_string()))
            }
            StatusCode::FORBIDDEN => {
                Err(HawkOpsError::AuthError("Access denied".to_string()))
            }
            status => {
                let error_msg = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "No error message available".to_string());
                Err(HawkOpsError::ApiError(format!(
                    "Request failed with status {}: {}",
                    status, error_msg
                )))
            }
        }
    }
}

// API Models
#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub organization_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scan {
    pub id: String,
    pub application_id: String,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub findings_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub organization_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
} 