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

mod config;
mod env;

pub use self::config::*;

use ::config as configrs;

use crate::error::prelude::*;

pub fn load_config() -> SResult<Config> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    let config = match std::env::var("APP_CONFIG").ok() {
        Some(config) => {
            let env: env::Env = config.try_into().expect(
                "Failed to parse APP_CONFIG environment variable; allowed values are `localhost`",
            );
            let environment_filename = format!("{}.toml", env.as_str());
            tracing::info!(
                "App configuration will be loaded from {}",
                environment_filename
            );
            configrs::Config::builder()
                .add_source(configrs::File::from(
                    configuration_directory.join(&environment_filename),
                ))
                .build()
                .map_err(|err| {
                    SError::from_msg(
                        SErrorType::ConfigurationError,
                        &format!("Failed to build configuration, error: {}", err),
                    )
                })?
        }
        None => {
            tracing::info!("App configuration will be loaded from environment variables");
            configrs::Config::builder()
                .add_source(configrs::Environment::default().separator("::"))
                .build()
                .map_err(|err| {
                    SError::from_msg(
                        SErrorType::ConfigurationError,
                        &format!("Failed to build configuration, error: {}", err),
                    )
                })?
        }
    };

    config.try_deserialize::<Config>().map_err(|err| {
        SError::from_msg(
            SErrorType::ParsingError,
            &format!("Failed to deserialize Config, error: {}", err),
        )
    })
}
