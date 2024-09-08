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
