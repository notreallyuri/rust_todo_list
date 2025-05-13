use crate::menu::{add_task, config, delete_task, get_task, update_task};

pub fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let main_menu = cliclack::select("Main Menu")
            .item("add_task", "Add Task", "Create a new task")
            .item("edit_task", "Update Task", "Edit an existing task")
            .item("get_task", "Get Task", "")
            .item("delete_task", "Delete Task", "")
            .item("config", "Configuration", "Configure the application")
            .item("exit", "Exit", "Exit the application")
            .interact()?;

        match main_menu {
            "add_task" => add_task::handler()?,
            "edit_task" => update_task::handler()?,
            "get_task" => get_task::handler()?,
            "config" => config::handler()?,
            "delete_task" => delete_task::handler()?,
            "exit" => {
                cliclack::outro("Exiting...")?;
                break;
            }
            _ => unreachable!("Unexpeted menu option"),
        }
    }

    Ok(())
}
