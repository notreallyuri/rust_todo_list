use crate::utils::{Pointers, get_next_task_id, read_pointers};
use chrono::{DateTime, Utc};
use std::{fs, path::PathBuf, vec};

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

    fn load_tasks(task_dir: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut tasks: Vec<Task> = vec![];

        for entry in fs::read_dir(task_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|e| e == "json") {
                let content = fs::read_to_string(&path)?;
                let task: Task = serde_json::from_str(&content)?;
                tasks.push(task);
            }
        }

        Ok(tasks)
    }

    pub fn read_task() -> Result<(), Box<dyn std::error::Error>> {
        let task_dir = read_pointers(Pointers::Task)?;
        let mut items: Vec<(String, String, String)> = vec![];
        let tasks: Vec<Task> = Task::load_tasks(&task_dir)?;

        for task in tasks {
            let id_str: String = task.id.to_string();
            let label: String = format!("{}: {}", task.id, task.title);
            let hint: String = format!("Status: {}", task.status);

            items.push((id_str, label, hint));
        }

        if items.is_empty() {
            cliclack::log::info("📭 No tasks found.")?;
            return Ok(());
        }

        let selected = cliclack::select("Select a task")
            .items(&items)
            .item("return".to_string(), "Return", "")
            .interact()?;

        if selected == "return" {
            return Ok(());
        }

        let task_id: u32 = selected.parse::<u32>()?;
        let file_path: PathBuf = std::path::Path::new(&task_dir).join(format!("{}.json", task_id));
        let content = fs::read_to_string(&file_path)?;
        let task: Task = serde_json::from_str(&content)?;

        cliclack::log::info(format!("📌 Title: {}", task.title))?;
        cliclack::log::info(format!("📝 Description: {}", task.description))?;
        cliclack::log::info(format!("📚 Content: {}", task.content))?;
        cliclack::log::info(format!("📍 Status: {}", task.status))?;
        if let Some(due_date) = task.due_date {
            cliclack::log::info(format!("⏰ Due Date: {}", due_date))?;
        }
        cliclack::log::info(format!("🕒 Created At: {}", task.created_at))?;

        Ok(())
    }

    pub fn list_tasks_by_status() -> Result<(), Box<dyn std::error::Error>> {
        let task_dir: String = read_pointers(Pointers::Task)?;
        let tasks: Vec<Task> = Task::load_tasks(&task_dir)?;
        let mut items: Vec<(String, String, String)> = vec![];

        let status: String = cliclack::select("Select task status")
            .item("todo".to_string(), "To Do", "")
            .item("in_progress".to_string(), "In Progress", "")
            .item("done".to_string(), "Done", "")
            .interact()?;

        for task in tasks {
            if task.status == status {
                let id_str: String = task.id.to_string();
                let label: String = format!("{}: {}", task.id, task.title);
                let hint: String = format!("Status: {}", task.status);

                items.push((id_str, label, hint));
            }
        }

        if items.is_empty() {
            cliclack::log::info(format!("📭 No tasks found with status '{}'", status))?;
            return Ok(());
        }

        items.iter().for_each(|(_, label, hint)| {
            let _ = cliclack::log::info(format!("📌 {} ({})", label, hint));
        });

        Ok(())
    }

    pub fn list_tasks() -> Result<(), Box<dyn std::error::Error>> {
        let task_dir: String = read_pointers(Pointers::Task)?;
        let mut items: Vec<(String, String, String)> = vec![];
        let tasks: Vec<Task> = Task::load_tasks(&task_dir)?;

        for task in tasks {
            let id_str: String = task.id.to_string();
            let label: String = format!("{}: {}", task.id, task.title);
            let hint: String = format!("Status: {}", task.status);

            items.push((id_str, label, hint));
        }

        if items.is_empty() {
            cliclack::log::info("📭 No tasks found.")?;
        } else {
            items.iter().for_each(|(_, label, hint)| {
                let _ = cliclack::log::info(format!("📌 {} ({})", label, hint));
            })
        }

        Ok(())
    }

    pub fn delete_done_tasks() -> Result<(), Box<dyn std::error::Error>> {
        let task_dir: String = read_pointers(Pointers::Task)?;
        let tasks: Vec<Task> = Task::load_tasks(&task_dir)?;
        let mut items: Vec<(String, String, String)> = vec![];

        for task in tasks {
            if task.status == "done" {
                let id_str: String = task.id.to_string();
                let label: String = format!("{}: {}", task.id, task.title);
                let hint: String = format!("Status: {}", task.status);

                items.push((id_str, label, hint));
            }
        }

        if items.is_empty() {
            cliclack::log::info("📭 No done tasks found.")?;
            return Ok(());
        }

        let confirm: bool = cliclack::confirm("⚠️ Are you sure you want to delete all done tasks?")
            .initial_value(false)
            .interact()?;

        if confirm {
            for (id_str, _, _) in items {
                let task_id: u32 = id_str.parse::<u32>()?;
                let file_path: PathBuf =
                    std::path::Path::new(&task_dir).join(format!("{}.json", task_id));
                if file_path.exists() {
                    fs::remove_file(&file_path)?;
                    cliclack::log::info(format!("✅ Task {} deleted successfully", task_id))?;
                } else {
                    cliclack::log::error(format!("❌ Task {} not found", task_id))?;
                }
            }
        } else {
            cliclack::log::info("❌ Task deletion cancelled")?;
        }

        Ok(())
    }

    pub fn delete_task() -> Result<(), Box<dyn std::error::Error>> {
        let task_dir: String = read_pointers(Pointers::Task)?;
        let mut items: Vec<(String, String, String)> = vec![];
        let tasks: Vec<Task> = Task::load_tasks(&task_dir)?;

        for task in tasks {
            let id_str: String = task.id.to_string();
            let label: String = format!("{}: {}", task.id, task.title);
            let hint: String = format!("Status: {}", task.status);

            items.push((id_str, label, hint));
        }

        if items.is_empty() {
            cliclack::log::info("📭 No tasks found.")?;
            return Ok(());
        }

        let selected = cliclack::select("Select a task to delete")
            .items(&items)
            .item("return".to_string(), "Return", "")
            .interact()?;

        if selected == "return" {
            return Ok(());
        }

        let task_id: u32 = selected.parse::<u32>()?;
        let file_path: PathBuf = std::path::Path::new(&task_dir).join(format!("{}.json", task_id));
        if file_path.exists() {
            if cliclack::confirm(format!(
                "⚠️ Are you sure you want to delete task {}?",
                task_id
            ))
            .initial_value(false)
            .interact()?
            {
                fs::remove_file(&file_path)?;
                cliclack::log::info(format!("✅ Task {} deleted successfully", task_id))?;
            } else {
                cliclack::log::info("❌ Task deletion cancelled")?;
            }
        } else {
            cliclack::log::error(format!("❌ Task {} not found", task_id))?;
        }

        Ok(())
    }
}
