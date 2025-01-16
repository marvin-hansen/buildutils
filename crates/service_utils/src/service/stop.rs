use crate::{ServiceUtil, ServiceUtilError};

impl ServiceUtil {
    pub(crate) async fn stop(&self, program: &str) -> Result<(), ServiceUtilError> {
        let handlers = &mut self
            .binary_handlers
            .write()
            .expect("Failed to obtain a write lock to binary handlers");

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
}
