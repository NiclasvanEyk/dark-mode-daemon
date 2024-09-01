use crate::ColorMode;
use gio::{prelude::SettingsExtManual, Settings};

use super::SettingsProvider;

pub struct FreeDesktopSettingsProvider {}

impl SettingsProvider for FreeDesktopSettingsProvider {
    fn get_settings() -> Settings {
        // https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html
        Settings::new("org.freedesktop.appearance")
    }

    fn get_color_mode(settings: &Settings) -> ColorMode {
        let color_scheme: u32 = settings.get("color-scheme");

        if color_scheme == 1 {
            return ColorMode::Dark;
        }

        ColorMode::Light
    }
}
