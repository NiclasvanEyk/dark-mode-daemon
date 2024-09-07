use std::fmt::Display;

/// Contains the command line interface.
pub mod cli;

/// How and where to find the scripts to execute.
pub mod discovery;

/// How the scripts are run.
pub mod execution;

/// The platform-specific parts.
pub mod platform;

use clap::ValueEnum;

#[cfg(target_os = "macos")]
use crate::platform::macos::MacOSNativeAdapter;

#[cfg(target_os = "linux")]
use crate::platform::linux::gsettings::{
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
            crate::cli::run(GSettingsAdapter::<GnomeDesktopSettingsProvider>::new());
        }
        SettingsProviderImplementation::Freedesktop => {
            crate::cli::run(GSettingsAdapter::<FreeDesktopSettingsProvider>::new());
        }
    };
}
