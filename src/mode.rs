use clap::ValueEnum;
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ColorMode {
    Light,
    Dark,
}

impl Display for ColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMode::Light => write!(f, "light"),
            ColorMode::Dark => write!(f, "dark"),
        }
    }
}

impl ColorMode {
    pub fn other(&self) -> ColorMode {
        match self {
            ColorMode::Light => ColorMode::Dark,
            ColorMode::Dark => ColorMode::Light,
        }
    }

    pub fn emoji(&self) -> String {
        match self {
            ColorMode::Light => String::from("🌞"),
            ColorMode::Dark => String::from("🌌"),
        }
    }
}
