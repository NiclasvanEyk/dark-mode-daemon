pub mod autostart;

use crate::{
    mode::ColorMode,
    platform::{ColorModeDaemon, ColorModeDetector},
};
use block2::RcBlock;
use objc2_app_kit::NSApplication;
use objc2_foundation::{ns_string, MainThreadMarker, NSDistributedNotificationCenter};

#[derive(Default)]
pub struct MacOsColorModeDetector {}

impl ColorModeDetector for MacOsColorModeDetector {
    async fn current_mode(&self) -> anyhow::Result<ColorMode> {
        current_mode()
    }
}

impl ColorModeDaemon for MacOsColorModeDetector {
    async fn on_color_changed<F>(&self, callback: F)
    where
        F: Fn(ColorMode) + 'static,
    {
        let execute_callback = RcBlock::new(move |_| {
            // FIXME: Error handling
            callback(current_mode().unwrap());
        });

        unsafe {
            let notification_center = NSDistributedNotificationCenter::defaultCenter();

            let name = ns_string!("AppleInterfaceThemeChangedNotification");
            notification_center.addObserverForName_object_queue_usingBlock(
                Some(name),
                None,
                None,
                &execute_callback,
            );
        };

        let mtm = MainThreadMarker::new().expect("must be on the main thread");

        NSApplication::sharedApplication(mtm).run();
    }
}

fn current_mode() -> anyhow::Result<ColorMode> {
    unsafe {
        let defaults = objc2_foundation::NSUserDefaults::standardUserDefaults();

        let Some(mode) = defaults.stringForKey(ns_string!("AppleInterfaceStyle")) else {
            // This seems to be empty when in light mode.
            return Ok(ColorMode::Light);
        };

        // This is just to be sure. In my tests, it was always empty when
        // in light mode, but we check against the documented values just
        // to be sure.
        if mode.to_string() == "Dark" {
            return Ok(ColorMode::Dark);
        }

        Ok(ColorMode::Light)
    }
}
