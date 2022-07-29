#[macro_use]
extern crate log;
extern crate tracing;
#[macro_use]
extern crate serde_json;

mod api;
mod config;
mod logging;
mod service;
mod storage;
mod server;
mod error;
mod application;
mod setup;
mod integration;

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
    let app = setup::build_application().await?;
    app.run_until_stopped().await
}
