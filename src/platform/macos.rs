use std::error::Error;

use crate::{execution::run_scripts, mode::ColorMode, platform::NativeAdapter};
use block2::RcBlock;
use objc2_app_kit::NSApplication;
use objc2_foundation::{ns_string, MainThreadMarker, NSDistributedNotificationCenter};

#[derive(Default)]
pub(crate) struct MacOsAdapter {}

fn current_mode() -> Result<ColorMode, Box<dyn Error>> {
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

impl NativeAdapter for MacOsAdapter {
    /// Implementation of the deamon, which differs between OSes.
    fn run_daemon<F>(&self, on_color_detected: F, verbose: bool)
    where
        F: Fn(ColorMode),
    {
        unsafe {
            let notification_center = NSDistributedNotificationCenter::defaultCenter();

            println!("Adding observer...");
            let name = ns_string!("AppleInterfaceThemeChangedNotification");
            notification_center.addObserverForName_object_queue_usingBlock(
                Some(name),
                None,
                None,
                &RcBlock::new(move |_| {
                    // FIXME: Error handling
                    on_color_detected(current_mode().unwrap());
                }),
            );
        };

        let mtm = MainThreadMarker::new().expect("must be on the main thread");

        println!("😈 Listening for color mode changes...");
        NSApplication::sharedApplication(mtm).run();
    }

    fn current_mode(&self) -> Result<ColorMode, Box<dyn Error>> {
        current_mode()
    }
}
