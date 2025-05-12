use crate::config::{Config, SortOrder};
use std::fs;

pub fn onboarding() -> Result<(), Box<dyn std::error::Error>> {
    cliclack::intro("Let's set up your configuration paths.")?;

    let task_path: String = cliclack::input("Where should we save the tasks?")
        .placeholder("./tasks.json")
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

    let config_path: String = cliclack::input("Where should we save the config?")
        .placeholder("./config.json")
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
    
    let config = Config::new(
        "date".to_string(),
        SortOrder::Asc,
        task_path.clone(),
    );


    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;

    cliclack::outro("Setup complete!")?;

    Ok(())
}
