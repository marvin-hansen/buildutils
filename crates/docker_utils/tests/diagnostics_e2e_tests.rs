/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

//! Drives `container_diagnostics` against real containers.
//!
//! The containers here are started directly rather than through `ContainerConfig`, because
//! the crate always passes `--rm` and always publishes a port. A post-mortem needs a corpse,
//! so these deliberately omit `--rm`, and the memory limit an OOM needs is not something
//! `ContainerConfig` can express.

use docker_utils::DockerUtil;
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Allocates a megabyte at a time until the cgroup limit kills it.
const OOM_SCRIPT: &str =
    r#"x=""; while true; do x="$x$(head -c 1000000 /dev/zero | tr '\0' 'a')"; done"#;

fn remove_container(container_id: &str) {
    Command::new("docker")
        .args(["rm", "-f", container_id])
        .output()
        .expect("Failed to remove container");
}

/// Starts a detached container without `--rm`, so that it survives its own exit.
fn run_container(container_id: &str, extra_args: &[&str], shell_command: &str) {
    remove_container(container_id);

    let mut cmd = Command::new("docker");
    cmd.args(["run", "--detach", "--name", container_id]);
    cmd.args(extra_args);
    cmd.args(["alpine:3.20", "sh", "-c", shell_command]);

    let out = cmd.output().expect("Failed to run docker");

    assert!(
        out.status.success(),
        "Failed to start {container_id}: {}",
        String::from_utf8_lossy(&out.stderr).trim()
    );
}

/// Blocks until the container is no longer running.
fn wait_until_exited(container_id: &str) {
    let deadline = Instant::now() + Duration::from_secs(60);

    loop {
        let out = Command::new("docker")
            .args(["inspect", "--format={{.State.Running}}", container_id])
            .output()
            .expect("Failed to run docker inspect");

        if String::from_utf8_lossy(&out.stdout).trim() == "false" {
            return;
        }

        assert!(
            Instant::now() < deadline,
            "{container_id} was still running after 60s"
        );
        sleep(Duration::from_millis(200));
    }
}

/// The headline case: an OOM kill must be reported as one.
///
/// An OOM presents to a client as a bare connection error, because the server did not get to
/// write an explanation. Without this the caller sees only "connection refused".
#[test]
fn diagnoses_an_oom_killed_container() {
    let container_id = "docker-utils-diag-oom";
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

    run_container(
        container_id,
        &["--memory", "8m", "--memory-swap", "8m"],
        OOM_SCRIPT,
    );
    wait_until_exited(container_id);

    let diag = docker_util.container_diagnostics(container_id, 20);

    remove_container(container_id);

    let diag = diag.expect("diagnostics must be available for a container that still exists");
    assert!(!diag.running());
    assert!(
        diag.looks_oom_killed(),
        "an OOM-killed container must be reported as one, got: {diag}"
    );
    assert_eq!(diag.exit_code(), 137, "137 is 128 + SIGKILL");
}

/// An ordinary non-zero exit must NOT be mistaken for an OOM.
#[test]
fn a_plain_non_zero_exit_is_not_reported_as_an_oom() {
    let container_id = "docker-utils-diag-exit3";
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

    run_container(container_id, &[], "exit 3");
    wait_until_exited(container_id);

    let diag = docker_util.container_diagnostics(container_id, 20);

    remove_container(container_id);

    let diag = diag.expect("diagnostics must be available");
    assert_eq!(diag.exit_code(), 3);
    assert!(!diag.oom_killed());
    assert!(
        !diag.looks_oom_killed(),
        "a clean non-zero exit must not read as an OOM, got: {diag}"
    );
}

#[test]
fn diagnoses_a_running_container() {
    let container_id = "docker-utils-diag-running";
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

    run_container(container_id, &[], "sleep 60");

    let diag = docker_util.container_diagnostics(container_id, 20);

    remove_container(container_id);

    let diag = diag.expect("diagnostics must be available");
    assert_eq!(diag.status(), "running");
    assert!(diag.running());
    assert_eq!(diag.exit_code(), 0);
    assert!(!diag.looks_oom_killed());
}

/// The log tail has to come from both streams, because anything built on glog writes only to
/// stderr and `docker logs` keeps the streams apart.
#[test]
fn captures_the_log_tail_from_both_streams() {
    let container_id = "docker-utils-diag-logs";
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

    run_container(
        container_id,
        &[],
        "echo on-stdout; echo on-stderr >&2; sleep 60",
    );
    // Give the container a moment to write before reading its logs back.
    sleep(Duration::from_millis(500));

    let diag = docker_util.container_diagnostics(container_id, 20);

    remove_container(container_id);

    let diag = diag.expect("diagnostics must be available");
    let logs = diag.logs().expect("logs must be captured");
    assert!(logs.contains("on-stdout"), "got: {logs}");
    assert!(logs.contains("on-stderr"), "got: {logs}");
}

/// A zero tail captures no lines, but still yields the exit state.
#[test]
fn a_zero_log_tail_still_yields_the_exit_state() {
    let container_id = "docker-utils-diag-notail";
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

    run_container(container_id, &[], "echo noisy; exit 7");
    wait_until_exited(container_id);

    let diag = docker_util.container_diagnostics(container_id, 0);

    remove_container(container_id);

    let diag = diag.expect("diagnostics must be available");
    assert_eq!(diag.exit_code(), 7);
    assert_eq!(
        diag.logs().map(str::trim),
        Some(""),
        "a zero tail must capture no lines"
    );
}

/// Once the container is gone there is nothing to report, and that must be an error rather
/// than a fabricated answer.
///
/// This is what a caller hits when the container was started with `--rm`, which is what this
/// crate always does: Docker deletes it the moment it exits, taking the exit code with it.
#[test]
fn reports_an_error_once_the_container_is_gone() {
    let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

    let err = docker_util
        .container_diagnostics("docker-utils-diag-never-existed", 20)
        .expect_err("a removed container must not yield diagnostics");

    assert!(
        err.to_string().contains("docker inspect failed"),
        "Unexpected error: {err}"
    );
}
