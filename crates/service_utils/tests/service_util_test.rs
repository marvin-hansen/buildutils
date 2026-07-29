/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

//! Drives `ServiceUtil` against a real process.
//!
//! The service binary is a shell script this file writes at run time, rather than a compiled
//! artefact copied into place. Bazel stages such artefacts automatically and Cargo does not,
//! so a test depending on one passes under Bazel and fails under Cargo. Creating it here
//! keeps the suite identical under both.

use service_utils::{Probe, ProbeContext, ServiceStartConfig, ServiceUtil, WaitStrategy};
use std::fs;
use std::io::Write;
use std::net::TcpListener;
use std::os::unix::fs::PermissionsExt;
use std::sync::atomic::{AtomicU32, Ordering};

/// Must be a `&'static str` because that is what `ServiceUtil` takes.
const ROOT_PATH: &str = "/tmp/service_utils_test_bin";
const PROGRAM: &str = "dummy_service";

static ATTEMPTS: AtomicU32 = AtomicU32::new(0);

/// Writes an executable script that stays alive, and returns once it exists.
fn stage_service_binary() {
    fs::create_dir_all(ROOT_PATH).expect("Failed to create the test bin directory");

    let path = format!("{ROOT_PATH}/{PROGRAM}");
    let mut file = fs::File::create(&path).expect("Failed to create the test binary");
    file.write_all(b"#!/bin/sh\nsleep 30\n")
        .expect("Failed to write the test binary");
    drop(file);

    let mut perms = fs::metadata(&path)
        .expect("Failed to stat the test binary")
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).expect("Failed to chmod the test binary");
}

/// Ready as soon as it is called, recording that it ran at all.
fn always_ready(_ctx: &ProbeContext) -> Probe<(), String> {
    ATTEMPTS.fetch_add(1, Ordering::SeqCst);
    Probe::Ready(())
}

/// Records the address it was handed, so a test can prove the config reached the probe.
static SEEN_PORT: AtomicU32 = AtomicU32::new(0);

fn records_the_address(ctx: &ProbeContext) -> Probe<(), String> {
    SEEN_PORT.store(u32::from(ctx.port()), Ordering::SeqCst);
    Probe::Ready(())
}

/// Never ready, so a wait can only end in a timeout.
fn never_ready(_ctx: &ProbeContext) -> Probe<(), String> {
    Probe::Retry("not yet".to_string())
}

/// Returns a port with nothing listening on it.
fn a_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    listener.local_addr().expect("Failed to read addr").port()
}

#[tokio::test]
async fn a_missing_binary_is_reported() {
    stage_service_binary();

    let res = ServiceUtil::new(ROOT_PATH, vec!["no_such_program"]).await;

    let err = res.expect_err("a binary that is not there must be reported");
    assert!(
        err.to_string().contains("Binary not found"),
        "Unexpected error: {err}"
    );
}

#[tokio::test]
async fn a_present_binary_is_accepted() {
    stage_service_binary();

    let res = ServiceUtil::new(ROOT_PATH, vec![PROGRAM]).await;

    assert!(res.is_ok(), "a staged binary must be accepted: {res:?}");
}

/// The readiness probe is honoured here, not only in docker_utils.
///
/// A strategy only one driver can run is the defect this crate is trying not to repeat.
#[tokio::test]
async fn a_readiness_probe_is_honoured() {
    stage_service_binary();
    ATTEMPTS.store(0, Ordering::SeqCst);

    let service_util = ServiceUtil::new(ROOT_PATH, vec![PROGRAM])
        .await
        .expect("Failed to create ServiceUtil");

    let config = ServiceStartConfig::builder()
        .program(PROGRAM)
        .port(a_free_port())
        .wait_strategy(WaitStrategy::WaitUntilReady {
            probe: always_ready,
            timeout_secs: 10,
            retry_delay_ms: 10,
        })
        .build();

    let res = service_util.start_service_from_config(config).await;

    assert!(res.is_ok(), "a ready probe must start the service: {res:?}");
    assert!(
        ATTEMPTS.load(Ordering::SeqCst) >= 1,
        "the probe must actually have been run"
    );
}

/// The address configured by the caller is the address the probe receives.
///
/// This is the whole point of putting it in the config: the caller cannot connect until it
/// knows the port and the driver cannot bind until the caller has chosen one, so the config
/// is what both sides read.
#[tokio::test]
async fn the_configured_address_reaches_the_probe() {
    stage_service_binary();
    let port = a_free_port();
    SEEN_PORT.store(0, Ordering::SeqCst);

    let service_util = ServiceUtil::new(ROOT_PATH, vec![PROGRAM])
        .await
        .expect("Failed to create ServiceUtil");

    let config = ServiceStartConfig::builder()
        .program(PROGRAM)
        .host("127.0.0.1")
        .port(port)
        .wait_strategy(WaitStrategy::WaitUntilReady {
            probe: records_the_address,
            timeout_secs: 10,
            retry_delay_ms: 10,
        })
        .build();

    service_util
        .start_service_from_config(config)
        .await
        .expect("the service must start");

    assert_eq!(
        SEEN_PORT.load(Ordering::SeqCst),
        u32::from(port),
        "the probe must be handed the port the config declared"
    );
}

