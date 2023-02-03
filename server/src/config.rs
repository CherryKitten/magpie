pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

pub(crate) fn get_config() -> AppConfig {
    AppConfig{
        host: "localhost".to_string(),
        port: 8080
    }
}
