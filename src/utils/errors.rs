use std::fmt;

#[derive(Debug)]
pub enum KubeMergeError {
    NoContent(String),
    FileNotFound(String),
    FileReadError(String),
    ParseError(String),
    WriteError(String),
    UserCancelled(String),
    InsufficientFiles(String),
    SomeOtherError,
}

impl fmt::Display for KubeMergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KubeMergeError::ParseError(msg) => write!(f, "{}", msg),
            _ => write!(f, "An unknown KubeMergeError occurred"),
        }
    }
}
