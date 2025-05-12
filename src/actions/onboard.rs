use crate::config::{Config, SortBy, SortOrder};
use std::fs;

pub fn onboarding() -> Result<(), Box<dyn std::error::Error>> {
    let task_path: String =
        cliclack::input("Where should we save the tasks? (Default: ./tasks.json)")
            .placeholder("./tasks.json")
            .default_input("./tasks.json")
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

    fs::write(&task_path, "[]")?;

    let config = Config::new(SortBy::CreatedAt, SortOrder::Ascending, task_path.clone());

    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;

    cliclack::outro("Setup complete!")?;

    Ok(())
}
