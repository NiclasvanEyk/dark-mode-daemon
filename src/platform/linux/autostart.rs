use std::{env, fmt::Display, fs, io, path::PathBuf};

use crate::cli::interface::AutostartCommand;

#[derive(Debug)]
pub enum Error {
    CouldNotDetermineConfigDir(xdg::BaseDirectoriesError),
    InvalidConfigDir(String),
    CouldNotCreateAutostartDir(io::Error),
    InvalidAutostartDir(String),
    CouldNotDetermineBinaryPath(io::Error),
    EntryAlreadyExists(PathBuf),
    FailedWritingAutostartEntry(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
                Error::CouldNotDetermineConfigDir(error) => write!(f, "Could not determine XDG config dir: {error}"),
                Error::InvalidConfigDir(message) => write!(f, "{message}"),
                Error::CouldNotCreateAutostartDir(error) => write!(f, "Could not create autostart directory: {error}"),
                Error::InvalidAutostartDir(message) => write!(f, "{message}"),
                Error::CouldNotDetermineBinaryPath(error) => write!(f, "Could not determine dark mode daemon executable path: {error}"),
                Error::EntryAlreadyExists(path_buf) => write!(f, "Autostart entry already exists at '{}'. There is a flag for forcing it to be overridden.", path_buf.display()),
                Error::FailedWritingAutostartEntry(error) => write!(f, "Failed writing autostart entry: {error}"),
            }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::CouldNotDetermineConfigDir(error) => Some(error),
            Error::InvalidConfigDir(_) => None,
            Error::CouldNotCreateAutostartDir(error) => Some(error),
            Error::InvalidAutostartDir(_) => None,
            Error::CouldNotDetermineBinaryPath(error) => Some(error),
            Error::EntryAlreadyExists(_) => None,
            Error::FailedWritingAutostartEntry(error) => Some(error),
        }
    }
}

pub fn is_setup() -> Result<bool, Error> {
    let xdg_directories = xdg::BaseDirectories::new().map_err(Error::CouldNotDetermineConfigDir)?;
    let config_home_dir = xdg_directories.get_config_home();
    let xdg_autostart_dir = config_home_dir.join("autostart");
    let autostart_entry_path = xdg_autostart_dir.join("dark-mode-daemon.desktop");
    Ok(autostart_entry_path.exists())
}

pub fn remove() -> Result<(), Error> {
    let autostart_entry_path = get_autostart_file_path(true)?;
    fs::remove_file(autostart_entry_path).map_err(Error::FailedWritingAutostartEntry)?;
    Ok(())
}

fn get_autostart_file_path(mkdirs: bool) -> Result<PathBuf, Error> {
    let xdg_directories = xdg::BaseDirectories::new().map_err(Error::CouldNotDetermineConfigDir)?;
    let config_home_dir = xdg_directories.get_config_home();

    if !config_home_dir.exists() {
        let message = format!("'{}' does not exist!", config_home_dir.display());
        return Err(Error::InvalidConfigDir(message));
    }

    if !config_home_dir.is_dir() {
        let message = format!("'{}' is not a directory!", config_home_dir.display());
        return Err(Error::InvalidConfigDir(message));
    }

    let xdg_autostart_dir = config_home_dir.join("autostart");
    if mkdirs && !xdg_autostart_dir.exists() {
        fs::create_dir(&xdg_autostart_dir).map_err(Error::CouldNotCreateAutostartDir)?;
    }

    if !xdg_autostart_dir.is_dir() {
        let message = format!("'{}' is not a directory!", config_home_dir.display());
        return Err(Error::InvalidAutostartDir(message));
    }

    return Ok(xdg_autostart_dir.join("dark-mode-daemon.desktop"));
}

pub fn install_autostart_xdg(
    explicit_dmd_binary_path: Option<PathBuf>,
    override_existing_entry: bool,
) -> Result<(), Error> {
    let autostart_entry_path = get_autostart_file_path(true)?;
    if !override_existing_entry && autostart_entry_path.exists() {
        return Err(Error::EntryAlreadyExists(autostart_entry_path));
    }

    let dmd_binary_path = match explicit_dmd_binary_path {
        Some(exe_path) => exe_path,
        None => env::current_exe().map_err(Error::CouldNotDetermineBinaryPath)?,
    };

    let contents = format!(
        r#"[Desktop Entry]
Name=Dark Mode Daemon
GenericName=Dark Mode Daemon
Comment=Runs scripts when the OS color mode changes
Exec={}
Terminal=false
Type=Application
X-GNOME-Autostart-enabled=true
"#,
        dmd_binary_path.display()
    );
    fs::write(&autostart_entry_path, contents).map_err(Error::FailedWritingAutostartEntry)?;

    println!(
        "Successfully created autostart entry at '{}'",
        autostart_entry_path.display()
    );
    println!("The next time you login, dark-mode-daemon should be started automatically.");
    println!("If you want to test it, you can log out and in again (no restart necessary). Alternatively you can run\n\n");
    println!("\tnohup dark-mode-daemon > /dev/null 2>&1 &");

    Ok(())
}

pub fn handle_autostart_command(command: AutostartCommand) -> anyhow::Result<()> {
    match command {
        AutostartCommand::Setup {
            binary_path,
            override_existing_entry,
        } => install_autostart_xdg(binary_path, override_existing_entry)?,
        AutostartCommand::Check => {
            if is_setup()? {
                println!("Autostart file exists!");
            } else {
                println!("Autostart not configured!");
            }
        }
        AutostartCommand::Remove => {
            if !is_setup()? {
                println!("Autostart not configured, nothing to remove");
                return Ok(());
            }

            remove()?;
        }
    }

    Ok(())
}
