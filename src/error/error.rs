use crate::error::prelude::*;

use std::fmt::Display;
use actix_web::{
    HttpResponseBuilder, error, http::StatusCode, HttpResponse,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SError {
    pub message: String,
    pub kind: SErrorType
}

impl error::ResponseError for SError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self.kind {
            SErrorType::RequestNotAcceptedError |
                SErrorType::InvalidJson => StatusCode::NOT_ACCEPTABLE,
            SErrorType::NotFoundError => StatusCode::NOT_FOUND,
            SErrorType::InternalServerError
                | SErrorType::ProtocolError
                | SErrorType::ParsingError
                | SErrorType::IOError 
                | SErrorType::RedisError
                | SErrorType::ConfigurationError
                | SErrorType::LockError
                | SErrorType::SerializationError
                => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Display for SError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        f.write_str(&self.kind.to_string())
    }
}
