mod convertors;
mod error;
mod error_type;
mod result;

pub mod prelude {
    pub use super::{error::SError, error_type::SErrorType, result::SResult};
}
