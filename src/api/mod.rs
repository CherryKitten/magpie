use crate::db::DbPool;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub mod response_container;
pub mod routes;

pub async fn run(pool: DbPool) -> Result<()> {
    let config = super::settings::get_config()?;
    let dev = config.get_bool("dev")?;

    let server = HttpServer::new(move || {
        let mut cors = Cors::default();
        if dev {
            cors = cors.allow_any_origin();
        };

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
            .wrap(Logger::default())
            .wrap(cors)
    });

    let address = (config.get_string("host")?, config.get_int("port")? as u16);

    info!("Starting API webserver on {}:{}", address.0, address.1);

    if config.get_bool("ssl")? {
        let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

        ssl_builder.set_private_key_file(config.get_string("ssl_key_file")?, SslFiletype::PEM)?;
        ssl_builder.set_certificate_chain_file(config.get_string("ssl_cert_file")?)?;

        server.bind_openssl(address, ssl_builder)?.run().await?;
    } else {
        server.bind(address)?.run().await?;
    }

    Ok(())
}
