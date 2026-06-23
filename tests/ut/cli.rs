use clap::Parser;
use coldtrace::cli::{Cli, Command};

#[test]
fn run_command_preserves_child_arguments_after_separator() {
    let cli = Cli::try_parse_from([
        "coldtrace",
        "run",
        "--name",
        "torch-import",
        "--",
        "python3",
        "-c",
        "import torch",
    ])
    .unwrap();

    let Command::Run(args) = cli.command;

    assert_eq!(args.name.as_deref(), Some("torch-import"));
    assert_eq!(args.command, ["python3", "-c", "import torch"]);
}

#[test]
fn run_command_requires_a_child_command() {
    let error = Cli::try_parse_from(["coldtrace", "run"]).unwrap_err();

    assert_eq!(
        error.kind(),
        clap::error::ErrorKind::MissingRequiredArgument
    );
}
