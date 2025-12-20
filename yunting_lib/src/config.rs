use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::ConfigError;

const CONFIG_FILE: &str = "yunting.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YuntingConfig {
    pub api_key: String,
    pub provinces: Vec<u64>,
}

impl Default for YuntingConfig {
    fn default() -> Self {
        YuntingConfig {
            api_key: "f0fc4c668392f9f9a447e48584c214ee".to_string(),
            provinces: vec![],
        }
    }
}

pub fn load_or_create_config(config_path: &Path) -> Result<YuntingConfig, ConfigError> {
    let config_file = config_path.join(CONFIG_FILE);
    if std::fs::metadata(&config_file).is_err() {
        let default_config = YuntingConfig::default();
        let toml_str = toml::to_string(&default_config)?;
        std::fs::write(&config_file, toml_str)?;
        return Ok(default_config);
    }

    let config_str = std::fs::read_to_string(&config_file)?;
    Ok(toml::from_str(&config_str)?)
}

pub fn load_config(config_path: &Path) -> Result<YuntingConfig, ConfigError> {
    let config_file = config_path.join(CONFIG_FILE);
    let config_str = std::fs::read_to_string(&config_file)?;
    Ok(toml::from_str(&config_str)?)
}

pub fn save_config(config_path: &Path, provinces: Vec<u64>) -> Result<(), ConfigError> {
    let config_file = config_path.join(CONFIG_FILE);
    let config = YuntingConfig {
        provinces,
        ..Default::default()
    };
    let toml_str = toml::to_string(&config)?;
    std::fs::write(&config_file, toml_str)?;
    Ok(())
}
