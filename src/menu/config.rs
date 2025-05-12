use crate::config;
use crate::utils::resolver;

pub fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = resolver()?;
    let mut config = config::Config::load_from(&config_path)?;

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
            .item("return", "Return", "Return to main menu")
            .interact()?;

        match config_menu {
            "sort_by" => {
                let sort_by_menu = cliclack::select("Sort By Menu")
                    .item("name", "Name", "")
                    .item("status", "Status", "")
                    .item("created_at", "Created At", "")
                    .item("updated_at", "Updated At", "")
                    .item("due_date", "Due Date", "")
                    .interact()?;

                match sort_by_menu {
                    "name" => config.sort_by = config::SortBy::Name,
                    "status" => config.sort_by = config::SortBy::Status,
                    "created_at" => config.sort_by = config::SortBy::CreatedAt,
                    "updated_at" => config.sort_by = config::SortBy::UpdatedAt,
                    "due_date" => config.sort_by = config::SortBy::DueDate,
                    _ => unreachable!("Unexpected sort by option"),
                }

                config.save_to(&config_path)?;
            }
            "sort_order" => {
                let sort_order_menu = cliclack::select("Sort Order Menu")
                    .item("asc", "Ascending", "")
                    .item("desc", "Descending", "")
                    .interact()?;

                match sort_order_menu {
                    "asc" => config.sort_order = config::SortOrder::Ascending,
                    "desc" => config.sort_order = config::SortOrder::Descending,
                    _ => unreachable!("Unexpected sort order option"),
                }

                config.save_to(&config_path)?;
            }
            "return" => {
                break;
            }
            _ => unreachable!("Unexpected menu option"),
        }
    }

    Ok(())
}
