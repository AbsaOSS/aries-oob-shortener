pub mod services;

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tracing_actix_web::TracingLogger;

use crate::api::configure_scopes;
use crate::config::Config;
use crate::error::prelude::*;

fn load_rustls_config() -> rustls::ServerConfig {
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut std::io::BufReader::new(std::fs::File::open("certs/cert.pem").unwrap());
    let key_file = &mut std::io::BufReader::new(std::fs::File::open("certs/key.pem").unwrap());

    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

pub async fn build_server(config: &mut Config) -> SResult<Server> {
    let address = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&address)?;

    config.server.port = listener
        .local_addr()
        .map_err(|err| {
            SError::from_msg(
                SErrorType::IOError,
                &format!("No available random port found, error: {}", err),
            )
        })?
        .port();

    tracing::info!("Building services");
    let services = web::Data::new(services::build_services(config).await?);

    tracing::info!("Configuring server");
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(services.clone())
            .configure(configure_scopes)
    });

    if config.server.enable_tls {
        tracing::info!("TLS enabled");
        let rustls_config = load_rustls_config();
        server = server.listen_rustls(listener, rustls_config)?;
    } else {
        server = server.listen(listener)?;
    };

    Ok(server.run())
}
