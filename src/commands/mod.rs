pub mod init;
pub mod add;
pub mod commit;
pub mod log;
pub mod status;
pub mod diff;

pub use init::InitCommand;
pub use add::AddCommand;
pub use commit::CommitCommand;
pub use log::LogCommand;
pub use status::StatusCommand;
pub use diff::DiffCommand;
use anyhow::Result;

pub trait NuggitCommand{
    fn execute(&mut self) -> Result<()>;
}



#[allow(dead_code)]
pub struct NuggitCommandResult{
    pub success: bool,
    pub message: String,
}

