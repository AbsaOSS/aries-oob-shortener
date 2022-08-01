use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Region};

use std::fs::File;
use std::io::Write;

use crate::error::prelude::*;

#[derive(Clone)]
pub struct S3ClientConfig {
    pub region: Option<String>,
}

pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub async fn new(config: S3ClientConfig) -> SResult<Self> {
        let S3ClientConfig { region } = config.clone();
        let region = region.unwrap_or("eu-west-1".into());
        let region_provider = RegionProviderChain::first_try(Region::new(region.clone()))
            .or_default_provider()
            .or_else(Region::new(region.clone()));
        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&shared_config);
        Ok(Self { client })
    }

    pub async fn get_object(&self, bucket: &str, object: &str, path: &str) -> SResult<()> {
        let response = self
            .client
            .get_object()
            .bucket(bucket)
            .key(object)
            .send()
            .await?;

        let bytes = response
            .body
            .collect()
            .await
            .map_err(|err| {
                SError::from_msg(
                    SErrorType::IOError,
                    &format!("Failed to load response byte stream, error: {}", err),
                )
            })?
            .into_bytes();
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "aws_test")]
mod tests {
    use super::*;
    use crate::logging::init_logger;

    #[tokio::test]
    async fn get_s3_data() {
        init_logger(None, None).unwrap();
        let mut tmp_dir = std::env::temp_dir();
        tmp_dir.push("testfile.txt");
        S3Client::new(S3ClientConfig { region: None })
            .await
            .unwrap()
            .get_object(
                "ctoblockchaindev-euw1-dev-dlt-localhost",
                "testread/testfile.txt",
                tmp_dir.to_str().unwrap(),
            )
            .await
            .unwrap();
        assert!(tmp_dir.exists());
    }
}
