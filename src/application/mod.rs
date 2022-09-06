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
