/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

//! Tests for the pure helpers behind the Docker commands. These run no Docker command and
//! need no daemon.

use docker_utils::utils_test::{
    build_run_args, container_port_from_id, exact_name_filter, image_tag,
};

const CONTAINER_ID: &str = "test-8080";
const IMAGE: &str = "test_image:latest";

/// Builds args for a container publishing ports, with everything else unset.
fn published_args() -> Vec<String> {
    build_run_args(CONTAINER_ID, 8080, None, None, None, IMAGE, false)
        .expect("Failed to build run args")
}

/// Builds args for the same container on the host network.
fn host_network_args() -> Vec<String> {
    build_run_args(CONTAINER_ID, 8080, None, None, None, IMAGE, true)
        .expect("Failed to build run args")
}

/// Returns the value following `flag`, if any.
fn value_after(args: &[String], flag: &str) -> Option<String> {
    let idx = args.iter().position(|a| a == flag)?;
    args.get(idx + 1).cloned()
}

/// Returns every value following an occurrence of `flag`.
fn values_after(args: &[String], flag: &str) -> Vec<String> {
    args.iter()
        .enumerate()
        .filter(|(_, a)| a.as_str() == flag)
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect()
}

#[test]
fn test_build_run_args_base_command() {
    let args = published_args();

    assert_eq!(args[0], "run");
    assert_eq!(args[1], "--rm");
    assert_eq!(args[2], "--detach");
    // The image is always the final argument.
    assert_eq!(args.last().unwrap(), IMAGE);
}

#[test]
fn test_build_run_args_publishes_connection_port() {
    let args = published_args();

    assert_eq!(values_after(&args, "--publish"), vec!["8080:8080"]);
    assert!(!args.contains(&"--network".to_string()));
}

#[test]
fn test_build_run_args_publishes_additional_ports() {
    let args = build_run_args(
        CONTAINER_ID,
        8080,
        Some(&[8081, 8082]),
        None,
        None,
        IMAGE,
        false,
    )
    .expect("Failed to build run args");

    assert_eq!(
        values_after(&args, "--publish"),
        vec!["8080:8080", "8081:8081", "8082:8082"]
    );
}

#[test]
fn test_build_run_args_sets_container_name() {
    let args = published_args();
    assert_eq!(value_after(&args, "--name"), Some(CONTAINER_ID.to_string()));
}

#[test]
fn test_build_run_args_sets_platform() {
    let args = build_run_args(
        CONTAINER_ID,
        8080,
        None,
        Some("linux/amd64"),
        None,
        IMAGE,
        false,
    )
    .expect("Failed to build run args");

    assert_eq!(
        value_after(&args, "--platform"),
        Some("linux/amd64".to_string())
    );
}

#[test]
fn test_build_run_args_omits_platform_when_unset() {
    let args = published_args();
    assert!(!args.contains(&"--platform".to_string()));
}

#[test]
fn test_build_run_args_gives_each_env_var_its_own_flag() {
    let args = build_run_args(
        CONTAINER_ID,
        8080,
        None,
        None,
        Some(&["ENV_VAR=VALUE", "DEBUG=true"]),
        IMAGE,
        false,
    )
    .expect("Failed to build run args");

    // Docker only reads one value per -e, so a shared flag would turn the second variable
    // into the image name.
    assert_eq!(
        values_after(&args, "-e"),
        vec!["ENV_VAR=VALUE", "DEBUG=true"]
    );
    assert_eq!(args.last().unwrap(), IMAGE);
}

#[test]
fn test_build_run_args_host_network_sets_network_host() {
    let args = host_network_args();
    assert_eq!(value_after(&args, "--network"), Some("host".to_string()));
}

#[test]
fn test_build_run_args_host_network_publishes_nothing() {
    let args = host_network_args();

    assert!(
        !args.contains(&"--publish".to_string()),
        "host network mode must not publish ports, got: {args:?}"
    );
}

