mod config;
mod env;

pub use self::config::*;

use ::config as configrs;

use crate::error::prelude::*;

pub fn load_config() -> SResult<config::Config> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    let config = match std::env::var("APP_CONFIG").ok() {
        Some(config) => {
            let env: env::Env = config.try_into()
                .expect("Failed to parse APP_CONFIG environment variable; allowed values are `localhost`");
            let environment_filename = format!("{}.toml", env.as_str());
            tracing::info!("App configuration will be loaded from {}", environment_filename);
            configrs::Config::builder()
                .add_source(configrs::File::from(configuration_directory.join(&environment_filename)))
                .build()
                .map_err(|err| SError::from_msg(SErrorType::ConfigurationError, &format!("Failed to build configuration, error: {}", err)))?
        }
        None => {
            tracing::info!("App configuration will be loaded from environment variables");
            configrs::Config::builder()
                .add_source(configrs::Environment::default().separator("__"))
                .build()
                .map_err(|err| SError::from_msg(SErrorType::ConfigurationError, &format!("Failed to build configuration, error: {}", err)))?
        }
    };
    
    config.try_deserialize::<config::Config>()
        .map_err(|err| SError::from_msg(SErrorType::ParsingError, &format!("Failed to deserialize Config, error: {}", err)))
}