/// Without a port there is no address, and saying so beats probing a made-up one.
#[tokio::test]
async fn a_readiness_probe_without_a_port_is_reported() {
    stage_service_binary();

    let service_util = ServiceUtil::new(ROOT_PATH, vec![PROGRAM])
        .await
        .expect("Failed to create ServiceUtil");

    let config = ServiceStartConfig::builder()
        .program(PROGRAM)
        .wait_strategy(WaitStrategy::WaitUntilReady {
            probe: always_ready,
            timeout_secs: 10,
            retry_delay_ms: 10,
        })
        .build();

    let err = service_util
        .start_service_from_config(config)
        .await
        .expect_err("a probe with no address must be reported");

    assert!(
        err.to_string().contains("no port is set"),
        "the error must say what is missing: {err}"
    );
}

/// A probe that never succeeds ends in a timeout rather than hanging.
#[tokio::test]
async fn a_readiness_probe_that_never_succeeds_times_out() {
    stage_service_binary();

    let service_util = ServiceUtil::new(ROOT_PATH, vec![PROGRAM])
        .await
        .expect("Failed to create ServiceUtil");

    let config = ServiceStartConfig::builder()
        .program(PROGRAM)
        .port(a_free_port())
        .wait_strategy(WaitStrategy::WaitUntilReady {
            probe: never_ready,
            timeout_secs: 1,
            retry_delay_ms: 10,
        })
        .build();

    let err = service_util
        .start_service_from_config(config)
        .await
        .expect_err("a probe that never succeeds must time out");

    assert!(err.to_string().contains("Timeout"), "got: {err}");
    assert!(
        err.to_string().contains("not yet"),
        "the probe's own message must survive: {err}"
    );
}

/// A container log strategy has no container here, and is reported rather than waited on.
#[tokio::test]
async fn a_console_output_strategy_is_reported_as_unsupported() {
    stage_service_binary();

    let service_util = ServiceUtil::new(ROOT_PATH, vec![PROGRAM])
        .await
        .expect("Failed to create ServiceUtil");

    let config = ServiceStartConfig::builder()
        .program(PROGRAM)
        .wait_strategy(WaitStrategy::WaitUntilConsoleOutputContains(
            "ready".to_string(),
            1,
        ))
        .build();

    let err = service_util
        .start_service_from_config(config)
        .await
        .expect_err("service_utils cannot read container logs");

    assert!(
        err.to_string().contains("Unsupported wait strategy"),
        "Unexpected error: {err}"
    );
}

/// Starting a program that was never declared is reported.
#[tokio::test]
async fn starting_an_undeclared_program_is_reported() {
    stage_service_binary();

    let service_util = ServiceUtil::new(ROOT_PATH, vec![PROGRAM])
        .await
        .expect("Failed to create ServiceUtil");

    let config = ServiceStartConfig::builder()
        .program("some_other_program")
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let err = service_util
        .start_service_from_config(config)
        .await
        .expect_err("an undeclared program must be reported");

    assert!(
        err.to_string().contains("Binary has not been added"),
        "Unexpected error: {err}"
    );
}

/// A started service can be stopped again by the PID it reported.
///
/// Under Bazel the sandbox reaps the process when the target finishes, so nothing needed
/// this. Under Cargo there is no sandbox: a service left running keeps its port and the next
/// run of the same test cannot bind.
#[tokio::test]
async fn a_started_service_can_be_stopped_by_its_pid() {
    stage_service_binary();

    let service_util = ServiceUtil::new(ROOT_PATH, vec![PROGRAM])
        .await
        .expect("Failed to create ServiceUtil");

    let config = ServiceStartConfig::builder()
        .program(PROGRAM)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let pid = service_util
        .start_service_from_config(config)
        .await
        .expect("the service must start");

    assert!(pid > 0, "a started service must report its PID");
    assert!(
        process_is_alive(pid),
        "the service must be running before it is stopped"
    );

    service_util
        .stop_service(pid)
        .expect("the service must stop");

    // Give the signal a moment to land.
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(
        !process_is_running(pid),
        "the service must no longer be running after stop_service"
    );
}

/// Stopping something already gone is the requested end state, not an error.
#[tokio::test]
async fn stopping_an_already_stopped_service_succeeds() {
    stage_service_binary();

    let service_util = ServiceUtil::new(ROOT_PATH, vec![PROGRAM])
        .await
        .expect("Failed to create ServiceUtil");

    let config = ServiceStartConfig::builder()
        .program(PROGRAM)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let pid = service_util
        .start_service_from_config(config)
        .await
        .expect("the service must start");

    service_util
        .stop_service(pid)
        .expect("the first stop works");
    std::thread::sleep(std::time::Duration::from_millis(500));

    let res = service_util.stop_service(pid);

    assert!(
        res.is_ok(),
        "stopping an already stopped service must succeed: {res:?}"
    );
}

/// Whether a PID exists at all, including as a zombie.
fn process_is_alive(pid: u32) -> bool {
    process_state(pid).is_some()
}

/// Whether a PID is still a running process.
///
/// A stopped service is dead but not reaped, because the crate drops the `Child` on purpose
/// so the service can outlive the call that started it. It therefore lingers as a zombie
/// until the calling process exits, which keeps the PID in the table even though the process
/// is gone and its port released. `kill -0` cannot tell the two apart; the process state can.
fn process_is_running(pid: u32) -> bool {
    match process_state(pid) {
        Some(state) => !state.starts_with('Z'),
        None => false,
    }
}

/// The process state letter from `ps`, or `None` when the PID is gone entirely.
fn process_state(pid: u32) -> Option<String> {
    let out = std::process::Command::new("ps")
        .args(["-o", "stat=", "-p", &pid.to_string()])
        .output()
        .expect("Failed to run ps");

    let state = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if state.is_empty() { None } else { Some(state) }
}
