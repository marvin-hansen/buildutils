/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use docker_utils::*;

/// Creates a sample `ContainerConfig` for testing purposes.
fn sample_container_config() -> ContainerConfig<'static> {
    ContainerConfig::new(
        "test_container",
        "test_image",
        "latest",
        "0.0.0.0",
        8080,
        Some(&[8081, 8082]),
        Some(&["ENV_VAR=VALUE", "DEBUG=true"]),
        Some(&["/tmp/host:/tmp/container:ro"]),
        Some("linux/amd64"),
        false, // Publish ports rather than using the host network
        true,
        false,
        WaitStrategy::default(), // NoWait is the default wait strategy
    )
}

#[test]
fn test_builder() {
    let config = ContainerConfig::builder()
        .name("test_container")
        .image("test_image")
        .tag("latest")
        .url("0.0.0.0")
        .connection_port(8080)
        .reuse_container(true)
        .keep_configuration(true)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    // assert
    assert_eq!(config.name(), "test_container");
    assert_eq!(config.container_image(), "test_image:latest");
    assert_eq!(config.container_name(), "test_container-8080");
    assert_eq!(config.url(), "0.0.0.0");
    assert_eq!(config.connection_port(), 8080);
    assert_eq!(config.additional_ports(), None);
    assert_eq!(config.additional_env_vars(), None);
    assert_eq!(config.platform(), None);
    assert!(config.reuse_container());
    assert!(config.keep_configuration());
    // host_network is optional in the builder and defaults to publishing ports.
    assert!(!config.host_network());
    assert_eq!(config.wait_strategy(), &WaitStrategy::NoWait);
}

#[test]
fn test_builder_host_network() {
    let config = ContainerConfig::builder()
        .name("test_container")
        .image("test_image")
        .tag("latest")
        .url("0.0.0.0")
        .connection_port(8080)
        .reuse_container(true)
        .keep_configuration(true)
        .host_network(true)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    assert!(config.host_network());
    // Everything else is unaffected by the network mode.
    assert_eq!(config.name(), "test_container");
    assert_eq!(config.connection_port(), 8080);
}

#[test]
fn test_name() {
    let config = sample_container_config();
    assert_eq!(config.name(), "test_container");
}

#[test]
fn test_container_image() {
    let config = sample_container_config();
    assert_eq!(config.container_image(), "test_image:latest");
}

#[test]
fn test_container_name() {
    let config = sample_container_config();
    assert_eq!(config.container_name(), "test_container-8080");
}

#[test]
fn test_url() {
    let config = sample_container_config();
    assert_eq!(config.url(), "0.0.0.0");
}

#[test]
fn test_connection_port() {
    let config = sample_container_config();
    assert_eq!(config.connection_port(), 8080);
}

#[test]
fn test_additional_ports() {
    let config = sample_container_config();
    assert_eq!(config.additional_ports(), Some(&[8081, 8082][..]));
}

#[test]
fn test_additional_env_vars() {
    let config = sample_container_config();
    assert_eq!(
        config.additional_env_vars(),
        Some(&["ENV_VAR=VALUE", "DEBUG=true"][..])
    );
}

#[test]
fn test_platform() {
    let config = sample_container_config();
    assert_eq!(config.platform(), Some("linux/amd64"));
}

#[test]
fn test_reuse_container() {
    let config = sample_container_config();
    assert!(config.reuse_container());
}

#[test]
fn test_keep_configuration() {
    let config = sample_container_config();
    assert!(!config.keep_configuration());
}

#[test]
fn test_wait_strategy() {
    let config = sample_container_config();
    assert_eq!(config.wait_strategy(), &WaitStrategy::default());
}

#[test]
fn test_host_network_defaults_to_false() {
    // `new` has no host_network parameter and always publishes ports.
    let config = sample_container_config();
    assert!(!config.host_network());
}

#[test]
fn test_default_host_network_is_false() {
    let config = ContainerConfig::default();
    assert!(!config.host_network());
}

#[test]
fn test_display_reports_host_network() {
    let config = ContainerConfig::builder()
        .name("test_container")
        .image("test_image")
        .tag("latest")
        .url("0.0.0.0")
        .connection_port(8080)
        .reuse_container(true)
        .keep_configuration(true)
        .host_network(true)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    assert!(format!("{config}").contains("host_network: true"));
    assert!(format!("{}", sample_container_config()).contains("host_network: false"));
}
