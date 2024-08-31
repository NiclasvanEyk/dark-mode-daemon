use xdg::BaseDirectories;

use std::{
    fs::{canonicalize, read_dir, ReadDir},
    iter::Iterator,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

// FIXME: The errors should properly implement the Error trait so we can properly
// communicate to the user when something goes wrong.

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum ScriptsDirectoryError {
    Missing,
    Unreadable(PathBuf),
}

pub(crate) enum ScriptsDirectoryEntry {
    Script(PathBuf),
    Directory(PathBuf),
    NonExecutableFile(PathBuf),
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum ScriptsDirectoryEntryError {
    CouldNotStartIterating,
    FailedToReadMetadata {
        source: PathBuf,
        error: std::io::Error,
    },
    FailedToResolveSymlink {
        source: PathBuf,
        error: std::io::Error,
    },
}

pub(crate) struct ScriptsDirectory {
    dir: ReadDir,
}

impl ScriptsDirectory {
    pub fn read() -> Result<Self, ScriptsDirectoryError> {
        let dmd_directory = BaseDirectories::with_prefix("dark-mode-daemon").unwrap();
        let Some(scripts_directory) = dmd_directory.find_config_file("scripts") else {
            return Err(ScriptsDirectoryError::Missing);
        };

        let Ok(scripts_iterator) = read_dir(&scripts_directory) else {
            return Err(ScriptsDirectoryError::Unreadable(scripts_directory));
        };

        Ok(Self {
            dir: scripts_iterator,
        })
    }
}

impl Iterator for ScriptsDirectory {
    type Item = Result<ScriptsDirectoryEntry, ScriptsDirectoryEntryError>;

    fn next(&mut self) -> Option<Self::Item> {
        let iteration_result = self.dir.next()?;

        let dir_entry = match iteration_result {
            Ok(dir_entry) => dir_entry,
            Err(_) => {
                return Some(Err(ScriptsDirectoryEntryError::CouldNotStartIterating));
            }
        };

        let mut path_buf = dir_entry.path();
        let mut metadata = match dir_entry.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                return Some(Err(ScriptsDirectoryEntryError::FailedToReadMetadata {
                    source: path_buf,
                    error,
                }));
            }
        };

        // There is some duplication here that would be nice to unify, but we
        // basically repeat the above logic again and again if the file is
        // symlinked.
        loop {
            if !metadata.is_symlink() {
                break;
            }

            match canonicalize(&path_buf) {
                Ok(new_path_buf) => {
                    path_buf = new_path_buf;
                }
                Err(error) => {
                    return Some(Err(ScriptsDirectoryEntryError::FailedToResolveSymlink {
                        source: path_buf,
                        error,
                    }))
                }
            };

            match path_buf.metadata() {
                Ok(new_metadata) => {
                    metadata = new_metadata;
                }
                Err(error) => {
                    return Some(Err(ScriptsDirectoryEntryError::FailedToReadMetadata {
                        source: path_buf,
                        error,
                    }))
                }
            };
        }

        if !metadata.is_file() {
            return Some(Ok(ScriptsDirectoryEntry::Directory(path_buf)));
        }

        let is_executable = metadata.permissions().mode() & 0o111 != 0;
        if !is_executable {
            return Some(Ok(ScriptsDirectoryEntry::NonExecutableFile(path_buf)));
        }

        Some(Ok(ScriptsDirectoryEntry::Script(path_buf)))
    }
}
