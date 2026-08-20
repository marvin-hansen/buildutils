/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

//! Verifies that bind mounts reach the container and that its process can read them.
//!
//! Asserting on `docker inspect` alone would only prove the flag was passed. What a caller
//! actually needs is a file the containerised program can open, which is the whole reason the
//! option exists: some programs take a path rather than a value -- Dgraph's
//! `--acl "secret-file=<path>"` has no inline form -- so the only alternative is baking the
//! file into an image, which commits it to a layer.
//!
//! Ports here are exclusive to this file so it cannot collide with the other integration tests.

use docker_utils::{ContainerConfig, DockerUtil, WaitStrategy};
use std::process::Command;

const VOLUME_NAME: &str = "docker-utils-volume";
const VOLUME_PORT: u16 = 6394;
const CONTENT: &str = "mounted-payload";

/// Writes a file in a fresh directory and returns that directory.
fn host_dir() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("{VOLUME_NAME}-{VOLUME_PORT}"));
    std::fs::create_dir_all(&dir).expect("Failed to create host directory");
    std::fs::write(dir.join("payload.txt"), CONTENT).expect("Failed to write payload");
    dir
}

fn remove_container(container_id: &str) {
    Command::new("docker")
        .args(["rm", "-f", container_id])
        .output()
        .expect("Failed to remove container");
}

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

/// A mounted file must be readable by the process inside the container.
#[test]
fn test_volume_is_readable_inside_the_container() {
    let dir = host_dir();
    let spec = format!("{}:/mnt/payload:ro", dir.display());
    let volumes = [spec.as_str()];

    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");
    let container_id = format!("{VOLUME_NAME}-{VOLUME_PORT}");
    remove_container(&container_id);

    let config = ContainerConfig::builder()
        .name(VOLUME_NAME)
        .image("redis")
        .tag("7-alpine")
        .url("0.0.0.0")
        .connection_port(VOLUME_PORT)
        .volumes(&volumes)
        .reuse_container(false)
        .keep_configuration(false)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let started = docker_util.setup_container(&config);
    assert!(started.is_ok(), "{started:?}");

    // The mount is declared...
    let mounts = inspect(&container_id, "{{range .Mounts}}{{.Destination}} {{end}}");
    assert!(mounts.contains("/mnt/payload"), "mounts were: {mounts}");

    // ...and the file is actually there, which is the part that matters.
    let out = Command::new("docker")
        .args(["exec", &container_id, "cat", "/mnt/payload/payload.txt"])
        .output()
        .expect("Failed to exec in container");

    assert!(
        out.status.success(),
        "reading the mounted file failed: {}",
        String::from_utf8_lossy(&out.stderr).trim()
    );
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), CONTENT);

    remove_container(&container_id);
    std::fs::remove_dir_all(&dir).ok();
}

/// A read-only mount must be read-only, or ":ro" is decoration.
#[test]
fn test_read_only_volume_rejects_writes() {
    let dir = host_dir();
    let spec = format!("{}:/mnt/payload:ro", dir.display());
    let volumes = [spec.as_str()];

    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");
    let name = "docker-utils-volume-ro";
    let port = 6395;
    let container_id = format!("{name}-{port}");
    remove_container(&container_id);

    let config = ContainerConfig::builder()
        .name(name)
        .image("redis")
        .tag("7-alpine")
        .url("0.0.0.0")
        .connection_port(port)
        .volumes(&volumes)
        .reuse_container(false)
        .keep_configuration(false)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    assert!(docker_util.setup_container(&config).is_ok());

    let out = Command::new("docker")
        .args([
            "exec",
            &container_id,
            "sh",
            "-c",
            "echo nope > /mnt/payload/payload.txt",
        ])
        .output()
        .expect("Failed to exec in container");

    assert!(
        !out.status.success(),
        "a :ro mount accepted a write, so the flag is not reaching Docker"
    );

    remove_container(&container_id);
    std::fs::remove_dir_all(&dir).ok();
}
