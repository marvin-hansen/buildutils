use crate::service::verify::verify_binary_exists;
use crate::{ServiceUtil, ServiceUtilError};

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
        if dbg {
            println!("[ServiceUtil]: OK! All binaries exists in path: {root_path}");
        }

        Ok(ServiceUtil {
            dbg,
            root_path,
            binaries,
        })
    }
}
