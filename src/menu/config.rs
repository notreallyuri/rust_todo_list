use crate::config::Config;
use crate::resolver::resolver;

pub fn handle_config_menu() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = resolver()?;
    let config = Config::load_from(&config_path)?;

    loop {
        let config_menu = cliclack::select("Configuration Menu")
            .item(
                "sort_by",
                "Sort By",
                format!("Current sorted by: {:?}", config.sort_by),
            )
            .item(
                "sort_order",
                "Sort Order",
                format!("Current sort order: {:?}", config.sort_order),
            )
            .item(
                "data_path",
                "Data Path",
                format!("Current data path: {}", config.data_path),
            )
            .item("return", "Return", "Return to main menu")
            .interact()?;

        match config_menu {
            "sort_by" => {}
            "sort_order" => {}
            "data_path" => {}
            "return" => {
                break;
            }
            _ => unreachable!("Unexpected menu option"),
        }
    }

    Ok(())
}
