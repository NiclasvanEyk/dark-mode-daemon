/// Contains the command line interface.
pub mod cli;

/// How and where to find the scripts to execute.
pub mod discovery;

/// How the scripts are run.
pub mod execution;

/// The color mode struct.
pub mod mode;

/// The platform-specific parts.
pub mod platform;

#[cfg(target_os = "macos")]
use crate::platform::macos::MacOSNativeAdapter;

#[cfg(target_os = "macos")]
fn main() {
    crate::cli::run(MacOSNativeAdapter::default());
}

#[cfg(target_os = "linux")]
use crate::platform::linux::gsettings::{
    freedesktop::FreeDesktopSettingsProvider, gnome::GnomeDesktopSettingsProvider,
    GSettingsAdapter, SettingsProviderImplementation,
};

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
