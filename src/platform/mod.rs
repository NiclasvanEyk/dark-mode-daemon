#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

use crate::mode::ColorMode;
use std::{error::Error, thread::sleep, time::Duration};

/// Adapter that encapsulates platform-specific parts of dark mode daemon.
/// TODO: This probably also needs a method for cleaning up the listener in the
/// case of an interrupt or error!
pub trait NativeAdapter {
    /// Implementation of the deamon, which differs between OSes.
    fn run_daemon<F>(
        &self,
        on_color_detected: F,
        verbose: bool,
    ) -> impl std::future::Future<Output = ()>
    where
        F: Fn(ColorMode);

    /// Query the OS for the current color mode.
    fn current_mode(&self) -> Result<ColorMode, Box<dyn Error>>;
}

pub trait ColorModeDaemon {
    fn on_color_changed<F>(&self, callback: F) -> impl std::future::Future<Output = ()>
    where
        F: Fn(ColorMode) + 'static;
}

pub trait ColorModeDetector {
    /// Query the OS for the current color mode.
    fn current_mode(&self) -> impl std::future::Future<Output = anyhow::Result<ColorMode>>;
}

pub struct PollingColorModeDaemon<Detector>
where
    Detector: ColorModeDetector,
{
    detector: Detector,
    sleep_duration: Duration,
}

impl<Detector> PollingColorModeDaemon<Detector>
where
    Detector: ColorModeDetector,
{
    pub fn new(detector: Detector, sleep_duration: Duration) -> Self {
        Self {
            detector,
            sleep_duration,
        }
    }
}

impl<Detector> ColorModeDetector for PollingColorModeDaemon<Detector>
where
    Detector: ColorModeDetector,
{
    async fn current_mode(&self) -> anyhow::Result<ColorMode> {
        self.detector.current_mode().await
    }
}

impl<Detector> ColorModeDaemon for PollingColorModeDaemon<Detector>
where
    Detector: ColorModeDetector,
{
    async fn on_color_changed<F>(&self, callback: F)
    where
        F: Fn(ColorMode),
    {
        let mut previous_mode = self.detector.current_mode().await.unwrap();
        loop {
            sleep(self.sleep_duration);
            let current_mode = self.detector.current_mode().await.unwrap();
            if previous_mode != current_mode {
                previous_mode = current_mode;
                callback(current_mode);
            }
        }
    }
}
