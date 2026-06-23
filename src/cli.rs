use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "coldtrace")]
#[command(about = "Run a command and record cold-start facts")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Run(RunArgs),
}

#[derive(Debug, Args)]
pub struct RunArgs {
    #[arg(long)]
    pub name: Option<String>,

    #[arg(required = true, last = true)]
    pub command: Vec<String>,
}
