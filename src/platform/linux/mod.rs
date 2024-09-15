use futures::StreamExt;
use std::boxed::Box;
use std::error::Error;
use std::result::Result;

use crate::execution::run_scripts;
use crate::{mode::ColorMode, platform::NativeAdapter};

#[derive(Default)]
pub struct LinuxAdapter {}

async fn run_scripts_on_mode_change(verbose: bool) -> anyhow::Result<()> {
    while let Some(mode) = dark_light::subscribe().await?.next().await {
        let color_mode = match mode {
            dark_light::Mode::Dark => ColorMode::Dark,
            dark_light::Mode::Light => ColorMode::Light,
            dark_light::Mode::Default => ColorMode::Light,
        };
        run_scripts(color_mode, verbose, true);
    }

    Ok(())
}
impl NativeAdapter for LinuxAdapter {
    fn run_daemon(&self, verbose: bool) {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                println!("😈 Listening for color mode changes...");
                let _ = run_scripts_on_mode_change(verbose).await;
            })
    }

    fn current_mode(&self) -> Result<ColorMode, Box<(dyn Error)>> {
        let mode = match dark_light::detect() {
            dark_light::Mode::Dark => ColorMode::Dark,
            dark_light::Mode::Light => ColorMode::Light,
            dark_light::Mode::Default => ColorMode::Light,
        };

        Ok(mode)
    }
}
