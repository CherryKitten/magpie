use clap::{Parser, ValueEnum};

pub mod error;
pub mod db;

pub use error::Result;

/// Cute music player oWo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// What mode to run in
    #[arg(value_enum)]
    pub mode: Option<Mode>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Headless
    Headless,
    /// With native GUI
    Gui,
    /// With CLI
    Cli,
}
