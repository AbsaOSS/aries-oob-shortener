use redis::AsyncCommands;

use std::fmt::Debug;

use crate::error::prelude::*;

pub struct RedisClient {
    connection: redis::aio::Connection,
}

impl RedisClient {
    pub async fn connect(redis_url: &str) -> SResult<Self> {
        let client = redis::Client::open(redis_url).unwrap();
        let connection = client.get_async_connection().await.map_err(|err| {
            SError::from_msg(
                SErrorType::RedisError,
                &format!(
                    "Failed to connect to redis on {:?}, error: {:?}",
                    redis_url, err
                ),
            )
        })?;
        Ok(Self { connection })
    }

    pub async fn get(&mut self, key: &str) -> SResult<Option<String>> {
        Ok(self.connection.get(key).await.map_err(|err| {
            SError::from_msg(
                SErrorType::RedisError,
                &format!("Failed to get value for key {:?}, error: {:?}", key, err),
            )
        })?)
    }

    pub async fn set(
        &mut self,
        key: &str,
        value: &str,
        expire_in_secs: Option<u32>,
    ) -> SResult<()> {
        redis::Pipeline::new()
            .cmd("SET")
            .arg(key)
            .arg(value)
            .ignore()
            .expire(
                key,
                expire_in_secs
                    .unwrap_or(30 * 60)
                    .try_into()
                    .map_err(|err| {
                        SError::from_msg(
                            SErrorType::ParsingError,
                            &format!(
                                "Failed to convert expiration time to usize, error: {:?}",
                                err
                            ),
                        )
                    })?,
            )
            .query_async(&mut self.connection)
            .await
            .map_err(|err| {
                SError::from_msg(
                    SErrorType::RedisError,
                    &format!("Failed to set value for key {:?}, error: {:?}", key, err),
                )
            })
    }
}

impl Debug for RedisClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str("RedisClient")
    }
}

#[cfg(test)]
#[cfg(feature = "integration_test")]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_set() {
        let mut client = RedisClient::connect("redis://localhost:6379/0")
            .await
            .unwrap();
        client.set("abc", "hi", None).await.unwrap();
        let res = client.get("abc").await.unwrap().unwrap();
        assert_eq!(res, "hi");
        let res = client.get("xyz").await.unwrap();
        assert_eq!(res, None);
    }

    #[tokio::test]
    async fn test_expiration() {
        let mut client = RedisClient::connect("redis://localhost:6379/0")
            .await
            .unwrap();
        client.set("def", "hi", Some(1)).await.unwrap();
        tokio::time::sleep(std::time::Duration::new(2, 0)).await;
        let res = client.get("def").await.unwrap();
        assert_eq!(res, None);
    }

    #[tokio::test]
    async fn test_fails_to_connect_if_url_incorrect() {
        let err = RedisClient::connect("redis://nothere:9736/0")
            .await
            .unwrap_err();
        assert!(matches!(err.kind, SErrorType::RedisError));
    }
}
