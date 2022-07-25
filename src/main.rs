#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[macro_use]
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

use crate::error::prelude::*;
use crate::logging::init_logger;

#[actix_web::main]
async fn main() -> SResult<()> {
    // TODO: Since the logger can be initialized only once
    // but the application multiple times (e.g. tests), we want to separate logging 
    // and app config..how to configure logger, than? One common config with both
    // app and logger config merged.
    init_logger(true, None, None)?;
    let app = setup::build_application().await?;
    app.run_until_stopped().await
}
