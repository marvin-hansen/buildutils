use docker_utils::{ContainerConfig, DockerUtil, WaitStrategy};

fn get_test_container_config() -> ContainerConfig<'static> {
    ContainerConfig::new(
        "nginx",
        "nginx",
        "1.27.0",
        "0.0.0.0",
        80,
        None,
        None,
        None,
        true, // Keep the container running for re-use
        true, // Keep the same container config across all env. setups.
        WaitStrategy::WaitUntilConsoleOutputContains(
            "Configuration complete; ready for start up".to_string(),
            15,
        ),
    )
}

// Always test DockerUtil with a container that is NOT used in any other integration tests.
// Otherwise, on local runs, this test suite may stop containers that are used in other tests and hence
// lead to random flaky tests and / or incorrect test results.

#[tokio::test]
async fn test_docker_util() {
    let docker_util = DockerUtil::with_debug().expect("Failed to create DockerUtil");
    let container_config = get_test_container_config();
    let container_id = "nginx-80";

    test_pull(&docker_util, &container_config, container_id).await;

    test_start_container(&docker_util, &container_config, container_id, 80).await;

    test_container_exists(&docker_util, container_id).await;

    test_stop_container(&docker_util, container_id).await;

    test_container_stopped(&docker_util, container_id).await;
}

async fn test_pull(
    docker_util: &DockerUtil,
    container_config: &ContainerConfig<'static>,
    container_id: &str,
) {
    println!("test_pull");
    let image = container_config.image();
    let tag = container_config.tag();
    let image = &format!("{image}:{tag}");
    let platform = container_config.platform();

    let res = docker_util.pull_container_image(container_id, image, platform);
    assert!(res.is_ok());
}

async fn test_start_container(
    docker_util: &DockerUtil,
    container_config: &ContainerConfig<'static>,
    expected_container_id: &str,
    expected_container_port: u16,
) {
    println!("test_start_container");

    let res = docker_util.get_or_start_container(container_config);
    assert!(res.is_ok());

    let (container_id, container_port) = res.unwrap();
    assert_eq!(container_id, expected_container_id);
    assert_eq!(container_port, expected_container_port);
}

async fn test_container_exists(docker_util: &DockerUtil, container_id: &str) {
    println!("test_container_exists");

    let res = docker_util.check_if_container_is_running(container_id);
    assert!(res.is_ok());
}

async fn test_stop_container(docker_util: &DockerUtil, container_id: &str) {
    println!("test_stop_container");

    let delete_container = true;
    let res = docker_util.stop_container(container_id, delete_container);
    assert!(res.is_ok());

    let res = docker_util.check_if_container_is_running(container_id);

    assert!(res.is_ok());
    assert!(!res.unwrap());
}

async fn test_container_stopped(docker_util: &DockerUtil, container_id: &str) {
    println!("test_container_stopped");

    let res = docker_util.check_if_container_is_running(container_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());
}
