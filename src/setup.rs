use crate::application::Application;
use crate::configuration::load_config;
use crate::error::prelude::*;
use crate::integration::download_certs;

pub async fn build_application() -> SResult<Application> {
    let config = load_config()?;
    if let (Some(cert_config), Some(aws_config)) = (
        config.server_internal.certs.clone(),
        config.server_internal.aws.clone(),
    ) {
        tracing::info!("Downloading internal certificates");
        download_certs(&cert_config, &aws_config).await?;
    };
    if let (Some(cert_config), Some(aws_config)) = (
        config.server_external.certs.clone(),
        config.server_external.aws.clone(),
    ) {
        tracing::info!("Downloading external certificates");
        download_certs(&cert_config, &aws_config).await?;
    };
    Application::build(config).await
}
