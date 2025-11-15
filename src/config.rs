use std::path::Path;

use crate::mode::ColorMode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct Config {
    /// The default colormode, if none can be inferred.
    /// Defaults to "dark" if not set.
    #[serde(default = "default_color_mode")]
    pub default_mode: ColorMode,

    /// By default defined scripts are run sequentially in alphabetical order.
    /// Set this to true to have them run in parallel, which speeds up the
    /// process a bit.
    #[serde(default = "default_run_scripts_in_parallel")]
    pub run_scripts_in_parallel: bool,

    /// Logs the output of each script into {logs_dir}/{script_name}.txt
    /// for debugging purposes.
    #[serde(default = "default_log_command_output")]
    pub log_command_output: bool,

    /// A directory that contains the
    #[serde(default = "default_logs_dir")]
    pub logs_dir: String,
}

fn default_color_mode() -> ColorMode {
    ColorMode::Dark
}

fn default_run_scripts_in_parallel() -> bool {
    false
}

fn default_log_command_output() -> bool {
    false
}

fn default_logs_dir() -> String {
    // TODO: Adjust this based on the OS
    "/var/log/dark-mode-daemon/script-outputs".into()
}

impl Config {
    pub fn try_read_from(file_path: &Path) -> anyhow::Result<Self> {
        let file_contents = read_to_string(file_path)?;
        toml::from_str(&file_contents).map_err(|err| err.into())
    }

    pub fn default() -> Self {
        Config {
            default_mode: default_color_mode(),
            run_scripts_in_parallel: default_run_scripts_in_parallel(),
            log_command_output: default_log_command_output(),
            logs_dir: default_logs_dir(),
        }
    }
}
