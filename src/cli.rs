use std::{
    io::{self, IsTerminal},
    process::exit,
};

use clap::{Parser, Subcommand};

use crate::{
    discovery::{ScriptsDirectory, ScriptsDirectoryEntryKind},
    execution::run_scripts,
    mode::ColorMode,
    platform::{ColorModeDaemon, ColorModeDetector},
};

#[derive(Parser)]
#[command(name = "dark-mode-daemon")]
#[command(bin_name = "dark-mode-daemon")]
#[command(about = "😈 Run scripts when the system color scheme changes between light and dark. 🦇")]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Starts the daemon.
    Daemon {
        #[arg(short, long)]
        verbose: bool,
    },

    /// Prints the current color mode.
    Current {
        /// Whether to not only print the current color mode once, but also
        /// print it again if it changes (without running any scripts).
        #[arg(short, long)]
        watch: bool,

        /// Whether to only print the name, without any emojis.
        /// Useful for using this in a programatic way.
        #[arg(short, long)]
        plain: bool,
    },

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

pub async fn run<F, Futu, Daemon>(native_adapter: F) -> anyhow::Result<()>
where
    Futu: std::future::Future<Output = anyhow::Result<Daemon>>,
    F: FnOnce() -> Futu,
    Daemon: ColorModeDaemon + ColorModeDetector,
{
    let cli = Cli::parse();
    let command = cli
        .command
        .unwrap_or_else(|| Command::Daemon { verbose: false });

    match command {
        Command::Daemon { verbose } => {
            let adapter = native_adapter().await?;
            println!("😈 Running scripts initially for current color mode...");
            // FIXME: Actually handle errors here
            let mode = adapter.current_mode().await.unwrap();
            run_scripts(mode, verbose, true);

            println!("😈 Spawning daemon...");
            adapter
                .on_color_changed(|mode| run_scripts(mode, verbose, true))
                .await;
        }
        Command::Current { watch, plain } => {
            // FIXME: error handling
            let adapter = native_adapter().await?;
            let mode = adapter.current_mode().await.unwrap();
            if plain {
                println!("{}", mode);
            } else {
                println!("{} {}", mode.emoji(), mode);
            }
            if !watch {
                return Ok(());
            }

            adapter
                .on_color_changed(|mode| {
                    if plain {
                        println!("{}", mode);
                    } else {
                        println!("{} {}", mode.emoji(), mode);
                    }
                })
                .await;
        }
        Command::Run { mode, verbose } => run_scripts(mode, verbose, true),
        Command::List { resolve, verbose } => {
            let environment = Environment::infer();
            let scripts_directory = match ScriptsDirectory::read() {
                Ok(directory) => directory,
                Err(error) => {
                    // TODO: we can probably exit 0 if the directory is just missing.
                    println!("ScriptsDirectoryError: {error:?}");
                    exit(-1);
                }
            };

            if !environment.piped {
                println!(
                    "📂 Using scripts in {}...\n",
                    scripts_directory.path.to_string_lossy()
                );
            }

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
    };

    Ok(())
}

pub struct Environment {
    /// Whether the current command is being piped into something.
    piped: bool,
}

impl Environment {
    pub fn infer() -> Self {
        let is_terminal = io::stdin().is_terminal();
        Self {
            piped: !is_terminal,
        }
    }
}
