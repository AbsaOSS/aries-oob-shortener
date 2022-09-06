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

use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationConfig {
    pub short_url_base: url::Url,
    pub long_url_base: url::Url,
    pub default_expire_in_sec: Option<u32>,
}

#[derive(serde::Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub certs: Option<CertificateConfig>,
    pub aws: Option<AwsConfig>,
}

#[derive(serde::Deserialize, Clone)]
pub struct CertificateConfig {
    pub certificate_path: String,
    pub certificate_key_path: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct S3Config {
    pub certificate_bucket: String,
    pub certificate_path: String,
    pub certificate_key_path: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct AwsConfig {
    pub region: Option<String>,
    pub s3: S3Config,
}

#[derive(serde::Deserialize, Clone)]
pub struct RedisConfig {
    pub url: url::Url,
}

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub server_internal: ServerConfig,
    pub server_external: ServerConfig,
    pub application: ApplicationConfig,
    pub redis: RedisConfig,
}
