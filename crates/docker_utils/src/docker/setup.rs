use crate::{ContainerConfig, DockerError, DockerUtil};

impl DockerUtil {
    /// Sets up a Docker container based on the provided configuration, handling existence checks and version management.
    ///
    /// # Arguments
    ///
    /// * `self` - Reference to the `DockerUtil` instance
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
    pub async fn setup_container(
        &self,
        container_config: &ContainerConfig<'_>,
    ) -> Result<(String, u16), DockerError> {
        let container_name = &container_config.container_name();
        let target_tag = container_config.tag();

        self.dbg_print(&format!(
            "Check if Container already exists: {container_name}"
        ));

        let exists = self
            .check_if_container_is_running(container_name)
            .unwrap_or_else(|_| {
                panic!("[get_running_container]:  container already exists: {container_name}")
            });
        self.dbg_print(&format!("Container {container_name} exists: {exists}"));

        if exists {
            self.dbg_print(&format!(
                "Check if running Container {container_name} uses target tag: {target_tag}",
            ));

            let container_current = self
                .check_if_running_container_uses_target_tag(container_name, target_tag)
                .unwrap_or_else(|_| panic!("[TestEnv/CI:setup_container]: Failed to check if container {container_name} use target tag: {target_tag}"));

            if container_current {
                self.dbg_print(&format!(
                    "Container {container_name} uses target tag: {container_current}"
                ));
            } else {
                self.dbg_print(&format!("Container uses DIFFERENT tag : {container_name}"));
                self.dbg_print(&format!("STOP running Container : {container_name}"));

                self.stop_container(container_name).unwrap_or_else(|_| {
                    panic!(
                        "[TestEnv/CI:setup_container]: Failed to check stop container {container_name} "
                    )
                });
            }
        }

        let (container_name, container_port) = self
            .get_or_start_container_config(container_config)
            .unwrap_or_else(|_| {
                panic!("[TestEnv/CI:setup_container]: Failed to setup container: {container_name}")
            });

        if exists {
            self.dbg_print(&format!(
                "Reuse Container {container_name} with target tag {target_tag}"
            ));
        } else {
            self.dbg_print(&format!(
                "Start container {container_name} with target tag {target_tag}"
            ));
        }

        Ok((container_name, container_port))
    }
}
