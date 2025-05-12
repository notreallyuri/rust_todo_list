pub fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let delete_menu = cliclack::select("Delete Menu")
        .item("delete_all", "Delete All Tasks", "")
        .item("delete_by_id", "Delete Task", "Delete a task by ID")
        .item("return", "Return", "Return to main menu")
        .interact()?;

    match delete_menu {
        "delete_all" => {
            cliclack::outro("Delete All Tasks")?;
        }
        "delete_by_id" => {
            cliclack::outro("Delete Task by ID")?;
        }
        "return" => {}
        _ => unreachable!("Unexpected menu option"),
    }

    Ok(())
}
