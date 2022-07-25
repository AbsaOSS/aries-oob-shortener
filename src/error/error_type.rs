use serde::{Serialize, Deserialize};

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
    LockError
}

impl SErrorType {
    pub fn to_string(&self) -> String {
        match self {
            SErrorType::InternalServerError => String::from("Internal Server Error"),
            SErrorType::RequestNotAcceptedError => String::from("Request Not Accepted"),
            SErrorType::NotFoundError => String::from("Not Found"),
            SErrorType::InvalidJson => String::from("Invalid JSON"),
            SErrorType::ProtocolError => String::from("Protocol Error"),
            SErrorType::ParsingError => String::from("Parsing Error"),
            SErrorType::SerializationError => String::from("Serialization Error"),
            SErrorType::IOError => String::from("IO Error"),
            SErrorType::RedisError => String::from("Redis Error"),
            SErrorType::ConfigurationError => String::from("Configuration Error"),
            SErrorType::LockError => String::from("Lock Error"),
        }
    }
}
