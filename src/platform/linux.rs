use ashpd::desktop::settings::{ColorScheme as GnomeColorMode, Settings as GnomeSettings};
use futures::StreamExt;

use crate::{
    mode::ColorMode,
    platform::{ColorModeDaemon, ColorModeDetector},
};

#[derive(Default)]
pub struct LinuxAdapter {}

impl From<GnomeColorMode> for ColorMode {
    fn from(value: GnomeColorMode) -> Self {
        match value {
            GnomeColorMode::NoPreference => ColorMode::Light,
            GnomeColorMode::PreferDark => ColorMode::Dark,
            GnomeColorMode::PreferLight => ColorMode::Light,
        }
    }
}

pub struct LinuxColorModeDetector<'a> {
    settings: GnomeSettings<'a>,
}

impl<'a> LinuxColorModeDetector<'a> {
    pub async fn default() -> anyhow::Result<Self> {
        let settings = GnomeSettings::new().await?;

        Ok(Self { settings })
    }
}

impl<'a> ColorModeDetector for LinuxColorModeDetector<'a> {
    async fn current_mode(&self) -> anyhow::Result<ColorMode> {
        let color_scheme = self.settings.color_scheme().await?;
        Ok(ColorMode::from(color_scheme))
    }
}

impl<'a> ColorModeDaemon for LinuxColorModeDetector<'a> {
    async fn on_color_changed<F>(&self, callback: F)
    where
        F: Fn(ColorMode),
    {
        let mut color_mode_changes = self
            .settings
            .receive_color_scheme_changed()
            .await
            .unwrap()
            .map(|color_scheme| ColorMode::from(color_scheme));

        // Tests showed, that for some reason the mode change is triggered twice
        // right after one another. To prevent from running our scripts twice, we
        // do some deduplication here.
        let mut previous_mode: Option<ColorMode> = None;
        while let Some(mode) = color_mode_changes.next().await {
            if mode != previous_mode.unwrap_or_else(|| mode.other()) {
                previous_mode = Some(mode);
                callback(mode);
            }
        }
    }
}
