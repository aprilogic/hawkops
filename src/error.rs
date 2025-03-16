use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HawkOpsError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("SSL/TLS error: {0}")]
    SslError(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

pub type HawkOpsResult<T> = Result<T, HawkOpsError>;

// Helper functions for error context
pub trait ErrorContext<T> {
    fn context<C>(self, context: C) -> HawkOpsResult<T>
    where
        C: std::fmt::Display;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> HawkOpsResult<T>
    where
        C: std::fmt::Display,
    {
        self.map_err(|e| {
            HawkOpsError::UnexpectedError(format!("{}: {}", context, e))
        })
    }
}

// Convenience function for creating API errors
pub fn api_error<S: Into<String>>(message: S) -> HawkOpsError {
    HawkOpsError::ApiError(message.into())
}

// Convenience function for creating auth errors
pub fn auth_error<S: Into<String>>(message: S) -> HawkOpsError {
    HawkOpsError::AuthError(message.into())
}

// Convenience function for creating config errors
pub fn config_error<S: Into<String>>(message: S) -> HawkOpsError {
    HawkOpsError::ConfigError(message.into())
}

// Convenience function for creating invalid input errors
pub fn invalid_input<S: Into<String>>(message: S) -> HawkOpsError {
    HawkOpsError::InvalidInput(message.into())
} 