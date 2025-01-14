use crate::DockerError;
use std::process::Command;

mod setup;
mod start;
mod stop;

mod utils;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DockerUtil {
    dbg: bool,
}

impl DockerUtil {
    /// Create a new instance of the `DockerUtil` struct.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the `DockerUtil` struct with default values.
    ///
    pub fn new() -> Result<Self, DockerError> {
        Self::build(false)
    }

    /// Create a new instance of the `DockerUtil` struct with debug mode enabled.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a new instance of the `DockerUtil` struct with debug mode enabled, or a `DockerError` if an error occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use docker_utils::DockerUtil;
    ///
    /// // Requires running Docker. Start Docker and uncomment.
    /// //let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil with debug mode");
    /// ```
    pub fn with_debug() -> Result<Self, DockerError> {
        Self::build(true)
    }

    /// Build a new instance of the `DockerUtil` struct with the given debug flag.
    ///
    /// # Arguments
    ///
    /// * `dbg` - A boolean flag indicating whether to enable debug mode.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a new instance of the `DockerUtil` struct if successful, or a `DockerError` if an error occurred.
    ///
    fn build(dbg: bool) -> Result<Self, DockerError> {
        match Command::new("docker").arg("-v").spawn() {
            Ok(_) => Ok(Self { dbg }),
            Err(e) => Err(DockerError::from(format!(
                "Error connecting to Docker. Is Docker running? Error: {e}"
            ))),
        }
    }
}

impl Default for DockerUtil {
    fn default() -> Self {
        Self::new().expect("Failed to create DockerUtil")
    }
}

impl DockerUtil {
    pub(crate) fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[DockerUtil]: {s}");
        }
    }
}
