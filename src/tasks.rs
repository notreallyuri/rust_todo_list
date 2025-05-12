use crate::utils::{Pointers, read_pointers, get_next_task_id};
use chrono::{DateTime, Utc};
use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    id: u32,
    title: String,
    description: String,
    content: String,
    status: String,
    due_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        id: u32,
        title: String,
        description: String,
        content: String,
        status: String,
        due_date: Option<DateTime<Utc>>,
    ) -> Self {
        Task {
            id,
            title,
            description,
            content,
            status,
            due_date,
            created_at: Utc::now(),
        }
    }

    pub fn create(
        title: String,
        description: String,
        content: String,
        status: String,
        due_date: Option<DateTime<Utc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let task_dir = read_pointers(Pointers::Task)?;
        let task_id = get_next_task_id(&task_dir)?;

        let task = Task::new(task_id, title, description, content, status, due_date);

        fs::create_dir_all(&task_dir)?;

        let file_path = std::path::Path::new(&task_dir).join(format!("{}.json", task_id));
        let json = serde_json::to_string_pretty(&task)?;
        fs::write(&file_path, json)?;
        cliclack::log::info(format!("✅ Task created at '{}'", file_path.display()))?;
        Ok(())
    }

    pub fn display(&self, id: u32) {
        if self.id != id {
            println!("Task with ID {} not found.", id);
            return;
        }

        println!("Task ID: {}", self.id);
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Content: {}", self.content);
        println!("Status: {}", self.status);
        if let Some(due_date) = &self.due_date {
            println!("Due Date: {}", due_date);
        }
        println!("Created At: {}", self.created_at);
    }
}
