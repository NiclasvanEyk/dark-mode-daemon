use dark_mode_daemon::ColorMode;
use gio::{prelude::SettingsExtManual, Settings};

use super::SettingsProvider;

pub struct GnomeDesktopSettingsProvider {}

impl SettingsProvider for GnomeDesktopSettingsProvider {
    fn get_settings() -> Settings {
        // https://github.com/GNOME/gsettings-desktop-schemas/blob/3ed4080d2403d5bf24d0f765f97a01dd511483e3/schemas/org.gnome.desktop.interface.gschema.xml.in#L302
        Settings::new("org.gnome.desktop.interface")
    }

    fn get_color_mode(settings: &Settings) -> ColorMode {
        let color_scheme: String = settings.get("color-scheme");

        // See https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html
        // for possible values.
        // Possible values are here:
        // https://github.com/GNOME/gsettings-desktop-schemas/blob/3ed4080d2403d5bf24d0f765f97a01dd511483e3/schemas/org.gnome.desktop.interface.gschema.xml.in#L302
        if color_scheme == "prefer-dark" {
            return ColorMode::Dark;
        }

        ColorMode::Light
    }
}
