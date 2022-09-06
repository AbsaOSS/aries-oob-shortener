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

use std::sync::Once;

use tokio::task::JoinHandle;

use dlt_shortener::application::Application;
use dlt_shortener::configuration::{load_config, Config};
use dlt_shortener::error::prelude::*;
use dlt_shortener::logging::init_logger;

use crate::client::{Client, ClientConfig};

static TEST_LOGGING_INIT: Once = Once::new();

pub struct TestSetup {
    pub client: Client,
    pub app_config: Config,
    pub handle: JoinHandle<SResult<((), ())>>,
}

pub async fn init() -> TestSetup {
    TEST_LOGGING_INIT.call_once(|| {
        init_logger(None, Some("test")).unwrap();
    });
    let mut app_config = load_config().unwrap();
    app_config.server_internal.port = 0;
    app_config.server_external.port = 0;
    let app = Application::build(app_config.clone()).await.unwrap();
    let client_config = ClientConfig {
        host: app.config().server_internal.host,
        port_internal: app.config().server_internal.port,
        port_external: app.config().server_external.port,
    };
    let client = Client::build(client_config).unwrap();
    let handle = tokio::spawn(app.run_until_stopped());
    TestSetup {
        client,
        app_config,
        handle,
    }
}
