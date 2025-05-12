use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result as IoResult;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SortBy {
    Name,
    Status,
    CreatedAt,
    UpdatedAt,
    DueDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
}

impl Config {
    pub fn new(sort_by: SortBy, sort_order: SortOrder) -> Self {
        Config {
            sort_by,
            sort_order,
        }
    }

    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> IoResult<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }

    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(&path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn ensure_exists<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            let default = Config::new(SortBy::CreatedAt, SortOrder::Ascending);
            default.save_to(path_ref)?;
            cliclack::log::info(format!(
                "⚠️  Config file not found at '{}'. Recreating default config.",
                path_ref.display()
            ))?;
        }

        Ok(())
    }
}
