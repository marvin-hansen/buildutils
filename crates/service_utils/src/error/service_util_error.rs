use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ServiceUtilError {
    ServiceStartFailed(String),
    ServiceHealthcheckFailed(String),
    ServiceStopFailed(String),
    ServiceNotSupported(String),
    ServiceNotRunning(String),
    UnknownError(String),
}

impl Error for ServiceUtilError {}

impl fmt::Display for ServiceUtilError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceStartFailed(e) => {
                write!(f, "[ServiceUtilError]: Service start failed: {e}")
            }
            Self::ServiceHealthcheckFailed(e) => {
                write!(f, "[ServiceUtilError]: Service healthcheck failed: {e}")
            }
            Self::ServiceStopFailed(e) => {
                write!(f, "[ServiceUtilError]: Service stop failed: {e}")
            }
            Self::ServiceNotSupported(e) => {
                write!(f, "[ServiceUtilError]: Service not supported: {e}")
            }
            Self::ServiceNotRunning(e) => {
                write!(f, "[ServiceUtilError]: Service not running: {e}")
            }
            Self::UnknownError(e) => {
                write!(f, "[ServiceUtilError]: Unknown error: {e}")
            }
        }
    }
}
