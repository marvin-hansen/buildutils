use common_container::ContainerConfig;
use docker_utils::DockerUtil;
use std::fmt::Error;

mod postgres_config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");
    let container_config = postgres_config::postgres_db_container_config();
    let container_id = "postgres-5432";

    start_container(&docker_util, &container_config, container_id, 5432).await;

    container_exists(&docker_util, container_id).await;

    stop_container(&docker_util, container_id).await;

    container_deleted(&docker_util, container_id).await;

    Ok(())
}

async fn start_container(
    docker_util: &DockerUtil,
    container_config: &ContainerConfig<'static>,
    expected_container_id: &str,
    expected_container_port: u16,
) {
    println!("test_start_container");

    let res = docker_util.get_or_start_container_config(container_config);
    assert!(res.is_ok());

    let (container_id, container_port) = res.unwrap();
    assert_eq!(container_id, expected_container_id);
    assert_eq!(container_port, expected_container_port);
    println!("container started: {}", container_id);
}

async fn container_exists(docker_util: &DockerUtil, container_id: &str) {
    println!("test_container_exists");

    let res = docker_util.check_if_container_is_running(container_id);
    assert!(res.is_ok());
    assert!(res.unwrap());
    println!("container exists: {}", container_id);
}

async fn stop_container(docker_util: &DockerUtil, container_id: &str) {
    println!("test_stop_container");

    let res = docker_util.stop_container(container_id);
    assert!(res.is_ok());

    let res = docker_util.check_if_container_is_running(container_id);

    assert!(res.is_ok());
    assert!(!res.unwrap());
    println!("container stopped: {}", container_id);
}

async fn container_deleted(docker_util: &DockerUtil, container_id: &str) {
    println!("test_container_deleted");

    let res = docker_util.check_if_container_is_running(container_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());
    println!("container deleted: {}", container_id);
}
