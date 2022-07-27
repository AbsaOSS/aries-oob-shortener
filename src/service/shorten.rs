use std::sync::Arc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use tokio::sync::Mutex;
use serde_json::Value;

use crate::error::prelude::*;
use crate::storage::RedisClient;
use crate::config::Config;

pub struct ServiceShorten {
    redis_client: Arc<Mutex<RedisClient>>,
    config: Config
}

fn calculate_hash<T: Hash>(t: T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    format!("{:x}", s.finish())
}

impl ServiceShorten {
    pub fn new(redis_client: Arc<Mutex<RedisClient>>, config: Config) -> Self {
        Self { redis_client, config }
    }

    pub async fn shorten(&self, base_url: &str, msg: &str, expire_in_secs: Option<u32>) -> SResult<String> {
        let hash = calculate_hash(msg);
        let shortened = format!("{}/{}", base_url, hash);
        self.redis_client
            .lock()
            .await
            .set(&hash, msg, expire_in_secs)
            .await?;
        Ok(shortened)
    }

    pub async fn get_message(&self, msg_hash: &str) -> SResult<Value> {
        let msg = self.redis_client
            .lock()
            .await
            .get(msg_hash)
            .await?;
        if let Some(msg) = msg {
            Ok(serde_json::from_str::<Value>(&msg)
                .map_err(|err| SError::from_msg(SErrorType::ParsingError, &format!("Failed to deserialize OOB msg {}, error: {}", msg, err)))?
            )
        } else {
            Err(SError::from_msg(SErrorType::NotFoundError, &format!("No OOB msg found for hash {}", msg_hash)))
        }
    }

    pub async fn get_long_url(&self, msg_hash: &str) -> SResult<String> {
        let msg = self.get_message(msg_hash).await?;
        let msg = serde_json::to_string(&msg)
            .map_err(|err| SError::from_msg(SErrorType::SerializationError, &format!("Failed to serialize OOB msg {}, error: {}", msg, err)))?;
        let encoded = base64::encode(&msg);
        let mut long = self.config.application.long_url_base.clone();
        long.set_path(&encoded);
        Ok(long.to_string())
    }
}

#[cfg(test)]
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
