use crate::ServiceUtil;

impl ServiceUtil {
    pub fn root_path(&self) -> &'static str {
        self.root_path
    }

    pub fn binaries(&self) -> &Vec<&'static str> {
        &self.binaries
    }
}
