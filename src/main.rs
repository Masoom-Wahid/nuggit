mod cli;
mod commands;
mod utils;
mod config;
mod index;

use anyhow::Result;
use cli::{Cli, Command};
use clap::Parser;
use commands::NuggitCommand;
use utils::logger::setup_logger;
use index::Index;

fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logger(cli.verbose)?;
    match cli.command {
        Command::Init(args) => commands::InitCommand{path: args.path}.execute()?,
        Command::Add(args) => commands::AddCommand::new(args.path)?.execute()?,
        Command::List(_) => {
            let index = Index::new(None)?;
            index.list_entries()?;
        },
        _ => panic!("not implemented yet"),
    }
    Ok(())
}
