/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

//! Tests for the caller-supplied readiness probe. These run no command and need no Docker.

use std::cell::Cell;
use std::time::Duration;
use wait_utils::{Probe, wait_until_ready, wait_until_ready_async};

const FAST: Duration = Duration::from_millis(1);

#[test]
fn returns_the_value_the_probe_built() {
    let out = wait_until_ready(false, Duration::from_secs(5), FAST, || {
        Probe::Ready::<_, String>(42)
    });

    assert_eq!(out.unwrap(), 42);
}

#[test]
fn retries_until_ready() {
    let mut attempts = 0;
    let out = wait_until_ready(false, Duration::from_secs(5), FAST, || {
        attempts += 1;
        if attempts < 4 {
            Probe::Retry("starting".to_string())
        } else {
            Probe::Ready(attempts)
        }
    });

    assert_eq!(out.unwrap(), 4);
}

// The whole point of the three-way outcome: a permanent failure must not be retried into a
// timeout, because the timeout message would then describe the symptom and not the cause.
#[test]
fn a_fatal_probe_stops_immediately_and_reports_the_cause() {
    let mut attempts = 0;
    let out = wait_until_ready::<(), _, _>(false, Duration::from_secs(30), FAST, || {
        attempts += 1;
        Probe::Fatal("container stopped")
    });

    assert_eq!(attempts, 1, "a fatal probe must not be retried");
    assert!(format!("{}", out.unwrap_err()).contains("container stopped"));
}

/// A probe may retry for a while and only then discover the failure is permanent.
#[test]
fn a_fatal_probe_after_retries_stops_at_that_point() {
    let mut attempts = 0;
    let out = wait_until_ready::<(), _, _>(false, Duration::from_secs(30), FAST, || {
        attempts += 1;
        if attempts < 3 {
            Probe::Retry("starting")
        } else {
            Probe::Fatal("container stopped")
        }
    });

    assert_eq!(attempts, 3, "it must stop on the fatal attempt, not later");
    let err = format!("{}", out.unwrap_err());
    assert!(err.contains("container stopped"));
    assert!(
        !err.contains("Timeout"),
        "a fatal failure must not be reported as a timeout: {err}"
    );
}

#[test]
fn timing_out_reports_the_last_error_not_a_placeholder() {
    let out = wait_until_ready::<(), _, _>(false, Duration::from_millis(20), FAST, || {
        Probe::Retry("still starting")
    });

    let err = format!("{}", out.unwrap_err());
    assert!(err.contains("Timeout"));
    assert!(err.contains("still starting"));
}

/// Even a zero timeout must run the probe once, so that an already-ready service is not
/// reported as a timeout.
#[test]
fn a_zero_timeout_still_runs_the_probe_once() {
    let mut attempts = 0;
    let out = wait_until_ready(false, Duration::ZERO, FAST, || {
        attempts += 1;
        Probe::Ready::<_, String>(attempts)
    });

    assert_eq!(out.unwrap(), 1);
}

/// A zero timeout that is not immediately ready gives up after exactly one attempt.
#[test]
fn a_zero_timeout_gives_up_after_one_attempt() {
    let mut attempts = 0;
    let out = wait_until_ready::<(), _, _>(false, Duration::ZERO, FAST, || {
        attempts += 1;
        Probe::Retry("not yet")
    });

    assert_eq!(attempts, 1);
    assert!(format!("{}", out.unwrap_err()).contains("Timeout"));
}

/// The debug log collapses repeats, which must not change the outcome or the attempt count.
#[test]
fn collapsing_repeated_messages_does_not_change_the_result() {
    let mut attempts = 0;
    let out = wait_until_ready(true, Duration::from_secs(5), FAST, || {
        attempts += 1;
        match attempts {
            1..=3 => Probe::Retry("same".to_string()),
            4 => Probe::Retry("different".to_string()),
            _ => Probe::Ready(attempts),
        }
    });

    assert_eq!(out.unwrap(), 5);
}

#[tokio::test]
async fn async_returns_the_value_the_probe_built() {
    let out = wait_until_ready_async(false, Duration::from_secs(5), FAST, || async {
        Probe::Ready::<_, String>(42)
    })
    .await;

    assert_eq!(out.unwrap(), 42);
}

#[tokio::test]
async fn async_retries_until_ready() {
    // A Cell keeps the counter reachable from both the closure and the future it returns,
    // which a plain mutable capture cannot do across an async block.
    let attempts = Cell::new(0u32);

    let out = wait_until_ready_async(false, Duration::from_secs(5), FAST, || {
        attempts.set(attempts.get() + 1);
        let seen = attempts.get();
        async move {
            if seen < 4 {
                Probe::Retry("starting".to_string())
            } else {
                Probe::Ready(seen)
            }
        }
    })
    .await;

    assert_eq!(out.unwrap(), 4);
    assert_eq!(attempts.get(), 4);
}

#[tokio::test]
async fn async_fatal_probe_stops_immediately() {
    let attempts = Cell::new(0u32);

    let out = wait_until_ready_async::<(), _, _, _>(false, Duration::from_secs(30), FAST, || {
        attempts.set(attempts.get() + 1);
        async { Probe::Fatal("container stopped") }
    })
    .await;

    assert_eq!(attempts.get(), 1, "a fatal probe must not be retried");
    assert!(format!("{}", out.unwrap_err()).contains("container stopped"));
}

#[tokio::test]
async fn async_timing_out_reports_the_last_error() {
    let out =
        wait_until_ready_async::<(), _, _, _>(false, Duration::from_millis(20), FAST, || async {
            Probe::Retry("still starting")
        })
        .await;

    let err = format!("{}", out.unwrap_err());
    assert!(err.contains("Timeout"));
    assert!(err.contains("still starting"));
}

#[tokio::test]
async fn async_zero_timeout_still_runs_the_probe_once() {
    let attempts = Cell::new(0u32);

    let out = wait_until_ready_async(false, Duration::ZERO, FAST, || {
        attempts.set(attempts.get() + 1);
        let seen = attempts.get();
        async move { Probe::Ready::<_, String>(seen) }
    })
    .await;

    assert_eq!(out.unwrap(), 1);
}
