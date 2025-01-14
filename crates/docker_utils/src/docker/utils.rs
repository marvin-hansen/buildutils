use crate::{DockerError, DockerUtil};

use std::process::Command;

impl DockerUtil {
    /// Pulls a container image from a registry.
    ///
    /// This method pulls a container image from a specified registry using the `docker pull` command.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the `DockerUtil` object.
    /// * `container_id` - The ID of the container to start.
    /// * `image` - The container image with tag.
    /// * `platform` - Optional platform tag, such as linux/amd64.
    ///
    /// # Returns
    ///
    /// * `Result<(), DockerError>` - Returns `Ok(())` if the image is pulled successfully, or an `Err` containing the error if it fails.
    ///
    /// # Errors
    ///
    /// Returns a `DockerError` if there is an error pulling the container image.
    ///
    pub fn pull_container_image(
        &self,
        container_id: &str,
        image: &str,
        platform: Option<&str>,
    ) -> Result<(), DockerError> {
        // Example docker pull --platform linux/amd64  asia-northeast1-docker.pkg.dev/future-309012/image-repo/api_proxy:b422ae3
        self.dbg_print(&format!(
            "[pull_container_image]: Pull container image for: {container_id}."
        ));

        // construct initial command
        let mut cmd = Command::new("docker");
        cmd.arg("pull");

        if platform.is_some() {
            let p = platform.expect("Failed to unwrap Docker platform string");
            cmd.arg("--platform").arg(p);
        }

        // Add the image
        cmd.arg(image);

        self.dbg_print(&format!("[pull_container_image]: Pull command: {cmd:?}"));

        // Run the command & return error in case of failure
        match cmd.output() {
            Ok(out) => {
                if out.status.success() {
                    self.dbg_print(&format!(
                        "[pull_container_image]: success. Image Pulled {image}"
                    ));
                } else {
                    eprintln!(
                        "Error pulling container image {}: {}",
                        container_id,
                        String::from_utf8_lossy(&out.stderr)
                    );
                    return Err(DockerError::from(format!(
                        "Error starting container {}: {}",
                        container_id,
                        String::from_utf8_lossy(&out.stderr)
                    )));
                }

                Ok(())
            }
            Err(e) => {
                eprintln!();
                eprintln!("Error pulling container image {container_id}: {e}");
                eprintln!();
                panic!("")
            }
        }
    }

    /// Check if a container is starting.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container to check.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the container is starting, `Ok(false)` if the container is not starting, or `Err(DockerError)` if an error occurred.
    ///
    pub fn check_if_container_is_starting(&self, container_id: &str) -> Result<bool, DockerError> {
        self.dbg_print(&format!(
            "[check_if_container_is_starting]: Check container image for: {container_id}."
        ));

        // Example docker logs apiproxy-7777
        match Command::new("docker")
            .arg("logs")
            .arg(format!(" {container_id}"))
            .output()
        {
            Ok(out) => {
                self.dbg_print(&format!(
                    "[check_if_container_is_starting]: \n
                    success: {} \n
                    Output: {}",
                    out.status.success(),
                    String::from_utf8_lossy(out.stdout.as_slice()),
                ));

                if out.status.success() {
                    if out.stdout.is_empty() {
                        Ok(false)
                    } else {
                        Ok(true)
                    }
                } else {
                    Ok(false)
                }
            }
            Err(_) => Ok(false),
        }
    }

