pub mod init;
pub mod add;
pub mod commit;

pub use init::InitCommand;
pub use add::AddCommand;
pub use commit::CommitCommand;

use anyhow::Result;

pub trait NuggitCommand{
    fn execute(&mut self) -> Result<()>;
}



#[allow(dead_code)]
pub struct NuggitCommandResult{
    pub success: bool,
    pub message: String,
}

