/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

//! Tests for the caller-supplied readiness probe. These run no command and need no Docker.

use std::time::Duration;
use wait_utils::{Probe, ProbeContext, WaitStrategy, wait_until_ready};

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

/// The context is what removes the need for a probe to capture anything.
#[test]
fn a_probe_context_reports_what_the_driver_knows() {
    let ctx = ProbeContext::new("redis-6379".to_string(), "127.0.0.1".to_string(), 6379, 3);

    assert_eq!(ctx.id(), "redis-6379");
    assert_eq!(ctx.host(), "127.0.0.1");
    assert_eq!(ctx.port(), 6379);
    assert_eq!(ctx.attempt(), 3);
}

/// A probe can escalate on its own after N attempts, without carrying state.
#[test]
fn a_probe_can_escalate_on_the_attempt_count() {
    fn give_up_after_two(ctx: &ProbeContext) -> Probe<(), String> {
        if ctx.attempt() >= 2 {
            Probe::Fatal("tried twice, giving up".to_string())
        } else {
            Probe::Retry("not yet".to_string())
        }
    }

    let mut attempt = 0u32;
    let out = wait_until_ready::<(), _, _>(false, Duration::from_secs(30), FAST, || {
        attempt += 1;
        give_up_after_two(&ProbeContext::new(
            "id".to_string(),
            "host".to_string(),
            1,
            attempt,
        ))
    });

    assert_eq!(attempt, 2, "it must stop on the attempt that escalated");
    assert!(format!("{}", out.unwrap_err()).contains("giving up"));
}

/// A `fn` pointer keeps every derive `WaitStrategy` and its holders rely on.
///
/// This is the whole reason the probe is a function pointer rather than a boxed closure: a
/// `Box<dyn Fn>` would strip `Clone`, `Eq`, `Ord` and `Hash` from `ContainerConfig` too.
#[test]
fn the_wait_until_ready_variant_keeps_the_derives() {
    fn probe(_ctx: &ProbeContext) -> Probe<(), String> {
        Probe::Ready(())
    }

    let strategy = WaitStrategy::WaitUntilReady {
        probe,
        timeout_secs: 30,
        retry_delay_ms: 100,
    };

    let cloned = strategy.clone();
    assert_eq!(strategy, cloned);
    assert!(format!("{strategy:?}").contains("WaitUntilReady"));
    assert!(strategy != WaitStrategy::NoWait);

    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(strategy);
    assert_eq!(set.len(), 1);
}
