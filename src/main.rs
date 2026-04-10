#![allow(dead_code)]

use clap::{Parser, Subcommand};
use std::process;

mod cli;
mod commands;
mod config;
mod download;
mod manifest;
mod paths;
mod ui;
mod version;

use cli::{SapphireCommand, SelfCommand};

#[derive(Parser)]
#[command(
    name = "facet",
    about = "The official toolchain manager and project CLI for the Sapphire programming language",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Arguments passed through to the active Sapphire binary
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    passthrough: Vec<String>,
}

#[derive(Subcommand)]
enum Command {
    /// Manage Sapphire toolchain installations
    Sapphire {
        #[command(subcommand)]
        subcommand: SapphireCommand,
    },

    /// Create a new Sapphire project
    New {
        /// Project name
        name: String,
        /// Project template
        #[arg(long, default_value = "application")]
        template: String,
    },

    /// Run a project script or binary
    Run {
        /// Script or binary to run
        script: Option<String>,
        /// Additional arguments
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Run the project test suite
    Test {
        /// Additional arguments passed to the test runner
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Run the linter
    Lint {
        /// Additional arguments passed to the linter
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Start an interactive Sapphire console (REPL)
    Console,

    /// Print facet and active Sapphire versions
    Version,

    /// Manage facet itself
    #[command(name = "self")]
    SelfCmd {
        #[command(subcommand)]
        subcommand: SelfCommand,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Command::Sapphire { subcommand }) => commands::sapphire::run(subcommand),
        Some(Command::New { name, template }) => commands::new::run(name, template),
        Some(Command::Run { script, args }) => commands::run::run(script, args),
        Some(Command::Test { args }) => commands::test::run(args),
        Some(Command::Lint { args }) => commands::lint::run(args),
        Some(Command::Console) => commands::console::run(),
        Some(Command::Version) => {
            println!("facet {}", env!("CARGO_PKG_VERSION"));
            let paths = paths::Paths::new();
            let cwd = std::env::current_dir().unwrap_or_default();
            match version::resolve(&cwd, &paths) {
                Ok(Some(r)) => println!("sapphire {}", r.version),
                Ok(None) => println!("sapphire (none)"),
                Err(_) => println!("sapphire (unknown)"),
            }
            Ok(())
        }
        Some(Command::SelfCmd { subcommand }) => commands::self_cmd::run(subcommand),
        None => {
            if cli.passthrough.is_empty() {
                eprintln!("No command provided. Run `facet --help` for usage.");
                process::exit(1);
            }
            commands::passthrough::run(cli.passthrough)
        }
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
