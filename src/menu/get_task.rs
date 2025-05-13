use crate::tasks;

pub fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let get_menu = cliclack::select("Get Menu")
        .item("get_all", "Get All Tasks", "")
        .item("get_by_id", "Get Task", "Get a task by ID")
        .item("get_by_status", "Get Tasks", "Get tasks by status")
        .item("return", "Return", "Return to main menu")
        .interact()?;

    match get_menu {
        "get_all" => tasks::Task::list_tasks()?,
        "get_by_id" => tasks::Task::read_task()?,
        "get_by_status" => tasks::Task::list_tasks_by_status()?,
        "return" => {}
        _ => unreachable!("Unexpected menu option"),
    }

    Ok(())
}
