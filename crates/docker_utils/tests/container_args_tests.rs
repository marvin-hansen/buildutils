/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

//! Verifies the arguments the crate hands to `docker run`, by inspecting the container that
//! comes out the other end. Ports used here are exclusive to this file so that it cannot
//! interfere with the other integration tests running alongside it.

use docker_utils::{ContainerConfig, DockerUtil, WaitStrategy};
use std::process::Command;

/// Reads a single Go template field from `docker inspect`.
fn inspect(container_id: &str, format: &str) -> String {
    let out = Command::new("docker")
        .arg("inspect")
        .arg(format!("--format={format}"))
        .arg(container_id)
        .output()
        .expect("Failed to run docker inspect");

    assert!(
        out.status.success(),
        "docker inspect failed for {container_id}: {}",
        String::from_utf8_lossy(&out.stderr).trim()
    );

    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn remove_container(container_id: &str) {
    Command::new("docker")
        .args(["rm", "-f", container_id])
        .output()
        .expect("Failed to remove container");
}

/// Every environment variable has to reach the container.
///
/// Docker reads a single value per `-e` flag, so sharing one flag across variables turned
/// the second one into the image name and the run failed outright.
#[test]
fn test_every_env_var_reaches_the_container() {
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");
    let name = "docker-utils-envvars";
    let port = 6391;
    let container_id = format!("{name}-{port}");

    let container_config = ContainerConfig::builder()
        .name(name)
        .image("redis")
        .tag("7-alpine")
        .url("0.0.0.0")
        .connection_port(port)
        .additional_env_vars(&["FIRST_VAR=one", "SECOND_VAR=two", "THIRD_VAR=three"])
        .reuse_container(false)
        .keep_configuration(false)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let res = docker_util.get_or_start_container(&container_config);
    assert!(res.is_ok(), "Failed to start container: {res:?}");

    let env = inspect(&container_id, "{{.Config.Env}}");

    remove_container(&container_id);

    for expected in ["FIRST_VAR=one", "SECOND_VAR=two", "THIRD_VAR=three"] {
        assert!(
            env.contains(expected),
            "{expected} must reach the container, got: {env}"
        );
    }
}

/// Every additional port has to be published alongside the connection port.
#[test]
fn test_additional_ports_are_published() {
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");
    let name = "docker-utils-ports";
    let port = 6392;
    let container_id = format!("{name}-{port}");

    let container_config = ContainerConfig::builder()
        .name(name)
        .image("redis")
        .tag("7-alpine")
        .url("0.0.0.0")
        .connection_port(port)
        .additional_ports(&[6393, 6394])
        .reuse_container(false)
        .keep_configuration(false)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let res = docker_util.get_or_start_container(&container_config);
    assert!(res.is_ok(), "Failed to start container: {res:?}");

    let bindings = inspect(&container_id, "{{.HostConfig.PortBindings}}");

    remove_container(&container_id);

    for expected in ["6392", "6393", "6394"] {
        assert!(
            bindings.contains(expected),
            "Port {expected} must be published, got: {bindings}"
        );
    }
}

/// A zero port is rejected rather than handed to Docker.
#[test]
fn test_zero_additional_port_is_rejected() {
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

    let container_config = ContainerConfig::builder()
        .name("docker-utils-zeroport")
        .image("redis")
        .tag("7-alpine")
        .url("0.0.0.0")
        .connection_port(6395)
        .additional_ports(&[0])
        .reuse_container(false)
        .keep_configuration(false)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let err = docker_util
        .get_or_start_container(&container_config)
        .expect_err("A zero port must be rejected");

    assert!(
        err.to_string().contains("Port cannot be 0"),
        "Unexpected error: {err}"
    );
}

/// A zero port is rejected on the host network too, where ports are not published at all.
///
/// Skipping the port loop in host network mode would silently accept an invalid config.
#[test]
fn test_zero_additional_port_is_rejected_on_the_host_network() {
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

    let container_config = ContainerConfig::builder()
        .name("docker-utils-zeroport-host")
        .image("redis")
        .tag("7-alpine")
        .url("0.0.0.0")
        .connection_port(6396)
        .additional_ports(&[0])
        .host_network(true)
        .reuse_container(false)
        .keep_configuration(false)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let err = docker_util
        .get_or_start_container(&container_config)
        .expect_err("A zero port must be rejected on the host network as well");

    assert!(
        err.to_string().contains("Port cannot be 0"),
        "Unexpected error: {err}"
    );
}

/// An image that cannot be pulled must be reported, not waited on.
#[test]
fn test_unknown_image_is_reported() {
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

    let container_config = ContainerConfig::builder()
        .name("docker-utils-unknown-image")
        .image("docker-utils-no-such-image-exists")
        .tag("0.0.0")
        .url("0.0.0.0")
        .connection_port(6397)
        .reuse_container(false)
        .keep_configuration(false)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let res = docker_util.get_or_start_container(&container_config);

    assert!(
        res.is_err(),
        "An unknown image must be reported as an error"
    );
}
