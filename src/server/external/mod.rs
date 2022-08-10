mod scopes;

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::configuration::Config;
use crate::error::prelude::*;
use crate::server::tls::load_rustls_config;
use crate::service::build_services;

pub async fn build_server_external(config: &mut Config) -> SResult<Server> {
    let address = format!(
        "{}:{}",
        config.server_external.host, config.server_external.port
    );
    let listener = TcpListener::bind(&address)?;

    config.server_external.port = listener
        .local_addr()
        .map_err(|err| {
            SError::from_msg(
                SErrorType::IOError,
                &format!("Error allocating port: {}", err),
            )
        })?
        .port();

    tracing::info!("Building services");
    let services = web::Data::new(build_services(config).await?);

    tracing::info!("Configuring internal server");
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(services.clone())
            .configure(scopes::configure_scopes_external)
    });

    if config.server_external.enable_tls {
        tracing::info!("TLS enabled");
        let rustls_config = load_rustls_config();
        server = server.listen_rustls(listener, rustls_config)?;
    } else {
        server = server.listen(listener)?;
    };

    Ok(server.run())
}
