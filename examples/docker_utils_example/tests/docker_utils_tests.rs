use docker_utils::*;

pub fn postgres_db_container_config() -> ContainerConfig<'static> {
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

#[test]
fn test_docker_utils() {
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");
    let container_config = postgres_db_container_config();
    let container_id = "postgres-5432";
    let expected_container_id = container_id;
    let expected_container_port = 5432;

    println!("start_container");
    let res = docker_util.get_or_start_container(&container_config);
    assert!(res.is_ok());

    let (container_id, container_port) = res.unwrap();
    assert_eq!(container_id, expected_container_id);
    assert_eq!(container_port, expected_container_port);

    println!("check_if_container_is_running");

    let res = docker_util.check_if_container_is_running(&container_id);
    assert!(res.is_ok());
    assert!(res.unwrap());
    println!("container is running: {}", &container_id);

    println!("test_stop_container");
    let delete_container = true;
    let res = docker_util.stop_container(&container_id, delete_container);
    assert!(res.is_ok());

    println!("test_container_deleted");
    let res = docker_util.check_if_container_is_running(&container_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());
    println!("container deleted: {}", &container_id);
}
