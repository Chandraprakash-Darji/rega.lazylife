/// Prints the config path to stdout
pub fn print_config_path() {
    if let Some(path) = get_config_path() {
        println!("Config path: {}", path.display());
    } else {
        println!("Could not determine config path");
    }
}
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub data_path: PathBuf,
}

pub fn get_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(".config").join("lazylife").join("config.yaml"))
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = get_config_path().ok_or("Could not find config directory")?;
    let content = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path().ok_or("Could not find config directory")?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
    let content = serde_yaml::to_string(self)?;
    fs::write(config_path, content)?;
    Ok(())
    }
}