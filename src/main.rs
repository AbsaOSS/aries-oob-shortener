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

#[macro_use]
extern crate log;
extern crate tracing;
#[macro_use]
extern crate serde_json;

mod api;
mod application;
mod configuration;
mod error;
mod integration;
mod logging;
mod server;
mod service;
mod setup;
mod storage;

use crate::error::prelude::*;
use crate::logging::init_logger;

#[actix_web::main]
async fn main() -> SResult<()> {
    let ecs_task_metadata = match integration::try_get_ecs_task_metadata().await {
        Ok(metadata) => metadata,
        Err(err) => {
            warn!("Encountered an error when fetching ECS metadata: {}, ECS metadata will not be used", err);
            None
        }
    };
    init_logger(ecs_task_metadata, Some("dlt-shortener"))?;
    tracing::info!("Logger initialized");
    let app = setup::build_application().await?;
    app.run_until_stopped().await.map(|res| res.0)
}
