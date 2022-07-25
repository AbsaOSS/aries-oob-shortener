use redis::AsyncCommands;

use std::fmt::Debug;

use crate::error::prelude::*;

pub struct RedisClient {
   connection: redis::aio::Connection
}

impl RedisClient {
    pub async fn connect(redis_url: &str) -> SResult<Self> {
        let client = redis::Client::open(redis_url).unwrap();
        let connection = client.get_async_connection()
            .await
            .map_err(|err| SError::from_msg(SErrorType::RedisError, &format!("Failed to connect to redis on {:?}, error: {:?}", redis_url, err)))?;
        Ok(Self { connection })
    }

    pub async fn get(&mut self, key: &str) -> SResult<Option<String>> {
        Ok(self.connection.get(key).await.unwrap())
    }

    // TODO: Use generics
    pub async fn set(&mut self, key: &str, value: &str) -> SResult<()> {
        let _: () = self.connection.set(key, value).await.unwrap();
        Ok(())
    }
}

impl Debug for RedisClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str("RedisClient")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_set() {
       let mut client = RedisClient::connect("redis://localhost:6379/0").await.unwrap();
       client.set("abc", "hi").await.unwrap();
       let res = client.get("abc").await.unwrap().unwrap();
       assert_eq!(res, "hi");
       let res = client.get("xyz").await.unwrap();
       assert_eq!(res, None);
    }

    #[tokio::test]
    async fn test_fails_to_connect_if_url_incorrect() {
        let err = RedisClient::connect("redis://nothere:9736/0").await.unwrap_err();
        assert!(matches!(err.kind, SErrorType::RedisError));
    }
}
