use crate::ServiceUtil;

impl ServiceUtil {
    pub fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ServiceUtil]: {s}");
        }
    }
}
