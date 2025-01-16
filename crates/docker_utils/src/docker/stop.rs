use crate::{DockerError, DockerUtil};

use std::process::Command;

impl DockerUtil {
    /// Stop a container
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container to stop.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the container was successfully stopped, or `Err(DockerError)` if an error occurred.
    ///
    pub(crate) fn stop(&self, container_id: &str) -> Result<(), DockerError> {
        self.dbg_print("[stop_container]: Check if container exists.");
        let exists = self
            .check_if_container_is_running(container_id)
            .expect("Failed to check if container exists");

        if !exists {
            return Err(DockerError::from(format!(
                "Container doesn't exists: {container_id}"
            )));
        }

        if exists {
            self.dbg_print("[stop_container]: Container exists. Stopping it.");
            // Example: docker rm -f nginx-80
            // https://stackoverflow.com/questions/35122773/single-command-to-stop-and-remove-docker-container
            return match Command::new("docker")
                .arg("rm")
                .arg("-f")
                .arg(container_id)
                .status()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(DockerError::from(format!(
                    "[stop_container]: Error stopping container {container_id}: {e}"
                ))),
            };
        }

        Ok(())
    }
}
