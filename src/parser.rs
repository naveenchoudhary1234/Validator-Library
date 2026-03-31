use std::fs;
use crate::config::ServerConfig;
use crate::error::ValidationError;

pub fn parse_toml_file(path: &str) -> Result<ServerConfig, ValidationError> {
    let content = fs::read_to_string(path)
        .map_err(|e| ValidationError::EmptyConfiguration {
            field: format!("Cannot read file '{}': {}", path, e),
        })?;

    toml::from_str(&content)
        .map_err(|e| ValidationError::EmptyConfiguration {
            field: format!("TOML parse error: {}", e),
        })
}