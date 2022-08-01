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
    client
        .get_object(
            &aws_config.s3.certificate_bucket,
            &aws_config.s3.certificate_authority_path,
            &cert_config.certificate_authority_path,
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
