use crate::{ContainerConfig, DockerError, DockerUtil};

impl DockerUtil {

    pub(crate) fn setup(
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
            .get_or_start(container_config)
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
