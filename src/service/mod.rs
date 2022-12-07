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

mod shorten;

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::configuration::Config;
use crate::error::prelude::*;
use crate::service::shorten::ServiceShorten;
use crate::storage::RedisClient;

pub struct Services {
    pub service_shorten: ServiceShorten,
}

pub async fn build_services(config: &Config) -> SResult<Services> {
    let redis_client = Arc::new(Mutex::new(
        RedisClient::connect(config.redis.url.as_str()).await?,
    ));
    info!("Building service layer with application config: {}, redis config: {}", json!(config.application), json!(config.redis));
    let service_shorten = ServiceShorten::new(redis_client, config.clone());
    Ok(Services { service_shorten })
}
