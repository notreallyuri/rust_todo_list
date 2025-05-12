use crate::menu::{add_task, config};

pub fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let main_menu = cliclack::select("Main Menu")
            .item("add_task", "Add Task", "Create a new task")
            .item("edit_task", "Edit Task", "Edit an existing task")
            .item("get_task", "Get Task", "")
            .item("delete_task", "Delete Task", "")
            .item("config", "Configuration", "Configure the application")
            .item("exit", "Exit", "Exit the application")
            .interact()?;

        match main_menu {
            "add_task" => add_task::handle_add_task_menu()?,
            "edit_task" => {
                cliclack::outro("Edit Task")?;
            }
            "get_task" => {
                let get_menu = cliclack::select("Get Menu")
                    .item("get_all", "Get All Tasks", "")
                    .item("get_by_id", "Get Task", "Get a task by ID")
                    .item("get_by_status", "Get Tasks", "Get tasks by status")
                    .interact()?;
            }
            "config" => config::handle_config_menu()?,
            "delete_task" => {
                let delete_menu = cliclack::select("Delete Menu")
                    .item("delete_all", "Delete All Tasks", "")
                    .item("delete_by_id", "Delete Task", "Delete a task by ID")
                    .interact()?;
            }
            "exit" => {
                cliclack::outro("Exiting...")?;
                break;
            }
            _ => unreachable!("Unexpeted menu option"),
        }
    }

    Ok(())
}
