use crate::config::{Config, SortBy, SortOrder};
use std::fs;
use std::path::Path;

pub fn onboarding() -> Result<String, Box<dyn std::error::Error>> {
    let task_path: String =
        cliclack::input("Where should we save the tasks? (Default: ./tasks)")
            .placeholder("./tasks")
            .default_input("./tasks")
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Please enter a valid path")
                } else if !input.starts_with("./") {
                    Err("Please enter a path that starts with ./")
                } else {
                    Ok(())
                }
            })
            .interact()?;

    let config_path: String =
        cliclack::input("Where should we save the config? (Default: ./config.json")
            .placeholder("./config.json")
            .default_input("./config.json")
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Please enter a valid path")
                } else if !input.starts_with("./") {
                    Err("Please enter a path that starts with ./")
                } else {
                    Ok(())
                }
            })
            .interact()?;

    let config_dir = Path::new(&config_path).parent();
    if let Some(dir) = config_dir {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
    }

    let config = Config::new(SortBy::CreatedAt, SortOrder::Ascending, task_path.clone());

    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;

    cliclack::outro("Setup complete!")?;

    Ok(config_path)
}
