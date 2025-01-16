use crate::DockerUtil;

impl Default for DockerUtil {
    fn default() -> Self {
        Self::new().expect("Failed to create DockerUtil")
    }
}
