/// Command implementations.
pub mod commands;

/// Command definitions.
pub mod interface;

/// Utility for checking if we are piped into something.
pub mod environment;

use clap::Parser;

use crate::{
    cli::interface::{Cli, Command},
    platform::{ColorModeDaemon, ColorModeDetector},
};

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
        Command::Daemon { verbose } => commands::daemon(native_adapter, verbose).await?,
        Command::Current { watch, plain } => {
            commands::current(native_adapter, watch, plain).await?
        }
        Command::Run { mode, verbose } => commands::run(mode, verbose, true),
        Command::Autostart { command } => {
            #[cfg(target_os = "linux")]
            crate::platform::linux::autostart::handle_autostart_command(command)?;

            #[cfg(target_os = "macos")]
            crate::platform::macos::autostart::handle_autostart_command(command)?;
        }
        Command::List { resolve, verbose } => commands::list(resolve, verbose)?,
    };

    Ok(())
}
