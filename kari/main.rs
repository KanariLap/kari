pub mod commands;
pub mod context;
pub mod logger;
pub mod updater;

use crate::{commands::*, context::*};
use kari_errors::Result;
use kari_span::symbol::create_session_if_not_set_then;

use clap::Parser;
use std::{path::PathBuf, process::exit};

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(Parser, Debug)]
#[clap(name = "kari", author = "t800", version)]
pub struct CLI {
    #[clap(short, global = true, help = "Print additional information for debugging")]
    debug: bool,

    #[clap(short, global = true, help = "Suppress CLI output")]
    quiet: bool,

    #[clap(subcommand)]
    command: Commands,

    #[clap(help = "Custom Kari PM backend URL", env = "APM_URL")]
    api: Option<String>,

    #[clap(long, global = true, help = "Optional path to Kari program root folder")]
    path: Option<PathBuf>,
}

///Kari compiler and package manager
#[derive(Parser, Debug)]
enum Commands {
    // #[clap(about = "Create a new Kari package in an existing directory")]
    // Init {
    //     #[clap(flatten)]
    //     command: Init,
    // },
    //
    #[clap(about = "Create a new Kari package in a new directory")]
    New {
        #[clap(flatten)]
        command: New,
    },
    #[clap(about = "Compile the current package as a program")]
    Build {
        #[clap(flatten)]
        command: Build,
    },
    #[clap(about = "Clean the output directory")]
    Clean {
        #[clap(flatten)]
        command: Clean,
    },
    #[clap(about = "Run a program with input variables")]
    Run {
        #[clap(flatten)]
        command: Run,
    },
    // #[clap(subcommand)]
    // Node(Node),
    // #[clap(about = "Deploy a program")]
    // Deploy {
    //     #[clap(flatten)]
    //     command: Deploy,
    // },
}

fn set_panic_hook() {
    #[cfg(not(debug_assertions))]
    std::panic::set_hook({
        Box::new(move |e| {
            eprintln!("thread `{}` {}", std::thread::current().name().unwrap_or("<unnamed>"), e);
            eprintln!("stack backtrace: \n{:?}", backtrace::Backtrace::new());
            eprintln!("error: internal compiler error: unexpected panic\n");
            eprintln!("note: the compiler unexpectedly panicked. this is a bug.\n");
            eprintln!(
                "note: we would appreciate a bug report: https://github.com/AleoHQ/leo/issues/new?labels=bug,panic&template=bug.md&title=[Bug]\n"
            );
            eprintln!(
                "note: {} {} running on {} {}\n",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                sys_info::os_type().unwrap_or_else(|e| e.to_string()),
                sys_info::os_release().unwrap_or_else(|e| e.to_string()),
            );
            eprintln!("note: compiler args: {}\n", std::env::args().collect::<Vec<_>>().join(" "));
            eprintln!("note: compiler flags: {:?}\n", CLI::parse());
        })
    });
}

pub fn handle_error<T>(res: Result<T>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{err}");
            exit(err.exit_code());
        }
    }
}

/// Run command with custom build arguments.
pub fn run_with_args(cli: CLI) -> Result<()> {
    if !cli.quiet {
        // Init logger with optional debug flag.
        logger::init_logger("kari", match cli.debug {
            false => 1,
            true => 2,
        })?;
    }

    // Get custom root folder and create context for it.
    // If not specified, default context will be created in cwd.
    let context = handle_error(Context::new(cli.path));

    match cli.command {
        Commands::New { command } => command.try_execute(context),
        Commands::Build { command } => command.try_execute(context),
        Commands::Clean { command } => command.try_execute(context),
        Commands::Run { command } => command.try_execute(context),
        // Commands::Node(command) => command.try_execute(context),
        // Commands::Deploy { command } => command.try_execute(context),
    }
}

fn main() {
    set_panic_hook();
    create_session_if_not_set_then(|_| handle_error(run_with_args(CLI::parse())));
}