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
        let (config_path, task_path) = actions::onboarding()?;
        let metadata_json = serde_json::json!({
            "config_path": config_path,
            "task_path": task_path
        });
        fs::write(pointer_path, serde_json::to_string_pretty(&metadata_json)?)?;
        config_path
    };

    Ok(config_path)
}

#[allow(dead_code)]
pub enum Pointers {
    Config,
    Task,
}

pub fn read_pointers(current_pointer: Pointers) -> Result<String, Box<dyn std::error::Error>> {
    let pointer_path: &'static str = "./src/config_pointer.json";
    let pointer_contents: String = fs::read_to_string(pointer_path)?;
    let parsed: serde_json::Value = serde_json::from_str(&pointer_contents)?;

    let pointer_key: &'static str = match current_pointer {
        Pointers::Config => "config_path",
        Pointers::Task => "task_path",
    };

    let pointer_value: String = parsed[pointer_key]
        .as_str()
        .unwrap_or("./config.json")
        .to_string();

    Ok(pointer_value)
}

pub fn get_next_task_id(task_dir: &str) -> Result<u32, Box<dyn std::error::Error>> {
    fs::create_dir_all(task_dir)?; // Ensure directory exists

    let mut max_id = 0;
    for entry in fs::read_dir(task_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(id) = stem.parse::<u32>() {
                    if id >= max_id {
                        max_id = id + 1;
                    }
                }
            }
        }
    }

    Ok(max_id)
}
