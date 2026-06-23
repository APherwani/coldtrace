use clap::Parser;
use coldtrace::cli::{Cli, Command};
use coldtrace::runner::{self, RunOptions};
use std::process;

fn main() {
    let cli = Cli::parse();

    if let Err(error) = dispatch(cli) {
        eprintln!("coldtrace: {error}");
        process::exit(2);
    }
}

fn dispatch(cli: Cli) -> runner::Result<()> {
    match cli.command {
        Command::Run(args) => {
            let run_dir = runner::run_command(RunOptions {
                name: args.name,
                command_argv: args.command,
            })?;

            println!("{}", run_dir.display());
            Ok(())
        }
    }
}
