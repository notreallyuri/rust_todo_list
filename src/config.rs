use serde::{Deserialize, Serialize};
use std::fs;

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
    pub data_path: String,
}

impl Config {
    pub fn load() -> Self {
        let contents = fs::read_to_string("config.json").unwrap_or_else(|_| {
            String::from(r#"{"sort_order":"asc","data_path":"tasks.json","sort_by":"date"}"#)
        });
        serde_json::from_str(&contents).expect("Invalid config file")
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).expect("Failed to serialize config");
        fs::write("config.json", json).expect("Could not write config file");
    }

    pub fn new(sort_by: SortBy, sort_order: SortOrder, data_path: String) -> Self {
        Config {
            sort_by,
            sort_order,
            data_path,
        }
    }
}
