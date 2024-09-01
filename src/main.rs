use std::fmt::Display;

pub mod cli;
pub mod discovery;
pub mod execution;
pub mod platform;
pub mod platform_specifics;

use clap::ValueEnum;

#[cfg(target_os = "macos")]
use crate::platform::macos::MacOSNativeAdapter;

#[cfg(target_os = "linux")]
use gsettings::{
    freedesktop::FreeDesktopSettingsProvider, gnome::GnomeDesktopSettingsProvider,
    GSettingsAdapter, SettingsProviderImplementation,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ColorMode {
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

#[cfg(target_os = "macos")]
fn main() {
    crate::cli::run(MacOSNativeAdapter::default());
}

#[cfg(target_os = "linux")]
fn main() {
    let implementation = SettingsProviderImplementation::Gnome;

    match implementation {
        SettingsProviderImplementation::Gnome => {
            dark_mode_daemon::cli::run(GSettingsAdapter::<GnomeDesktopSettingsProvider>::new());
        }
        SettingsProviderImplementation::Freedesktop => {
            dark_mode_daemon::cli::run(GSettingsAdapter::<FreeDesktopSettingsProvider>::new());
        }
    };
}
