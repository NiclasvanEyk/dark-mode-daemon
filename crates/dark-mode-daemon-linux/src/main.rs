mod gsettings;

use gsettings::{
    freedesktop::FreeDesktopSettingsProvider, gnome::GnomeDesktopSettingsProvider,
    GSettingsAdapter, SettingsProviderImplementation,
};

fn main() {
    let implementation = SettingsProviderImplementation::Gnome;

    match implementation {
        SettingsProviderImplementation::Gnome => {
            dark_mode_daemon::cli::run(GSettingsAdapter::<GnomeDesktopSettingsProvider>::new());
        }
        SettingsProviderImplementation::Freedesktop => {
            dark_mode_daemon::cli::run(GSettingsAdapter::<FreeDesktopSettingsProvider>::new());
        }
    };
}
