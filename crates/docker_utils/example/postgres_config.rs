use docker_utils::{ContainerConfig, WaitStrategy};

pub(crate) fn postgres_db_container_config() -> ContainerConfig<'static> {
    // Ensure name matches exactly the generated name by the DockerUtil
    ContainerConfig::new(
        "postgres",
        "postgres",
        "17-alpine3.20",
        "0.0.0.0",
        5432,
        None, // No additional ports. If you need to open extra ports, add   Some(&[8123, 8124]),
        Some(&["POSTGRES_PASSWORD=postgres"]),
        None, // No specific platform. Works by default. If you need a specific platform,
        // add Some("linux/amd64") for intel or Some("linux/arm64") for arm.
        true, // Keep the container running for re-use. Set to false to always start with a new container.
        true, // Keep the same container config.
        WaitStrategy::WaitUntilConsoleOutputContains(
            "PostgreSQL init process complete; ready for start up.".to_string(),
            15,
        ),
    )
}
