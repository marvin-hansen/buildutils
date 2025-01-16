use crate::{ContainerConfig, DockerError, DockerUtil};

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
    pub fn with_debug() -> Result<Self, DockerError> {
        Self::build(true)
    }

    /// Sets up a Docker container based on the provided configuration, handling existence checks and version management.
    ///
    /// # Arguments
    ///
    /// * `container_config` - Reference to a `ContainerConfig` containing the container configuration:
    ///   - Container name
    ///   - Image tag
    ///   - Other container-specific settings
    ///
    /// # Returns
    ///
    /// Returns a `Result<(String, u16), DockerError>`:
    /// * `Ok((container_name, port))` - A tuple containing:
    ///   - `container_name`: String - The name of the running container
    ///   - `port`: u16 - The exposed port number of the container
    /// * `Err(DockerError)` - If any Docker operation fails
    ///
    /// # Errors
    ///
    /// Returns a `DockerError` if:
    /// * Container existence check fails
    /// * Container tag verification fails
    /// * Container stop operation fails (when tag mismatch)
    /// * Container start operation fails
    /// * Port mapping operation fails
    /// * Docker API communication fails
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// * Container existence check critically fails
    /// * Tag verification critically fails
    /// * Container stop operation critically fails
    /// * Container setup critically fails
    ///
    /// # Implementation Notes
    ///
    /// This function performs the following steps:
    /// 1. Checks if a container with the given name already exists
    /// 2. If exists, verifies if it uses the target tag
    /// 3. If tag mismatch, stops the existing container
    /// 4. Creates or reuses the container with correct configuration
    /// 5. Returns the container name and exposed port
    ///
    pub fn setup_container(
        &self,
        container_config: &ContainerConfig<'_>,
    ) -> Result<(String, u16), DockerError> {
        self.setup(container_config)
    }

    /// Gets an existing container or starts a new one with the specified configuration
    ///
    /// # Arguments
    ///
    /// * `container_config` - The configuration of the container.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing the container name and port if successful,
    /// or a `DockerError` if an error occurs.
    ///
    pub fn get_or_start_container(
        &self,
        container_config: &ContainerConfig,
    ) -> Result<(String, u16), DockerError> {
        self.get_or_start(container_config)
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
        self.check_running(container_id)
    }

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
    pub fn stop_container(&self, container_id: &str) -> Result<(), DockerError> {
        self.stop(container_id)
    }

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
        self.pull(container_id, image, platform)
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
    pub fn prune_all_containers(&mut self) -> Result<(), DockerError> {
        self.prune()
    }
}
