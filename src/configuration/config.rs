use serde_aux::field_attributes::{deserialize_bool_from_anything, deserialize_number_from_string};

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationConfig {
    pub short_url_base: url::Url,
    pub long_url_base: url::Url,
}

#[derive(serde::Deserialize, Clone)]
pub struct CertificateConfig {
    pub certificate_path: String,
    pub certificate_key_path: String,
    pub certificate_authority_path: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct RedisConfig {
    pub url: url::Url,
}

#[derive(serde::Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub enable_tls: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct AwsConfig {
    pub region: Option<String>,
    pub s3: S3Config,
}

#[derive(serde::Deserialize, Clone)]
pub struct S3Config {
    pub certificate_bucket: String,
    pub certificate_path: String,
    pub certificate_key_path: String,
    pub certificate_authority_path: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub server_internal: ServerConfig,
    pub server_external: ServerConfig,
    pub application: ApplicationConfig,
    pub redis: RedisConfig,
    pub certs: Option<CertificateConfig>,
    pub aws: Option<AwsConfig>,
}
