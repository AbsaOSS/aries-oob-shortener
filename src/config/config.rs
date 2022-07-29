use serde_aux::field_attributes::{deserialize_number_from_string, deserialize_bool_from_anything};

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationConfig {
    pub short_url_base: url::Url,
    pub long_url_base: url::Url
}

#[derive(serde::Deserialize, Clone)]
pub struct CertificateConfig {
    pub certificate_path: String,
    pub certificate_key_path: String,
    pub certificate_authority_path: String
}

#[derive(serde::Deserialize, Clone)]
pub struct RedisConfig {
    pub redis_url: url::Url
}

#[derive(serde::Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub enable_tls: bool
}

#[derive(serde::Deserialize, Clone)]
pub struct AwsConfig {
    pub region: Option<String>,
    pub s3: S3Config
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
    pub redis: RedisConfig,
    pub application: ApplicationConfig,
    pub server: ServerConfig,
    pub certs: Option<CertificateConfig>,
    pub aws: Option<AwsConfig>
}
