use config::{Environment, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(unused)]
pub(crate) struct Config {
    pub(crate) database_url: String,
    pub(crate) library_path: String,
}

impl Config {
    pub fn new(config_path: Option<PathBuf>) -> Result<Self, config::ConfigError> {
        let mut s = config::Config::builder()
            .set_default("debug", false)?
            .set_default("database_url", "sqlite::memory:")?;

        if let Some(config_path) = config_path {
            s = s.add_source(File::from(config_path));
        } else {
            s = s
                .add_source(File::new("./config.yml", FileFormat::Yaml).required(false))
                .add_source(File::new("/etc/magpie/config.yml", FileFormat::Yaml).required(false))
        }

        s = s.add_source(Environment::with_prefix("magpie"));

        let s = s.build()?;

        s.try_deserialize()
    }
}
