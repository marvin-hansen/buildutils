use crate::{ServiceUtil, ServiceUtilError};

impl ServiceUtil {
    pub(crate) async fn stop_all(&self) -> Result<(), ServiceUtilError> {
        let handlers = &mut self
            .binary_handlers
            .write()
            .expect("Failed to obtain a write lock to binary handlers");

        // Check if there are any handlers; if not, return
        if handlers.is_empty() {
            return Ok(());
        };

        // Abort all handlers and clear the map
        for (_, handler) in handlers.drain() {
            handler.abort();
        }

        Ok(())
    }
}
