/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use std::process::Command;
use wait_utils::wait_until_console_output;

const READY_MESSAGE: &str = "ready to accept requests";

/// Starts a detached container running `shell_command`, replacing any earlier one.
fn run_logger_container(name: &str, shell_command: &str) {
    remove_container(name);

    let out = Command::new("docker")
        .args([
            "run",
            "--detach",
            "--name",
            name,
            "alpine:3.20",
            "sh",
            "-c",
            shell_command,
        ])
        .output()
        .expect("Failed to run docker");

    assert!(
        out.status.success(),
        "Failed to start {name}: {}",
        String::from_utf8_lossy(&out.stderr).trim()
    );
}

fn remove_container(name: &str) {
    Command::new("docker")
        .args(["rm", "-f", name])
        .output()
        .expect("Failed to remove container");
}

/// A service that reports readiness on stderr must be matched.
///
/// `docker logs` keeps the two streams apart rather than interleaving them, so searching
/// stdout alone means anything logging to stderr, which includes everything built on glog
/// such as dgraph, can never match and the strategy can only time out.
#[test]
fn test_console_output_matches_stderr() {
    let name = "wait-utils-stderr-logger";
    run_logger_container(name, &format!("echo '{READY_MESSAGE}' >&2; sleep 30"));

    let res = wait_until_console_output(false, name, READY_MESSAGE, &10);

    remove_container(name);

    assert!(
        res.is_ok(),
        "A readiness message on stderr must be matched: {res:?}"
    );
}

/// The stdout path must keep working.
#[test]
fn test_console_output_matches_stdout() {
    let name = "wait-utils-stdout-logger";
    run_logger_container(name, &format!("echo '{READY_MESSAGE}'; sleep 30"));

    let res = wait_until_console_output(false, name, READY_MESSAGE, &10);

    remove_container(name);

    assert!(
        res.is_ok(),
        "A readiness message on stdout must be matched: {res:?}"
    );
}

/// A container that does not exist times out rather than reporting readiness.
#[test]
fn test_console_output_of_a_missing_container() {
    let res = wait_until_console_output(false, "wait-utils-no-such-container", READY_MESSAGE, &2);

    let err = res.expect_err("A missing container must not be reported as ready");
    assert!(
        err.to_string().contains("Timeout"),
        "Unexpected error: {err}"
    );
}

/// A container that never logs the message times out, so the tests above are not vacuous.
#[test]
fn test_console_output_times_out_without_a_match() {
    let name = "wait-utils-silent-logger";
    run_logger_container(name, "echo 'starting up'; sleep 30");

    let res = wait_until_console_output(false, name, READY_MESSAGE, &2);

    remove_container(name);

    let err = res.expect_err("A container that never logs the message must time out");
    assert!(
        err.to_string().contains("Timeout"),
        "Unexpected error: {err}"
    );
}
