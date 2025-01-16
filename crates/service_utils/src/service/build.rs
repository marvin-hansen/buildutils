use crate::service::verify::verify_binary_exists;
use crate::{ServiceUtil, ServiceUtilError};
use std::collections::HashMap;
use std::sync::RwLock;

impl ServiceUtil {
    pub(crate) async fn build(
        dbg: bool,
        root_path: &'static str,
        binaries: Vec<&'static str>,
    ) -> Result<Self, ServiceUtilError> {
        if dbg {
            println!("[ServiceUtil]: Debug mode enabled");
        }

        verify_binary_exists(dbg, root_path, &binaries)?;

        Ok(ServiceUtil {
            dbg,
            root_path,
            binaries,
            binary_handlers: RwLock::new(HashMap::with_capacity(25)),
        })
    }
}
