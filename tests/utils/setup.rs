use std::sync::Once;

use tokio::task::JoinHandle;

use dlt_shortener::error::prelude::*;
use dlt_shortener::logging::init_logger;
use dlt_shortener::application::Application;
use dlt_shortener::config::{load_config, Config};

use crate::client::{Client, ClientConfig};

static TEST_LOGGING_INIT: Once = Once::new();

pub struct TestSetup<T> {
    pub client: Client,
    pub app_config: Config,
    pub handle: JoinHandle<T>
}

pub async fn init() -> TestSetup<Result<(), SError>> {
    TEST_LOGGING_INIT.call_once(|| {
        init_logger(false, Some("test"), Some("debug"), None).unwrap();
    });
    let mut app_config = load_config().unwrap(); 
    app_config.server.port = 0;
    let app = Application::build(app_config.clone()).await.unwrap();
    let client_config = ClientConfig {
        host: app.config().server.host,
        port: app.config().server.port
    };
    let client = Client::build(client_config).unwrap();
    let handle = tokio::spawn(app.run_until_stopped());
    TestSetup { client, app_config, handle }
}
