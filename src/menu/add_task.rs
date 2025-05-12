pub fn handle_add_task_menu() -> Result<(), Box<dyn std::error::Error>> {
    let add_menu = cliclack::select("Add Task Menu")
        .item("add_task", "Add Task", "")
        .item("add_task_with_due_date", "Add Task with Due Date", "")
        .item("return", "Return", "Return to main menu")
        .interact()?;

    match add_menu {
        "add_task" => {
            let title: String = cliclack::input("Enter task title")
                .placeholder("Task Title")
                .interact()?;
            let description: String = cliclack::input("Enter task description")
                .placeholder("Task Description")
                .interact()?;
            let content: String = cliclack::input("Enter task content")
                .placeholder("Task Content")
                .interact()?;
        }
        "add_task_with_due_date" => {
            let title: String = cliclack::input("Enter task title")
                .placeholder("Task Title")
                .interact()?;
            let description: String = cliclack::input("Enter task description")
                .placeholder("Task Description")
                .interact()?;
            let content: String = cliclack::input("Enter task content")
                .placeholder("Task Content")
                .interact()?;
            let due_date: String = cliclack::input("Enter due date (YYYY-MM-DD)")
                .placeholder("Due Date")
                .interact()?;
        }
        "return" => {}
        _ => unreachable!("Unexpected menu option"),
    }

    Ok(())
}
