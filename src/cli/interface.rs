use clap::{Parser, Subcommand};

use crate::mode::ColorMode;

/// 😈 Run scripts when the system color scheme changes between light and dark. 🦇
#[derive(Parser)]
#[command(name = "dark-mode-daemon", bin_name = "dark-mode-daemon", version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
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

    Autostart {
        #[command(subcommand)]
        command: AutostartCommand,
    },
}

/// Utilities for easily running the daemon on boot.
#[derive(Subcommand, Debug)]
pub enum AutostartCommand {
    /// Create the startup file
    Setup {
        #[arg(short, long)]
        binary_path: Option<std::path::PathBuf>,

        #[arg(short, long)]
        override_existing_entry: bool,
    },
    /// Check if the startup file exists
    Check,
    /// Remove the startup file
    Remove,
}
