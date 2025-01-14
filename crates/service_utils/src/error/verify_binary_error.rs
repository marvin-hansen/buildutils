use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum VerifyBinaryError {
    BinaryNotFound(String),
    BinaryWrongPlatform(String, String),
}

impl Error for VerifyBinaryError {}

impl Display for VerifyBinaryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BinaryNotFound(e) => {
                write!(f, "[VerifyBinaryError]: Binary not found: {e}")
            }
            Self::BinaryWrongPlatform(target, actual) => {
                write!(
                    f,
                    "[VerifyBinaryError]: Binary has wrong platform. \n
                    Binary platform should be {target}, but got {actual} "
                )
            }
        }
    }
}
