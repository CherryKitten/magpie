use std::path::{Path, PathBuf};

pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub test_path: PathBuf,
}

pub fn get_config() -> AppConfig {
    AppConfig {
        host: "localhost".to_string(),
        port: 8080,
        //test_path: PathBuf::from(Path::new("../test_data/music")),
        test_path: PathBuf::from(Path::new("/Volumes/Media/music")),
    }
}
