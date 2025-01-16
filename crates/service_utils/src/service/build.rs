use crate::{verify, ServiceUtil, ServiceUtilError};

impl ServiceUtil {
    pub(crate) async fn build(
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
