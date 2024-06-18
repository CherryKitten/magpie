use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short = 'c', long = "config")]
    pub(crate) config_path: Option<std::path::PathBuf>,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Commands {
    Server,
    Admin,
    Debug,
}
