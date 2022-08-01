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
