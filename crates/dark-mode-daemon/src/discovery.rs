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

pub(crate) enum ScriptsDirectoryEntryKind {
    Script,
    Directory,
    NonExecutableFile,
}

pub(crate) struct ScriptsDirectoryEntry {
    /// What type of entry this is.
    pub kind: ScriptsDirectoryEntryKind,

    /// The pathbuf pointing to the file descriptor in the scripts directory.
    pub script: PathBuf,

    /// The actual script location, wich may differ from
    /// [`ScriptsDirectoryEntry::script`], when dealing with symlinks.
    pub target: PathBuf,
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

        let original_path_buf = dir_entry.path();
        let path_buf = match canonicalize(original_path_buf.clone()) {
            Ok(path) => path,
            Err(error) => {
                return Some(Err(ScriptsDirectoryEntryError::FailedToResolveSymlink {
                    source: original_path_buf,
                    error,
                }));
            }
        };

        let metadata = match path_buf.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                return Some(Err(ScriptsDirectoryEntryError::FailedToReadMetadata {
                    source: original_path_buf,
                    error,
                }));
            }
        };

        if !metadata.is_file() {
            return Some(Ok(ScriptsDirectoryEntry {
                kind: ScriptsDirectoryEntryKind::Directory,
                script: original_path_buf,
                target: path_buf,
            }));
        }

        let is_executable = metadata.permissions().mode() & 0o111 != 0;
        if !is_executable {
            return Some(Ok(ScriptsDirectoryEntry {
                kind: ScriptsDirectoryEntryKind::NonExecutableFile,
                script: original_path_buf,
                target: path_buf,
            }));
        }

        Some(Ok(ScriptsDirectoryEntry {
            kind: ScriptsDirectoryEntryKind::Script,
            script: original_path_buf,
            target: path_buf,
        }))
    }
}
