#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

use crate::mode::ColorMode;
use std::error::Error;

/// Adapter that encapsulates platform-specific parts of dark mode daemon.
/// TODO: This probably also needs a method for cleaning up the listener in the
/// case of an interrupt or error!
pub trait NativeAdapter {
    /// Implementation of the deamon, which differs between OSes.
    fn run_daemon(&self, verbose: bool);

    /// Query the OS for the current color mode.
    fn current_mode(&self) -> Result<ColorMode, Box<dyn Error>>;
}
