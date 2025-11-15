use ashpd::desktop::settings::{ColorScheme as GnomeColorMode, Settings as GnomeSettings};
use futures::StreamExt;

use crate::{
    mode::ColorMode,
    platform::{ColorModeDaemon, ColorModeDetector},
};

#[derive(Default)]
pub struct LinuxAdapter {}

impl From<GnomeColorMode> for ColorMode {
    fn from(value: GnomeColorMode) -> Self {
        match value {
            GnomeColorMode::NoPreference => ColorMode::Light,
            GnomeColorMode::PreferDark => ColorMode::Dark,
            GnomeColorMode::PreferLight => ColorMode::Light,
        }
    }
}

pub struct LinuxColorModeDetector<'a> {
    settings: GnomeSettings<'a>,
}

impl<'a> LinuxColorModeDetector<'a> {
    pub async fn default() -> anyhow::Result<Self> {
        let settings = GnomeSettings::new().await?;

        Ok(Self { settings })
    }
}

impl<'a> ColorModeDetector for LinuxColorModeDetector<'a> {
    async fn current_mode(&self) -> anyhow::Result<ColorMode> {
        let color_scheme = self.settings.color_scheme().await?;
        Ok(ColorMode::from(color_scheme))
    }
}

impl<'a> ColorModeDaemon for LinuxColorModeDetector<'a> {
    async fn on_color_changed<F>(&self, callback: F)
    where
        F: Fn(ColorMode),
    {
        let mut color_mode_changes = self
            .settings
            .receive_color_scheme_changed()
            .await
            .unwrap()
            .map(|color_scheme| ColorMode::from(color_scheme));

        // Tests showed, that for some reason the mode change is triggered twice
        // right after one another. To prevent from running our scripts twice, we
        // do some deduplication here.
        let mut previous_mode: Option<ColorMode> = None;
        while let Some(mode) = color_mode_changes.next().await {
            if mode != previous_mode.unwrap_or_else(|| mode.other()) {
                previous_mode = Some(mode);
                callback(mode);
            }
        }
    }
}

pub mod autostart {
    use std::{env, fmt::Display, fs, io, path::PathBuf};

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
        let xdg_directories =
            xdg::BaseDirectories::new().map_err(Error::CouldNotDetermineConfigDir)?;
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
        let xdg_directories =
            xdg::BaseDirectories::new().map_err(Error::CouldNotDetermineConfigDir)?;
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

        Ok(())
    }
}