    /// Check if a container exists by its ID.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container to check.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the container exists, `Ok(false)` if the container does not exist, or `Err(DockerError)` if an error occurred.
    ///
    pub fn check_if_container_is_running(&self, container_id: &str) -> Result<bool, DockerError> {
        match self.get_running_container(container_id) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get the the name and of about a running container by its ID.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container to retrieve information about.
    ///
    /// # Returns
    ///
    /// Either returns the name and port of a container if its running, otherwise an `DockerError`.
    ///
    pub fn get_running_container(&self, container_id: &str) -> Result<(String, u16), DockerError> {
        self.dbg_print(&format!(
            "[get_running_container]: Check container image for: {container_id}."
        ));

        let mut cmd = Command::new("docker");
        cmd.arg("ps");
        cmd.arg("--filter=status=running");
        cmd.arg(format!("--filter=name={container_id}"));
        cmd.arg("--format={{.Names}}");

        self.dbg_print(&format!(
            "[get_running_container]: Run Docker command: {cmd:?}"
        ));

        let container = match cmd.output() {
            Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
            Err(e) => {
                return Err(DockerError::from(format!(
                    "[get_running_container]: Error getting container {container_id}: {e}"
                )));
            }
        };

        self.dbg_print(&format!(
            "[get_running_container]: Empty output: {}.",
            container.is_empty()
        ));

        if container.is_empty() {
            return Err(DockerError::from(format!(
                "[get_running_container]: Error no container found for ID: {container_id}",
            )));
        }

        let parts = container_id.split('-').collect::<Vec<&str>>();
        let port = parts
            .last()
            .expect("[get_running_container]: Failed to get container port")
            .trim()
            .parse::<u16>()
            .expect("[get_running_container]: Failed to convert container port from string to u16");

        Ok((container_id.to_string(), port))
    }

    /// Retrieves the image tag of a running container by its ID.
    ///
    /// This method takes a container ID as input and retrieves the image tag of the running container.
    /// It first checks if the container exists using the `check_if_container_exists` method.
    /// If the container exists, it executes the `docker ps` command with the `--filter` option to get the container image.
    /// The image tag is extracted from the output of the `docker ps` command.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the `DockerUtil` object.
    /// * `container_id` - The ID of the container for which the image tag is to be retrieved.
    ///
    /// # Returns
    ///
    /// * `Result<String, DockerError>` - The image tag of the running container as a string. Returns `Ok(tag)` if the image tag is retrieved successfully, or an `Err` containing the error if it fails.
    ///
    /// # Errors
    ///
    /// Returns a `DockerError` if the container does not exist or if there is an error executing the `docker ps` command.
    ///
    pub fn get_running_container_image_tag(
        &self,
        container_id: &str,
    ) -> Result<String, DockerError> {
        match self.check_if_container_is_running(container_id) {
            Ok(_) => {}
            Err(_) => {
                return Err(DockerError::from(format!(
                    "[get_running_container_image_tag]: Error no container found for ID: {container_id}",
                )));
            }
        }

        self.dbg_print("");
        let container_image = match Command::new("docker")
            .arg("ps")
            .arg(format!("--filter=name={container_id}"))
            .arg("--format={{.Image}}")
            .output()
        {
            Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
            Err(e) => {
                return Err(DockerError::from(format!(
                    "[get_container_image_tag]: Error getting container image for {container_id}: {e}"
                )));
            }
        };

        if container_image.is_empty() {
            return Err(DockerError::from(format!(
                "[get_container_image_tag]: Error no image found for container ID: {container_id}",
            )));
        }

        let parts = container_image.split(':').collect::<Vec<&str>>();
        let tag = parts
            .last()
            .expect("[get_container_image_tag]: Failed to get container tag")
            .trim()
            .to_owned();

        Ok(tag)
    }

    /// Checks if a running container uses a specific target tag.
    ///
    /// This method takes a container ID and a target tag as input and checks if the running container with the given ID uses the target tag.
    /// It first retrieves the image tag of the running container using the `get_running_container_image_tag` method.
    /// Then it compares the retrieved image tag with the target tag provided.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the `DockerUtil` object.
    /// * `container_id` - The ID of the container to check.
    /// * `target_tag` - The target tag to compare with the container's image tag.
    ///
    /// # Returns
    ///
    /// * `Result<bool, DockerError>` - Returns `Ok(true)` if the container uses the target tag, `Ok(false)` if it does not, or an [Err](cci:4:///Users/marvin/RustroverProjects/quant-engine/queng_utils/env_utils/src/ci/setup_containers.rs:98:0-118:0) containing the error.
    ///
    /// # Errors
    ///
    /// Returns a `DockerError` if there is an error getting the container image tag or if the comparison fails.
    ///
    pub fn check_if_running_container_uses_target_tag(
        &self,
        container_id: &str,
        target_tag: &str,
    ) -> Result<bool, DockerError> {
        match self.get_running_container_image_tag(container_id) {
            Ok(container_tag) => {
                Ok(container_tag.eq_ignore_ascii_case(target_tag))
            }
            Err(e) => {
                Err(DockerError::from(format!(
                    "[check_if_container_uses_target_tag]: Error getting container_tag for container ID: {container_id} {e}"
                )))
            }
        }
    }

    /// Prunes all stopped containers and their associated volumes and networks.
    ///
    /// This method executes the `docker system prune` command with the `--all` and `--force` options to remove all stopped containers, their associated volumes, and networks.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - A mutable reference to the `DockerUtil` object.
    ///
    /// # Returns
    ///
    /// * `Result<(), DockerError>` - Returns `Ok(())` if the containers are pruned successfully, or an `Err` containing the error if it fails.
    ///
    /// # Errors
    ///
    /// Returns a `DockerError` if there is an error executing the `docker system prune` command.
    ///
    pub fn prune_containers(&mut self) -> Result<(), DockerError> {
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
