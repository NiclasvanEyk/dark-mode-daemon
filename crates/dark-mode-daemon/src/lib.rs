use std::fmt::Display;

pub mod cli;
pub mod discovery;
pub mod execution;
pub mod platform_specifics;

use clap::ValueEnum;

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
