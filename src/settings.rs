use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use slog::Logger;
use std::env;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppOptions {
    pub listen_address: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLOptions {
    pub playground_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub app: AppOptions,
    pub graphql: GraphQLOptions,
}

impl Settings {
    pub fn new(logger: &Logger) -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        // base config
        settings.merge(File::with_name("config/base.toml"))?;

        // read config for ENVIRONMENT (or stage if not present)
        let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "stage".into());
        let config_path = format!("config/{}/config.toml", env);

        if !Path::new(&config_path).exists() {
            error!(logger, "config"; "config_path" => &config_path, "message" => "config_path not found on disk");
        } else {
            info!(logger, "config"; "config_path" => &config_path);
        }

        settings.merge(File::with_name(&config_path))?;

        // freezes settings
        settings.try_into()
    }
}
