use std::sync::Arc;

use tokio::sync::Mutex;

use crate::error::prelude::*;
use crate::config::Config;
use crate::service::ServiceShorten;
use crate::storage::RedisClient;

pub struct Services {
    pub service_shorten: ServiceShorten
}

pub async fn build_services(config: &Config) -> SResult<Services> {
    let redis_client = Arc::new(Mutex::new(RedisClient::connect(config.redis.redis_url.as_str()).await?));
    let service_shorten = ServiceShorten::new(redis_client.clone(), config.clone());
    Ok(Services {
        service_shorten
    })
}
