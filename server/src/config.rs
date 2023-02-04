use std::path::{Path, PathBuf};

pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub test_path: PathBuf,
}

pub(crate) fn get_config() -> AppConfig {
    AppConfig {
        host: "localhost".to_string(),
        port: 8080,
        test_path: PathBuf::from(Path::new("../test_data/music")),
    }
}