#[test]
fn test_build_run_args_host_network_skips_additional_ports() {
    let args = build_run_args(
        CONTAINER_ID,
        8080,
        Some(&[8081, 8082]),
        None,
        None,
        IMAGE,
        true,
    )
    .expect("Failed to build run args");

    assert!(
        !args.contains(&"--publish".to_string()),
        "host network mode must not publish additional ports, got: {args:?}"
    );
    assert_eq!(value_after(&args, "--network"), Some("host".to_string()));
}

#[test]
fn test_build_run_args_host_network_keeps_other_arguments() {
    let args = build_run_args(
        CONTAINER_ID,
        8080,
        None,
        Some("linux/arm64"),
        Some(&["DEBUG=true"]),
        IMAGE,
        true,
    )
    .expect("Failed to build run args");

    assert_eq!(
        value_after(&args, "--platform"),
        Some("linux/arm64".to_string())
    );
    assert_eq!(value_after(&args, "--name"), Some(CONTAINER_ID.to_string()));
    assert_eq!(values_after(&args, "-e"), vec!["DEBUG=true"]);
    assert_eq!(args.last().unwrap(), IMAGE);
}

#[test]
fn test_build_run_args_rejects_zero_port() {
    let res = build_run_args(CONTAINER_ID, 8080, Some(&[0]), None, None, IMAGE, false);

    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("Port cannot be 0"));
}

#[test]
fn test_build_run_args_rejects_zero_port_on_host_network() {
    // Validation must not be skipped just because ports are not published.
    let res = build_run_args(CONTAINER_ID, 8080, Some(&[0]), None, None, IMAGE, true);

    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("Port cannot be 0"));
}

#[test]
fn test_exact_name_filter_anchors_the_pattern() {
    // Without the anchors Docker also matches nginx-8080 for a nginx-80 lookup.
    assert_eq!(exact_name_filter("nginx-80"), "--filter=name=^nginx-80$");
}

#[test]
fn test_exact_name_filter_escapes_dots() {
    // A dot is a regular expression wildcard and the only metacharacter Docker allows in a
    // container name.
    assert_eq!(
        exact_name_filter("my.app-80"),
        "--filter=name=^my\\.app-80$"
    );
}

#[test]
fn test_container_port_from_id() {
    assert_eq!(container_port_from_id("nginx-80").unwrap(), 80);
    assert_eq!(container_port_from_id("my-app-8080").unwrap(), 8080);
    assert_eq!(container_port_from_id("postgres-5432").unwrap(), 5432);
}

#[test]
fn test_container_port_from_id_without_port_suffix() {
    // Must report an error rather than panicking: container IDs arrive from the caller and
    // are not required to carry a port suffix.
    let res = container_port_from_id("redis");

    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("redis"));
}

#[test]
fn test_container_port_from_id_rejects_out_of_range_port() {
    // 70000 does not fit a u16.
    assert!(container_port_from_id("service-70000").is_err());
}

#[test]
fn test_container_port_from_id_rejects_empty_suffix() {
    assert!(container_port_from_id("service-").is_err());
}

#[test]
fn test_image_tag() {
    assert_eq!(image_tag("nginx:1.27.0"), Some("1.27.0"));
    assert_eq!(image_tag("postgres:17-alpine3.20"), Some("17-alpine3.20"));
}

#[test]
fn test_image_tag_trims_command_output() {
    // docker ps terminates its output with a newline.
    assert_eq!(image_tag("nginx:1.27.0\n"), Some("1.27.0"));
}

#[test]
fn test_image_tag_of_registry_path() {
    assert_eq!(
        image_tag("asia-northeast1-docker.pkg.dev/project/repo/api:b422ae3"),
        Some("b422ae3")
    );
}

#[test]
fn test_image_tag_ignores_registry_port() {
    // The colon introduces the registry port, not a tag, so there is no tag to report.
    assert_eq!(image_tag("registry.io:5000/image"), None);
    // With a tag present the registry port must not confuse the lookup.
    assert_eq!(image_tag("registry.io:5000/image:1.0"), Some("1.0"));
}

#[test]
fn test_image_tag_of_untagged_image() {
    assert_eq!(image_tag("nginx"), None);
}
