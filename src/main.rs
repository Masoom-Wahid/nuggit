mod cli;
mod commands;
mod utils;
mod config;
mod index;
mod commit;

use anyhow::Result;
use cli::{Cli, Command};
use clap::Parser;
use commands::NuggitCommand;
use utils::logger::setup_logger;
use index::Index;
use commit::Commit;
fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logger(cli.verbose)?;
    match cli.command {
        Command::Init(args) => commands::InitCommand::new(args.path).execute()?,
        Command::Add(args) => commands::AddCommand::new(args.path)?.execute()?,
        Command::Commit(args) => commands::CommitCommand::new(args.message)?.execute()?,
        Command::Status(_) => Index::new(None)?.list_entries()?,
        Command::Log => Commit::list_all_commits()?,
        #[allow(unreachable_patterns)]
        _ => panic!("not implemented yet"),
    }
    Ok(())
}
