

pub fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let update_menu = cliclack::select("Update Menu")
        .item("update_task", "Update Task", "Update the name or content of a task")
        .item("update_task_status", "Update Task Status", "Update the status of a task")
        .item("update_task_due_date", "Update Task Due Date", "Update the due date of a task")
        .item("return", "Return", "Return to main menu")
        .interact()?;

    match update_menu {
        "update_task" => {}
        "update_task_status" => {}
        "return" => {}
        _ => unreachable!("Unexpected menu option"),
    }

    Ok(())
}