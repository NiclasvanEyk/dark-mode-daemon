use std::process::{exit, Command, Stdio};

use crate::{discovery::ScriptsDirectoryEntryKind, ColorMode};

pub fn run_scripts(mode: ColorMode, verbose: bool, pipe_stdio: bool) {
    let scripts_directory = match crate::discovery::ScriptsDirectory::read() {
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
                if verbose {
                    println!("{error:?}");
                }
                continue;
            }
        };

        match entry.kind {
            ScriptsDirectoryEntryKind::Directory => {
                if verbose {
                    println!(
                        "Skipping directory at '{}'...",
                        entry.target.to_string_lossy()
                    );
                }
                continue;
            }
            ScriptsDirectoryEntryKind::NonExecutableFile => {
                if verbose {
                    println!(
                        "Skipping non-executable file at '{}'...",
                        entry.target.to_string_lossy()
                    );
                }
                continue;
            }
            ScriptsDirectoryEntryKind::Script => {
                if verbose {
                    println!("🚀 Executing '{}'...", entry.target.to_string_lossy());
                }

                let path = entry.target;
                let mut command = Command::new(&path);
                command.env("DMD_COLOR_MODE", mode.to_string());

                if pipe_stdio {
                    command
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit());
                }

                let mut child = match command.spawn() {
                    Ok(child) => child,
                    Err(error) => {
                        println!("❌ Failed to launch '{}': {error}", path.to_string_lossy());
                        continue;
                    }
                };

                let status = match child.wait() {
                    Ok(status) => status,
                    Err(error) => {
                        println!("❌ Script '{}' failed: {error}", path.to_string_lossy());
                        continue;
                    }
                };

                if !status.success() {
                    println!("❌ Script '{}' failed!", path.to_string_lossy());
                }
            }
        }
    }
}