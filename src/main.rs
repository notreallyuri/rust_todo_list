mod actions;
mod config;
mod storage;
mod todo;
mod menu;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    if !fs::metadata("config.json").is_ok() {
        cliclack::intro("Config file not found. Let's set up your configuration paths.")?;
        actions::onboarding()?;
    } else {
        let config = config::Config::load();
        cliclack::intro(format!("Config loaded: {:?}", config))?;
    }

    let main_menu = menu::menu()?;

    Ok(())
}
