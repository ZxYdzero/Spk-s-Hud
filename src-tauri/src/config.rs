use std::{fs, io::Write, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub password: String,
    pub debug_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost:32456".to_string(),
            password: "default_password".to_string(),
            debug_mode: false,
        }
    }
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
    let config_path = "config.toml";
    if !Path::new(config_path).exists() {
        let default_config = Config::default();
        let toml_str = toml::to_string_pretty(&default_config)?;
        let mut file = fs::File::create(config_path)?;
        file.write_all("# 请注意,IP后面需要加端口号.\n".as_bytes())?;
        file.write_all(toml_str.as_bytes())?;
    }
    let content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}