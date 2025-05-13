use crate::tasks;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

pub fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let add_menu = cliclack::select("Add Task Menu")
        .item("add_task", "Add Task", "")
        .item("add_task_with_due_date", "Add Task with Due Date", "")
        .item("return", "Return", "Return to main menu")
        .interact()?;

    match add_menu {
        "add_task" => {
            let title: String = cliclack::input("Enter task title").interact()?;
            let description: String = cliclack::input("Enter task description").interact()?;
            let content: String = cliclack::input("Enter task content").interact()?;
            let status: String = cliclack::select("Select task status")
                .item("todo".to_string(), "To Do", "")
                .item("in_progress".to_string(), "In Progress", "")
                .item("done".to_string(), "Done", "")
                .interact()?;

            tasks::Task::create(title, description, content, status, None)?;
        }
        "add_task_with_due_date" => {
            let title: String = cliclack::input("Enter task title").interact()?;
            let description: String = cliclack::input("Enter task description").interact()?;
            let content: String = cliclack::input("Enter task content").interact()?;
            let status: String = cliclack::select("Select task status")
                .item("todo".to_string(), "To Do", "")
                .item("in_progress".to_string(), "In Progress", "")
                .item("done".to_string(), "Done", "")
                .interact()?;
            let due_date: String =
                cliclack::input("Enter task due date (YYYY-MM-DD)").interact()?;

            let due_date_parsed = if due_date.trim().is_empty() {
                None
            } else {
                let naive_date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d")?;
                let naive_datetime = NaiveDateTime::new(
                    naive_date,
                    chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                );
                let datetime_utc = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
                Some(datetime_utc)
            };

            tasks::Task::create(title, description, content, status, due_date_parsed)?;
        }
        "return" => {}
        _ => unreachable!("Unexpected menu option"),
    }

    Ok(())
}
