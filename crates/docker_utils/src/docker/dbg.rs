use crate::DockerUtil;

impl DockerUtil {
    pub(crate) fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[DockerUtil]: {s}");
        }
    }
}
