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

use crate::error::prelude::*;

use actix_web::{error, http::StatusCode, HttpResponse, HttpResponseBuilder};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SError {
    pub message: String,
    pub kind: SErrorType,
}

#[allow(dead_code)]
impl SError {
    pub fn from_msg(kind: SErrorType, msg: &str) -> Self {
        SError {
            kind,
            message: msg.to_string(),
        }
    }

    pub fn from_kind(kind: SErrorType) -> Self {
        let message = kind.to_string();
        SError { kind, message }
    }
}

impl error::ResponseError for SError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self.kind {
            SErrorType::RequestNotAcceptedError | SErrorType::InvalidJson => {
                StatusCode::NOT_ACCEPTABLE
            }
            SErrorType::NotFoundError => StatusCode::NOT_FOUND,
            SErrorType::InternalServerError
            | SErrorType::ProtocolError
            | SErrorType::ParsingError
            | SErrorType::IOError
            | SErrorType::RedisError
            | SErrorType::ConfigurationError
            | SErrorType::LockError
            | SErrorType::SerializationError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Display for SError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        f.write_str(&self.kind.to_string())
    }
}
