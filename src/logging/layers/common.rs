use std::fmt;

pub const NAME: &str = "name";
pub const LEVEL: &str = "level";
pub const MESSAGE: &str = "message";
pub const MODULE: &str = "module";
pub const TARGET: &str = "target";
pub const FILENAME: &str = "filename";
pub const TIMESTAMP: &str = "timestamp";

pub const RESERVED_FIELDS: [&str; 7] =
    [NAME, LEVEL, MESSAGE, MODULE, TARGET, FILENAME, TIMESTAMP];

#[derive(Clone, Debug)]
pub enum Type {
    EnterSpan,
    ExitSpan,
    Event,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Type::EnterSpan => "START",
            Type::ExitSpan => "END",
            Type::Event => "EVENT",
        };
        write!(f, "{}", repr)
    }
}
