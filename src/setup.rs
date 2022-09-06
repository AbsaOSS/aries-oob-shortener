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
