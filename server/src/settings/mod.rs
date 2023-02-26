use anyhow::Result;
use config::Config;

pub fn get_config() -> Result<Config> {
    Ok(Config::builder()
        .set_default("db", "../../magpie.db")?
        .add_source(config::File::with_name("magpie.toml"))
        .add_source(config::Environment::with_prefix("MAGPIE"))
        .build()?)
}
