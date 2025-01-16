use crate::ServiceUtil;
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::task::JoinHandle;

impl ServiceUtil {
    pub fn root_path(&self) -> &'static str {
        self.root_path
    }

    pub fn binaries(&self) -> &Vec<&'static str> {
        &self.binaries
    }

    pub fn binary_handlers(&self) -> &RwLock<HashMap<String, JoinHandle<()>>> {
        &self.binary_handlers
    }
}
