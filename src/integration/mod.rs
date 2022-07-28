mod s3;
mod ecs;

use crate::error::prelude::*;
use crate::config::{CertificateConfig, AwsConfig};
use s3::{S3Client, S3ClientConfig};

pub async fn download_certs(cert_config: &CertificateConfig, aws_config: &AwsConfig) -> SResult<()> {
    let client = S3Client::new(S3ClientConfig { region: aws_config.region.clone() }).await?;
    client.get_object(
        &aws_config.s3.certificate_bucket,
        &aws_config.s3.certificate_path,
        &cert_config.certificate_path
    ).await?;
    client.get_object(
        &aws_config.s3.certificate_bucket,
        &aws_config.s3.certificate_key_path,
        &cert_config.certificate_key_path
    ).await?;
    client.get_object(
        &aws_config.s3.certificate_bucket,
        &aws_config.s3.certificate_authority_path,
        &cert_config.certificate_authority_path
    ).await?;
    Ok(())
}
