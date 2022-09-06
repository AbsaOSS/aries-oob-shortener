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

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::Mutex;

use crate::configuration::Config;
use crate::error::prelude::*;
use crate::storage::RedisClient;

pub struct ServiceShorten {
    redis_client: Arc<Mutex<RedisClient>>,
    config: Config,
}

fn calculate_hash<T: Hash>(t: T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    format!("{:x}", s.finish())
}

fn deserialize_oob_msg(msg: &str) -> SResult<Value> {
    serde_json::from_str::<Value>(msg).map_err(|err| {
        SError::from_msg(
            SErrorType::ParsingError,
            &format!("Failed to deserialize OOB msg {}, error: {}", msg, err),
        )
    })
}

impl ServiceShorten {
    pub fn new(redis_client: Arc<Mutex<RedisClient>>, config: Config) -> Self {
        Self {
            redis_client,
            config,
        }
    }

    pub async fn shorten(
        &self,
        msg: &str,
        base_url: Option<url::Url>,
        expire_in_secs: Option<u32>,
    ) -> SResult<String> {
        deserialize_oob_msg(msg)?;
        let hash = calculate_hash(msg);
        let base = base_url.unwrap_or_else(|| self.config.application.short_url_base.clone());
        let shortened = base.join(&hash).map_err(|err| {
            SError::from_msg(
                SErrorType::ParsingError,
                &format!("Failed to construct shortened url, error: {}", err),
            )
        })?;
        self.redis_client
            .lock()
            .await
            .set(&hash, msg, expire_in_secs)
            .await?;
        Ok(shortened.to_string())
    }

    pub async fn get_message(&self, msg_hash: &str) -> SResult<Value> {
        let msg = self.redis_client.lock().await.get(msg_hash).await?;
        if let Some(msg) = msg {
            deserialize_oob_msg(&msg)
        } else {
            Err(SError::from_msg(
                SErrorType::NotFoundError,
                &format!("No OOB msg found for hash {}", msg_hash),
            ))
        }
    }

    pub async fn get_long_url(&self, msg_hash: &str) -> SResult<String> {
        let msg = self.get_message(msg_hash).await?;
        let msg = serde_json::to_string(&msg).map_err(|err| {
            SError::from_msg(
                SErrorType::SerializationError,
                &format!("Failed to serialize OOB msg {}, error: {}", msg, err),
            )
        })?;
        let encoded = base64::encode(&msg);
        let mut long = self.config.application.long_url_base.clone();
        long.set_query(Some(&format!("oob={}", encoded)));
        Ok(long.to_string())
    }
}

#[cfg(test)]
#[cfg(feature = "unit_test")]
mod test {
    use super::*;

    #[test]
    fn test_calculate_hash() {
        let hash1 = calculate_hash("abc");
        let hash2 = calculate_hash("abc");
        let hash3 = calculate_hash("def");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
