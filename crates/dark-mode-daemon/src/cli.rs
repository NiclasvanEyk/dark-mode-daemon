use std::sync::mpsc::{channel, Receiver, Sender};

use clap::{Parser, Subcommand};

use crate::{execution::run_scripts, platform_specifics::NativeAdapter, ColorMode};

#[derive(Parser)]
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

    /// Manually run scripts for testing.
    Run {
        /// The mode that the scripts should be run for.
        mode: ColorMode,

        #[arg(short, long)]
        verbose: bool,
    },

    /// Prints the scripts that would be run.
    List,
}

pub fn run<Adapter>(native_adapter: Adapter)
where
    Adapter: NativeAdapter,
{
    let cli = Cli::parse();

    match cli.command {
        Command::Daemon { verbose } => {
            let (sender, receiver): (Sender<ColorMode>, Receiver<ColorMode>) = channel();
            println!("😈 Spawning daemon...");
            native_adapter.setup_mode_change_listener(sender);

            println!("😈 Listening for color mode changes...");
            loop {
                // FIXME: Actually handle errors here
                let new_mode = receiver.recv().unwrap();
                run_scripts(new_mode, verbose, true);
            }
        }
        Command::Run { mode, verbose } => run_scripts(mode, verbose, true),
        Command::List => todo!(),
    }
}
