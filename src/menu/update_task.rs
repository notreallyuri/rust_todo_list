use crate::tasks;

pub fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let update_menu = cliclack::select("Update Menu")
        .item(
            "update_task",
            "Update Task",
            "Update the name or content of a task",
        )
        .item(
            "update_task_status",
            "Update Task Status",
            "Update the status of a task",
        )
        .item(
            "update_task_due_date",
            "Update Task Due Date",
            "Update the due date of a task",
        )
        .item("return", "Return", "Return to main menu")
        .interact()?;

    match update_menu {
        "update_task" => tasks::Task::update_task_default()?,
        "update_task_status" => tasks::Task::update_task_status()?,
        "update_task_due_date" => tasks::Task::update_task_due_date()?,
        "return" => return Ok(()),
        _ => unreachable!("Unexpected menu option"),
    }

    Ok(())
}
