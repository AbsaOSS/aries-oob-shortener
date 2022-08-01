mod convertors;
mod error_struct;
mod error_type;
mod result;

pub mod prelude {
    pub use super::{error_struct::SError, error_type::SErrorType, result::SResult};
}
