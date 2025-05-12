pub fn handler() -> Result<(), Box<dyn std::error::Error>> {
    let get_menu = cliclack::select("Get Menu")
        .item("get_all", "Get All Tasks", "")
        .item("get_by_id", "Get Task", "Get a task by ID")
        .item("get_by_status", "Get Tasks", "Get tasks by status")
        .interact()?;

    match get_menu {
        "get_all" => {
            cliclack::outro("Get All Tasks")?;
        }
        "get_by_id" => {}
        "get_by_status" => {}
        _ => unreachable!("Unexpected menu option"),
    }

    Ok(())
}
