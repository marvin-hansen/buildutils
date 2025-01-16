use std::process::Command;
use crate::{DockerError, DockerUtil};

impl DockerUtil {

    pub(crate) fn prune(&mut self) -> Result<(), DockerError> {
        match Command::new("docker")
            .arg("system")
            .arg("prune")
            .arg("--all")
            .arg("--force")
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(DockerError::from(format!("Error pruning containers: {e}"))),
        }
    }
}
