use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(unused)]
pub(crate) struct Settings {
    pub(crate) debug: bool,
    pub(crate) database_url: String,
}

impl Settings {
    pub fn new(config_path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let mut s = Config::builder()
            .set_default("debug", false)?
            .set_default("database_url", "sqlite::memory:")?;

        if let Some(config_path) = config_path {
            s = s.add_source(File::from(config_path));
        } else {
            s = s
                .add_source(File::with_name("./config.yml").required(false))
                .add_source(File::with_name("/etc/magpie/config.yml").required(false))
        }

        let s = s.add_source(Environment::with_prefix("magpie")).build()?;

        s.try_deserialize()
    }
}
