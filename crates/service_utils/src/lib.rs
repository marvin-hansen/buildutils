mod error;
mod types;
mod verify;
mod service;
mod api;

// Re-exports
pub use error::*;
use std::fmt::{Display, Formatter};
pub use types::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ServiceUtil {
    dbg: bool,
    root_path: &'static str,
    binaries: Vec<&'static str>,
}

impl ServiceUtil {
    pub async fn new(
        root_path: &'static str,
        binaries: Vec<&'static str>,
    ) -> Result<Self, ServiceUtilError> {
        Self::build(false, root_path, binaries).await
    }

    pub async fn with_debug(
        root_path: &'static str,
        binaries: Vec<&'static str>,
    ) -> Result<Self, ServiceUtilError> {
        Self::build(true, root_path, binaries).await
    }

    async fn build(
        dbg: bool,
        root_path: &'static str,
        binaries: Vec<&'static str>,
    ) -> Result<Self, ServiceUtilError> {
        if dbg {
            println!("[ServiceUtil]: Debug mode enabled");
        }

        if dbg {
            println!("[ServiceUtil]: Verify all binaries in path: {root_path}");
        }
        verify::verify_binary_exists(dbg, root_path, &binaries)?;

        Ok(ServiceUtil {
            dbg,
            root_path,
            binaries,
        })
    }
}

impl ServiceUtil {
    pub fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ServiceUtil]: {s}");
        }
    }
}

impl Display for ServiceUtil {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUtil {{ debug mode: {} }}", &self.dbg)
    }
}
