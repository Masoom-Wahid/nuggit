use crate::commands::NuggitCommand;
use anyhow::Result;
use crate::index::Index;

pub struct StatusCommand{}


impl StatusCommand{
    pub fn new() -> Self{
        StatusCommand{}
    }
}


impl NuggitCommand for StatusCommand{
    fn execute(&mut self) -> Result<()> {
        Index::new(None)?.list_entries()?;
        Ok(())
    }
}