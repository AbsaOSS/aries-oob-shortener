/*
 * Copyright 2022 ABSA Group Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod scopes;

use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
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

    let services = web::Data::new(build_services(config).await?);
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Logger::default().exclude_regex("/health"))
            .app_data(services.clone())
            .configure(scopes::configure_scopes_external)
    });

    if let Some(cert_config) = &config.server_external.certs {
        tracing::info!("External server: TLS enabled");
        let rustls_config = load_rustls_config(
            &cert_config.certificate_path,
            &cert_config.certificate_key_path,
        );
        server = server.listen_rustls(listener, rustls_config)?;
    } else {
        tracing::info!("TLS disabled");
        server = server.listen(listener)?;
    };

    tracing::info!("External server: Starting with config: {:?}", config.server_external);
    Ok(server.run())
}
