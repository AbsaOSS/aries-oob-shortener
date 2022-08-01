use actix_web::dev::Server;

use crate::config::Config;
use crate::error::prelude::*;
use crate::server::build_server;

#[allow(dead_code)]
pub struct Application {
    server: Server,
    config: Config,
}

#[allow(dead_code)]
impl Application {
    pub async fn build(mut config: Config) -> SResult<Self> {
        let server = build_server(&mut config).await?;
        Ok(Self { server, config })
    }

    pub async fn run_until_stopped(self) -> SResult<()> {
        self.server.await.map_err(|err| err.into())
    }

    pub fn config(&self) -> Config {
        self.config.clone()
    }
}
