use crate::error::{HawkOpsError, HawkOpsResult};
use config::{Config, Environment, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HawkOpsConfig {
    #[serde(default)]
    pub api: ApiConfig,
    #[serde(default)]
    pub auth: AuthConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    #[serde(default = "default_api_url")]
    pub base_url: String,
    pub api_key: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    #[serde(default = "default_token_refresh")]
    pub auto_refresh: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    pub file: Option<PathBuf>,
}

impl Default for HawkOpsConfig {
    fn default() -> Self {
        Self {
            api: ApiConfig::default(),
            auth: AuthConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: default_api_url(),
            api_key: None,
            timeout_seconds: default_timeout(),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            access_token: None,
            refresh_token: None,
            auto_refresh: default_token_refresh(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            file: None,
        }
    }
}

fn default_api_url() -> String {
    "https://api.stackhawk.com".to_string()
}

fn default_timeout() -> u64 {
    30
}

fn default_token_refresh() -> bool {
    true
}

fn default_log_level() -> String {
    "info".to_string()
}

impl HawkOpsConfig {
    pub fn load() -> HawkOpsResult<Self> {
        let config_path = Self::config_path()?;
        let mut builder = Config::builder();

        // Load default config
        builder = builder.add_source(Config::try_from(&Self::default())
            .map_err(|e| HawkOpsError::ConfigError(e.to_string()))?);

        // Load config file if it exists
        if config_path.exists() {
            builder = builder.add_source(File::from(config_path));
        }

        // Load environment variables (prefixed with HAWKOPS_)
        builder = builder.add_source(Environment::with_prefix("HAWKOPS").separator("_"));

        // Build the config
        let config = builder
            .build()
            .map_err(|e| HawkOpsError::ConfigError(e.to_string()))?;

        // Convert to our config type
        config
            .try_deserialize()
            .map_err(|e| HawkOpsError::ConfigError(e.to_string()))
    }

    pub fn save(&self) -> HawkOpsResult<()> {
        let config_path = Self::config_path()?;

        // Ensure the config directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| HawkOpsError::ConfigError(format!("Failed to create config directory: {}", e)))?;
        }

        // Serialize and save the config
        let config_str = toml::to_string_pretty(self)
            .map_err(|e| HawkOpsError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(&config_path, config_str)
            .map_err(|e| HawkOpsError::ConfigError(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    fn config_path() -> HawkOpsResult<PathBuf> {
        let proj_dirs = ProjectDirs::from("io", "aprilogic", "hawkops")
            .ok_or_else(|| HawkOpsError::ConfigError("Failed to determine config directory".to_string()))?;

        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    pub fn update_auth_tokens(&mut self, access_token: String, refresh_token: Option<String>) -> HawkOpsResult<()> {
        self.auth.access_token = Some(access_token);
        self.auth.refresh_token = refresh_token;
        self.save()
    }
}
