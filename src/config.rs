// use std::fs::File;
use serde::Deserialize;
use config;
use config::{Config, ConfigError, Environment, File};
use std::path::PathBuf;
use dirs;

#[derive(Deserialize, Debug)]
pub struct HawkOpsConfig {
    api_key: Option<String>,
    jwt: Option<String>,
    org_id: Option<String>,
    log_level: Option<String>,
    verbosity: Option<u8>,
}

pub fn load_config() -> Result<HawkOpsConfig, ConfigError> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let config_path = home_dir.join(".hawkops/config.yml");
    let builder = Config::builder()
        .add_source(File::from(config_path).required(false).format(config::FileFormat::Yaml))
        .add_source(Environment::with_prefix("HAWK").separator("_"));
    let config = builder.build()?;
    config.try_deserialize::<HawkOpsConfig>()
}
