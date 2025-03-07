use crate::commands::NuggitCommand;
use crate::commit::Commit;
use anyhow::Result;

pub struct LogCommand{}


impl LogCommand{
    pub fn new() -> Self{
        LogCommand{}
    }
}

impl NuggitCommand for LogCommand{
    fn execute(&mut self) -> Result<()> {
        Commit::list_all_commits()?;
        Ok(())
    }
}
