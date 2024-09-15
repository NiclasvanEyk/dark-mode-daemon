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
fn main() {
    crate::cli::run(crate::platform::macos::MacOsAdapter::default());
}

#[cfg(target_os = "linux")]
fn main() {
    crate::cli::run(crate::platform::dark_light::DarkLightAdapter::default());
}

#[cfg(target_os = "windows")]
fn main() {
    crate::cli::run(crate::platform::dark_light::DarkLightAdapter::default());
}
