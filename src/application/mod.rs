use actix_web::dev::Server;

use crate::configuration::Config;
use crate::error::prelude::*;
use crate::server::{build_server_external, build_server_internal};

#[allow(dead_code)]
pub struct Application {
    server_internal: Server,
    server_external: Server,
    config: Config,
}

#[allow(dead_code)]
impl Application {
    pub async fn build(mut config: Config) -> SResult<Self> {
        let server_internal = build_server_internal(&mut config).await?;
        let server_external = build_server_external(&mut config).await?;
        Ok(Self {
            server_internal,
            server_external,
            config,
        })
    }

    pub async fn run_until_stopped(self) -> SResult<((), ())> {
        tokio::try_join!(self.server_internal, self.server_external).map_err(|err| err.into())
    }

    pub fn config(&self) -> Config {
        self.config.clone()
    }
}
