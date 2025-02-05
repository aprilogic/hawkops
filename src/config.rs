use std::fs::File;
use serde::Deserialize;
use config;
use config::{Config, ConfigError, Environment};
use std::path::PathBuf;
use dirs;

#[derive(Deserialize)]
pub struct HawkOpsConfig {
    api_key: String,
    jwt: String,
    org_id: String,
    log_level: String,
    verbosity: u8,
}

pub fn load_config() -> Result<HawkOpsConfig, ConfigError> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let config_path = home_dir.join(".hawkops/config.yml");
    let mut builder = Config::builder()
        .add_source(File::from(config_path).required(false))
        .add_source(Environment::with_prefix("HAWK").separator("_"));
    let config = builder.build()?;
    config.try_deserialize::<HawkOpsConfig>()
}
