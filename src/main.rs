mod actions;
mod config;
mod menu;
mod tasks;
mod utils;

use crate::utils::resolver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = resolver()?;
    cliclack::log::info(format!("✅ Config loaded from '{}'", config_path))?;
    actions::menu::main_menu()?;
    Ok(())
}
