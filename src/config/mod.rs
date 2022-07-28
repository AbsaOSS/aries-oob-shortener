mod config;
mod env;

pub use self::config::*;

use ::config as configrs;

use crate::error::prelude::*;

pub fn load_config() -> SResult<config::Config> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    let env: env::Env = std::env::var("APP_CONFIG")
        .unwrap_or_else(|_| "localhost".into())
        .try_into()
        .expect("Failed to parse APP_CONFIG");
    let environment_filename = format!("{}.toml", env.as_str());

    let config = configrs::Config::builder()
        .add_source(configrs::File::from(configuration_directory.join(&environment_filename)))
        .add_source(configrs::Environment::default())
        .build()
        .map_err(|err| SError::from_msg(SErrorType::ConfigurationError, &format!("Failed to build configuration, error: {}", err)))?;
    
    config.try_deserialize::<config::Config>()
        .map_err(|err| SError::from_msg(SErrorType::ParsingError, &format!("Failed to deserialize Config, error: {}", err)))
}
