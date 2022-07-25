mod convertors;
mod error_type;
mod error;
mod result;

pub mod prelude {
    pub use super::{error::SError, error_type::SErrorType, result::SResult};
}
