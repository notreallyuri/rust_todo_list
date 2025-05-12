use crate::actions;
use crate::config;
use std::fs;

pub fn resolver() -> Result<String, Box<dyn std::error::Error>> {
    let pointer_path = "./src/config_pointer.json";

    let config_path = if fs::metadata(pointer_path).is_ok() {
        let pointer_contents = fs::read_to_string(pointer_path)?;
        let parsed: serde_json::Value = serde_json::from_str(&pointer_contents)?;
        let config_path = parsed["config_path"]
            .as_str()
            .unwrap_or("./config.json")
            .to_string();

        config::Config::ensure_exists(&config_path)?;

        config_path
    } else {
        cliclack::intro("First time setup: creating your configuration")?;
        let config_path = actions::onboarding()?; // Now return path from onboarding
        let metadata_json = serde_json::json!({ "config_path": config_path });
        fs::write(pointer_path, serde_json::to_string_pretty(&metadata_json)?)?;
        config_path
    };

    Ok(config_path)
}
