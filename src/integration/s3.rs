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

use std::fs;
use std::path::Path;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Region};

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
        let region = region.unwrap_or_else(|| "eu-west-1".into());
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

        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        };
        fs::write(path, &bytes)?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "aws_test")]
mod tests {
    use super::*;
    use crate::logging::init_logger;
    use tempfile::tempdir;

    #[tokio::test]
    async fn get_s3_data() {
        init_logger(None, None).unwrap();
        let path = tempdir()
            .unwrap()
            .path()
            .join("testdir")
            .join("testfile.txt");
        assert!(!path.exists());
        S3Client::new(S3ClientConfig { region: None })
            .await
            .unwrap()
            .get_object(
                "ctoblockchaindev-euw1-dev-dlt-localhost",
                "testread/testfile.txt",
                path.to_str().unwrap(),
            )
            .await
            .unwrap();
        assert!(path.exists());
    }
}
