use crate::tasks;

pub fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let add_menu = cliclack::select("Add Task Menu")
        .item("add_task", "Add Task", "")
        .item("add_task_with_due_date", "Add Task with Due Date", "")
        .item("return", "Return", "Return to main menu")
        .interact()?;

    match add_menu {
        "add_task" => {
            let title = cliclack::input("Enter task title").interact()?;
            let description = cliclack::input("Enter task description").interact()?;
            let content = cliclack::input("Enter task content").interact()?;
            let status = cliclack::input("Enter task status").interact()?;

            tasks::Task::create(title, description, content, status, None)?;
        }
        "add_task_with_due_date" => {}
        "return" => {}
        _ => unreachable!("Unexpected menu option"),
    }

    Ok(())
}
