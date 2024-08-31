use std::sync::mpsc::Sender;

use crate::ColorMode;

/// Adapter that encapsulates platform-specific parts of dark mode daemon.
/// TODO: This probably also needs a method for cleaning up the listener in the
/// case of an interrupt or error!
pub trait NativeAdapter {
    /// Adds the platform-specific listener that should post updates whenever
    /// the color mode changes.
    fn setup_mode_change_listener(&self, changes: Sender<ColorMode>);

    /// Query the OS for the current color mode.
    fn current_mode(&self) -> ColorMode;
}
