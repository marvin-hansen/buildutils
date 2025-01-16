use crate::{ServiceUtil, ServiceUtilError};

impl ServiceUtil {
    pub(crate) async fn stop(&self, program: &str) -> Result<(), ServiceUtilError> {
        let handlers = &mut self
            .binary_handlers
            .write()
            .expect("Failed to write to binary handlers");

        if handlers.is_empty() {
            return Ok(());
        }

        if handlers.contains_key(program) {
            let handler = handlers.get(program).expect("Failed to get handler");
            handler.abort();
            handlers.remove(program);
        }

        Ok(())
    }

    pub(crate) async fn stop_all(&self) -> Result<(), ServiceUtilError> {
        let handlers = &mut self
            .binary_handlers
            .write()
            .expect("Failed to write to binary handlers");

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
