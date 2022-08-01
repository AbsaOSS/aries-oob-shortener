use crate::error::prelude::*;
use aws_sdk_s3::types::SdkError;

impl std::convert::From<serde_json::Error> for SError {
    fn from(err: serde_json::Error) -> SError {
        let kind = SErrorType::InternalServerError;
        let message = format!("(De)serialization failed; err: {:?}", err.to_string());
        SError {
            message,
            kind,
        }
    }
}

impl std::convert::From<std::io::Error> for SError {
    fn from(err: std::io::Error) -> SError {
        let kind = SErrorType::InternalServerError;
        let message = format!("I/O error: {:?}", err.to_string());
        SError { message, kind }
    }
}

impl std::convert::From<log::SetLoggerError> for SError {
    fn from(err: log::SetLoggerError) -> SError {
        let kind = SErrorType::InternalServerError;
        let message = format!("Error initializing logger: {:?}", err.to_string());
        SError { message, kind }
    }
}

impl<E: std::error::Error> std::convert::From<SdkError<E>> for SError {
    fn from(err: SdkError<E>) -> SError {
        let kind = SErrorType::InternalServerError;
        let message = format!("AWS SDK S3 Error: {}", err);
        SError { message, kind }
    }
}

impl std::convert::From<std::str::Utf8Error> for SError {
    fn from(err: std::str::Utf8Error) -> SError {
        let kind = SErrorType::InternalServerError;
        let message = format!("UTF-8 Error: {}", err);
        SError { message, kind }
    }
}

impl std::convert::From<reqwest::Error> for SError {
    fn from(err: reqwest::Error) -> SError {
        let kind = SErrorType::InternalServerError;
        let message = format!("Reqwest Error: {}", err);
        SError { message, kind }
    }
}
