use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs;


pub fn load_config(path: &str, format: Option<&str>) -> Result<JsonValue, String> {
    let content = fs::read_to_string(path).map_err(|_| "Failed to read file")?;
    match format.unwrap_or_else(|| path.split('.').last().unwrap_or("")) {
        "json" => serde_json::from_str(&content).map_err(|_| "Invalid JSON format"),
        "toml" => toml::from_str(&content).map_err(|_| "Invalid TOML format"),
        "yaml" => serde_yaml::from_str(&content).map_err(|_| "Invalid YAML format"),
        _ => Err("Unsupported format".to_string()),
    }
}