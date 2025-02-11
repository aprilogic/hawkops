// use std::fs::File;
use serde::Deserialize;
use config;
use config::{Config, ConfigError, Environment, File};
use std::path::PathBuf;
use dirs;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct HawkOpsConfig {
    pub api_key: Option<String>,
    pub org_id: Option<String>,
    pub log_level: Option<String>,
    pub verbosity: Option<u8>,
}

pub fn load_config() -> Result<HawkOpsConfig, ConfigError> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    println!("home_dir set to {:?}", home_dir);
    let config_path = home_dir.join(".hawkops/config.yml");
    println!("config_path set to {:?}", config_path);
    let builder = Config::builder()
        .add_source(File::from(config_path)
            .required(false)
            .format(config::FileFormat::Yaml)
        )
        .add_source(Environment::with_prefix("HAWK").separator("_"));
    println!("builder set to {:?}", builder);
    let config = builder.build()?;
    println!("config set to {:?}", config);
    config.try_deserialize::<HawkOpsConfig>()
}
