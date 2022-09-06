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

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SErrorType {
    InternalServerError,
    RequestNotAcceptedError,
    NotFoundError,
    InvalidJson,
    ProtocolError,
    ParsingError,
    SerializationError,
    IOError,
    RedisError,
    ConfigurationError,
    LockError,
}

impl fmt::Display for SErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SErrorType::InternalServerError => write!(f, "Internal Server Error"),
            SErrorType::RequestNotAcceptedError => write!(f, "Request Not Accepted"),
            SErrorType::NotFoundError => write!(f, "Not Found"),
            SErrorType::InvalidJson => write!(f, "Invalid JSON"),
            SErrorType::ProtocolError => write!(f, "Protocol Error"),
            SErrorType::ParsingError => write!(f, "Parsing Error"),
            SErrorType::SerializationError => write!(f, "Serialization Error"),
            SErrorType::IOError => write!(f, "IO Error"),
            SErrorType::RedisError => write!(f, "Redis Error"),
            SErrorType::ConfigurationError => write!(f, "Configuration Error"),
            SErrorType::LockError => write!(f, "Lock Error"),
        }
    }
}
