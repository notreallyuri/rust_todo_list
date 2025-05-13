use crate::tasks;


pub fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let delete_menu = cliclack::select("Delete Menu")
        .item("delete_done", "Delete Done Tasks", "")
        .item("delete_by_id", "Delete Task", "Delete a task by ID")
        .item("return", "Return", "Return to main menu")
        .interact()?;

    match delete_menu {
        "delete_done" => tasks::Task::delete_done_tasks()?,
        "delete_by_id" => tasks::Task::delete_task()?,
        "return" => return Ok(()),
        _ => unreachable!("Unexpected menu option"),
    }

    Ok(())
}
