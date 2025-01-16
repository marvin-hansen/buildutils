use crate::DockerError;
use std::process::{exit, Command};

mod setup;
mod start;
mod stop;

mod build;
mod check_running;
mod dbg;
mod default;
mod prune;
mod pull;
mod utils;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DockerUtil {
    dbg: bool,
}

impl DockerUtil {
    /// Builds a new DockerUtil instance.
    ///
    /// Checks if Docker is running before returning the instance.
    /// If Docker is not running, it prints an error message and exits the program.
    pub(crate) fn build(dbg: bool) -> Result<Self, DockerError> {
        let mut cmd = Command::new("docker");
        cmd.arg("ps");

        if cmd.status().unwrap().success() {
            if dbg {
                println!("[DockerUtil]: Docker is running");
            }
        } else {
            println!();
            println!(" ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️");
            println!("🚨🚨🚨 DockerUtil: Mayday Mayday 🚨🚨🚨");
            println!(" ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️");
            println!("🚨🚨🚨 Cannot connect to Docker  🚨🚨🚨");
            println!("🚨🚨🚨 Is Docker up & running?   🚨🚨🚨");
            println!(" ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️");
            println!("🚨🚨🚨 Help guide & documentation 🚨🚨🚨");
            println!("Install Docker: https://docs.docker.com/engine/install/");
            println!("Install Obstack: https://docs.orbstack.dev/quick-start");
            exit(42)
        }

        Ok(Self { dbg })
    }
}
