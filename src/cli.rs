use std::process::exit;

use clap::{Parser, Subcommand};

use crate::{
    discovery::{ScriptsDirectory, ScriptsDirectoryEntryKind},
    execution::run_scripts,
    platform::NativeAdapter,
    ColorMode,
};

#[derive(Parser)]
#[command(name = "dark-mode-daemon")]
#[command(bin_name = "dark-mode-daemon")]
#[command(about = "😈 Run scripts when the system color scheme changes between light and dark. 🦇")]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Starts the daemon.
    Daemon {
        #[arg(short, long)]
        verbose: bool,
    },

    /// Prints the current color mode.
    Current,

    /// Manually run scripts for testing.
    Run {
        /// The mode that the scripts should be run for.
        mode: ColorMode,

        #[arg(short, long)]
        verbose: bool,
    },

    /// Prints the scripts that would be run.
    List {
        /// Print resolved target locations for symlinked scripts.
        #[arg(long)]
        resolve: bool,

        #[arg(short, long)]
        verbose: bool,
    },
}

pub fn run<Adapter>(native_adapter: Adapter)
where
    Adapter: NativeAdapter,
{
    let cli = Cli::parse();

    match cli.command {
        Command::Daemon { verbose } => {
            println!("😈 Running scripts initially for current color mode...");
            // FIXME: Actually handle errors here
            let mode = native_adapter.current_mode().unwrap();
            run_scripts(mode, verbose, true);

            println!("😈 Spawning daemon...");
            native_adapter.run_daemon(verbose);
        }
        Command::Current => {
            // FIXME: error handling
            println!("{}", native_adapter.current_mode().unwrap());
        }
        Command::Run { mode, verbose } => run_scripts(mode, verbose, true),
        Command::List { resolve, verbose } => {
            let scripts_directory = match ScriptsDirectory::read() {
                Ok(directory) => directory,
                Err(error) => {
                    // TODO: we can probably exit 0 if the directory is just missing.
                    println!("ScriptsDirectoryError: {error:?}");
                    exit(-1);
                }
            };

            for iteration_result in scripts_directory {
                let entry = match iteration_result {
                    Ok(entry) => entry,
                    Err(error) => {
                        println!("{error:?}");
                        exit(-1);
                    }
                };

                let mut path = entry.script;
                if resolve {
                    path = entry.target;
                }

                match entry.kind {
                    ScriptsDirectoryEntryKind::Directory => {
                        if verbose {
                            println!("{} (skipped, directory)", path.to_string_lossy());
                        }
                    }
                    ScriptsDirectoryEntryKind::NonExecutableFile => {
                        if verbose {
                            println!("{} (skipped, non-executable)", path.to_string_lossy());
                        }
                    }
                    ScriptsDirectoryEntryKind::Script => {
                        println!("{}", path.to_string_lossy());
                    }
                }
            }
        }
    }
}
