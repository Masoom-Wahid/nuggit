use clap::{Parser,Subcommand};


#[derive(Parser)]
pub struct InitArgs {
    #[arg(name="path", help="The path to the project")]
    pub path: Option<String>,
}


#[derive(Parser)]
pub struct AddArgs {
    #[arg(help="The path to of the file/files to add")]
    pub path: String
}


#[derive(Parser)]
pub struct CommitArgs {
    #[arg(short, long, help="The commit message")]
    pub message: String,
}


#[derive(Parser)]
pub struct StatusArgs {
    #[arg(help="The path to the index file")]
    pub path: Option<String>,
}


#[derive(Parser)]
pub struct DiffArgs {
    #[arg(help="The first hash")]
    pub first_hash: Option<String>,
    #[arg(help="The second hash")]
    pub second_hash: Option<String>,
}


#[derive(Subcommand)]
pub enum Command {
    #[command(name="init", about="Initialize a new project")]
    Init(InitArgs),
    #[command(name="add", about="Add a new file to the commit")]
    Add(AddArgs),
    #[command(name="commit", about="Commit the project")]
    Commit(CommitArgs),
    #[command(name="status", about="List indexes")]
    Status(StatusArgs),
    #[command(name="log", about="List all commits")]
    Log,
    #[command(name="diff", about="List all changes")]
    Diff(DiffArgs),
}


#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short, long, help="The verbosity level", default_value_t = 2)]
    pub verbose: u64,
}
