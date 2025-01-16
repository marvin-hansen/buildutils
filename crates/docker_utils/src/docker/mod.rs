use crate::DockerError;
use std::process::Command;

mod setup;
mod start;
mod stop;

mod check_running;
mod dbg;
mod prune;
mod pull;
mod utils;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DockerUtil {
    dbg: bool,
}

impl DockerUtil {
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
    pub(crate) fn build(dbg: bool) -> Result<Self, DockerError> {

        let mut cmd = Command::new("docker");
        cmd.arg("info");

        match cmd.status(){
            Ok(_) => {
                if dbg {
                    println!("[DockerUtil]: Docker is running");
                }
            }
            Err(_) => {
                panic!("🚨Failed to connect to docker. Is Docker running?🚨")
            }
        }
        Ok(Self { dbg })
    }
}

impl Default for DockerUtil {
    fn default() -> Self {
        Self::new().expect("Failed to create DockerUtil")
    }
}
