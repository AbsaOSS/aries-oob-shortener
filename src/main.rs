#[allow(unused_imports)]
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
    init_logger(true, None, None)?;
    let app = setup::build_application().await?;
    app.run_until_stopped().await
}
