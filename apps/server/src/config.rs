use std::path::PathBuf;
use serde::Deserialize;
use crate::server::ServerConfig;
use crate::scheduler::SchedulerConfig;

#[derive(Deserialize)]
/// Struct representation of the server configuration.
pub struct Config {
    pub server: ServerConfig,
    pub scheduler: SchedulerConfig,
}

/// Parses the configuration file into a `toml::Table` (a hashmap).
pub fn config() -> Result<Config, String> {
    let mut config_dir = PathBuf::from("/etc/celerityd");
    config_dir.push("config.toml");

    config_dir
        .try_exists()
        .map_err(|e| e.to_string())?
        .then(|| -> Result<Config, String> {
            let config = std::fs::read_to_string(config_dir).map_err(|e| e.to_string())?;
            toml::from_str::<Config>(&config).map_err(|e| e.to_string())
        })
        .ok_or("Configuration is invalid")?
}
