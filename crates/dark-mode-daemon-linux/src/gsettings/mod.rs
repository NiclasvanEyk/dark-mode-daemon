use std::{marker::PhantomData, sync::mpsc::Sender};

use dark_mode_daemon::{platform_specifics::NativeAdapter, ColorMode};
use gio::prelude::ObjectExt;
use gio::Settings;

pub mod freedesktop;
pub mod gnome;

// The actual schema id might vary. This is the gnome specific one, which
// works when I tested in inside a Fedora container.
// However, there is also a freedesktop standard, which could be used.
// Maybe this function should actually infer which schemas are present when
// constructing this (the result could be cached, so we don't do this work
// every time we re-create the settings object in the change signal
// handler). Depending on which schemas are present (maybe the standard one
// should be preferred), we could read the "right" one.
pub(crate) trait SettingsProvider {
    fn get_settings() -> Settings;

    fn get_color_mode(settings: &Settings) -> ColorMode;
}

pub(crate) enum SettingsProviderImplementation {
    Freedesktop,
    Gnome,
}

pub(crate) struct GSettingsAdapter<Provider>
where
    Provider: SettingsProvider,
{
    settings: Settings,
    provider: PhantomData<Provider>,
}

impl<Provider> GSettingsAdapter<Provider>
where
    Provider: SettingsProvider,
{
    pub(crate) fn new() -> Self {
        Self {
            settings: Provider::get_settings(),
            provider: PhantomData,
        }
    }
}

impl<Provider> NativeAdapter for GSettingsAdapter<Provider>
where
    Provider: SettingsProvider,
{
    fn setup_mode_change_listener(&self, changes: Sender<ColorMode>) {
        self.settings
            .connect("changed::color-scheme", true, move |_| {
                // `Settings` are (at the time of writing) not sync+send, so we
                // can't use the instance on self in this callback.
                let read_mode = Provider::get_color_mode(&Provider::get_settings());

                // FIXME: Only log if running in verbose mode
                println!("colorscheme changed!");
                println!("read colorscheme: {read_mode}");

                // FIXME: Maybe rather log or simply disconnect the listener?
                changes.send(read_mode).unwrap();
                None
            });
    }

    fn current_mode(&self) -> ColorMode {
        Provider::get_color_mode(&self.settings)
    }
}