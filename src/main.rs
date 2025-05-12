mod actions;
mod config;
mod storage;
mod todo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    actions::onboarding()?;

    Ok(())
}
