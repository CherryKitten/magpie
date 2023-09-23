use config::Config;

use crate::Result;

pub fn get_config() -> Result<Config> {
    Ok(Config::builder()
        .set_default("db", "../../magpie.db")?
        .set_default("ssl", false)?
        .set_default("host", "localhost")?
        .set_default("port", 8080)?
        .set_default("dev", false)?
        .add_source(config::File::with_name("magpie.toml"))
        .add_source(config::Environment::with_prefix("MAGPIE"))
        .build()?)
}
