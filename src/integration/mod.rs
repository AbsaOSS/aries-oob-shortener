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

#[allow(dead_code)]
mod ecs;
mod s3;

use std::collections::HashMap;

use crate::configuration::{AwsConfig, CertificateConfig};
use crate::error::prelude::*;

use s3::{S3Client, S3ClientConfig};
use serde_json::Value;

pub async fn download_certs(
    cert_config: &CertificateConfig,
    aws_config: &AwsConfig,
) -> SResult<()> {
    let client = S3Client::new(S3ClientConfig {
        region: aws_config.region.clone(),
    })
    .await?;
    client
        .get_object(
            &aws_config.s3.certificate_bucket,
            &aws_config.s3.certificate_path,
            &cert_config.certificate_path,
        )
        .await?;
    client
        .get_object(
            &aws_config.s3.certificate_bucket,
            &aws_config.s3.certificate_key_path,
            &cert_config.certificate_key_path,
        )
        .await?;
    Ok(())
}

// TODO: There must be a better way to do this
fn metadata_to_hashmap(metadata: &ecs::EcsTaskMetadata) -> SResult<HashMap<String, Value>> {
    let temp = serde_json::to_string(metadata).unwrap();
    let map: HashMap<String, Value> = serde_json::from_str(&temp).unwrap();
    Ok(map)
}

#[allow(dead_code)]
pub async fn try_get_ecs_task_metadata() -> SResult<Option<HashMap<String, Value>>> {
    if let Ok(url) = std::env::var("ECS_CONTAINER_METADATA_URI_V4") {
        let metadata = ecs::fetch_ecs_task_metadata(&url).await?;
        let map = metadata_to_hashmap(&metadata)?;
        Ok(Some(map))
    } else {
        Ok(None)
    }
}
