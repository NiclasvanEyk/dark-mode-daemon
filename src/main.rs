use std::{
    fmt::Display,
    fs::{canonicalize, read_dir},
    os::unix::fs::PermissionsExt,
    process::{exit, Command, Stdio},
};

use xdg::BaseDirectories;

enum ColorMode {
    Light,
    Dark,
}

impl Display for ColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMode::Light => write!(f, "light"),
            ColorMode::Dark => write!(f, "dark"),
        }
    }
}

fn main() {
    run_scripts(ColorMode::Dark, true, true);
}

fn run_scripts(mode: ColorMode, verbose: bool, pipe_stdio: bool) {
    let dmd_directory = BaseDirectories::with_prefix("dark-mode-daemon").unwrap();
    let Some(scripts_directory) = dmd_directory.find_config_file("scripts") else {
        println!("Could not find scripts directory!");
        exit(-1);
    };

    let Ok(scripts_iterator) = read_dir(&scripts_directory) else {
        println!("Failed to read scripts directory at '{scripts_directory:?}'!");
        exit(-1);
    };

    'scrips: for iteration_result in scripts_iterator {
        let Ok(dir_entry) = iteration_result else {
            continue;
        };
        let mut path_buf = dir_entry.path();
        let Ok(mut metadata) = dir_entry.metadata() else {
            if verbose {
                let path = path_buf.to_string_lossy();
                println!("Failed to read metadata for script at '{path}'!");
            }
            continue;
        };

        loop {
            if !metadata.is_symlink() {
                break;
            }

            match canonicalize(&path_buf) {
                Ok(new_path_buf) => {
                    path_buf = new_path_buf;
                }
                Err(_) => {
                    if verbose {
                        let path = path_buf.to_string_lossy();
                        println!("Failed to resolve link target of '{path}'!");
                    }
                    continue 'scrips;
                }
            };

            match path_buf.metadata() {
                Ok(new_metadata) => {
                    metadata = new_metadata;
                }
                Err(_) => {
                    if verbose {
                        let path = path_buf.to_string_lossy();
                        println!("Failed to read metadata for script at '{path}'!");
                    }
                    continue 'scrips;
                }
            };
        }

        let path = path_buf.to_string_lossy();
        if !metadata.is_file() {
            if verbose {
                println!("Skipping directory at '{path}'...");
            }
            continue;
        }

        let is_executable = metadata.permissions().mode() & 0o111 != 0;
        if !is_executable {
            if verbose {
                println!("Skipping non-executable file at '{path}'...");
            }
            continue;
        }

        if verbose {
            println!("🚀 Executing '{path}'...");
        }

        let mut command = Command::new(&path_buf);
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
                println!("❌ Failed to launch '{path}': {error}");
                continue;
            }
        };

        let status = match child.wait() {
            Ok(status) => status,
            Err(error) => {
                println!("❌ Script '{path}' failed: {error}");
                continue;
            }
        };

        if !status.success() {
            println!("❌ Script '{path}' failed!");
        }
    }
}
