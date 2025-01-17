use docker_utils::{ContainerConfig, WaitStrategy};

pub(crate) fn postgres_db_container_config() -> ContainerConfig<'static> {
    ContainerConfig::builder()
        .name("postgres")
        .image("postgres")
        .tag("17-alpine3.20")
        .url("0.0.0.0")
        .connection_port(5432)
        .additional_env_vars(&["POSTGRES_PASSWORD=postgres"])
        .reuse_container(true)
        .keep_configuration(true)
        .wait_strategy(WaitStrategy::WaitUntilConsoleOutputContains(
            "PostgreSQL init process complete; ready for start up.".to_string(),
            15,
        ))
        .build()
}
