use crate::error::prelude::*;

#[allow(dead_code)]
impl SError {
    pub fn from_msg(kind: SErrorType, msg: &str) -> Self {
        SError { kind, message: msg.to_string() }
    }

    pub fn from_kind(kind: SErrorType) -> Self {
        let message = kind.to_string();
        SError { kind, message }
    }
}

impl std::convert::From<serde_json::Error> for SError {
    fn from(err: serde_json::Error) -> SError {
        let kind = SErrorType::InternalServerError;
        let message = format!(
            "(De)serialization failed; err: {:?}", err.to_string()
        );
        SError { message: message.to_string(), kind }
    }
}

impl std::convert::From<std::io::Error> for SError {
    fn from(err: std::io::Error) -> SError {
        let kind = SErrorType::InternalServerError;
        let message = format!(
            "I/O error: {:?}", err.to_string()
        );
        SError { message, kind }
    }
}

impl std::convert::From<log::SetLoggerError> for SError {
    fn from(err: log::SetLoggerError) -> SError {
        let kind = SErrorType::InternalServerError;
        let message = format!(
            "Error initializing logger: {:?}", err.to_string()
        );
        SError { message, kind }
    }
}
