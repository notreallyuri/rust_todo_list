use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ToDoStatus {
    NotStarted,
    Pending,
    Completed,
}

fn default_id() -> Uuid {
    Uuid::new_v4()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    #[serde(default = "default_id")]
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: ToDoStatus,
}

impl ToDo {
    pub fn new(
        title: String,
        description: Option<String>,
        content: Option<String>,
        due_date: Option<DateTime<Utc>>,
    ) -> ToDo {
        ToDo {
            id: Uuid::new_v4(),
            title,
            description,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date,
            status: ToDoStatus::NotStarted,
        }
    }

    pub fn save(todos: &[ToDo]) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(todos)?;
        let mut file = fs::File::create("tasks.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn add(self) -> std::io::Result<()> {
        let mut todos = ToDo::find_all();
        todos.push(self);
        ToDo::save(&todos)
    }

    pub fn update(id: Uuid, uptated_todo: ToDo) -> std::io::Result<()> {
        let mut todos = ToDo::find_all();

        if let Some(pos) = todos.iter().position(|todo| todo.id == id) {
            todos[pos] = uptated_todo;
            ToDo::save(&todos)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "ToDo item not found",
            ))
        }
    }

    pub fn find_unique(id: Uuid) -> Option<ToDo> {
        let data = fs::read_to_string("tasks.json").ok()?;
        let todos: Vec<ToDo> = serde_json::from_str(&data).ok()?;

        todos.into_iter().find(|todo| todo.id == id)
    }

    pub fn find_all() -> Vec<ToDo> {
        match fs::read_to_string("tasks.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => vec![],
        }
    }
}
