use std::process::exit;

use crate::cli::environment::Environment;
use crate::discovery::{ScriptsDirectory, ScriptsDirectoryEntryKind};

pub fn list(resolve: bool, verbose: bool) -> anyhow::Result<()> {
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

    Ok(())
}
