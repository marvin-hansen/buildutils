/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

//! Tests for the `docker inspect` post-mortem parser. These run no Docker command.

use docker_utils::utils_test::{INSPECT_FORMAT, parse_inspect_line};

/// A container that the kernel OOM killer stopped. Captured from a real `docker inspect`.
const OOM_LINE: &str =
    "exited\tfalse\t0\ttrue\t137\t2026-07-29T03:53:36.896197823Z\t2026-07-29T03:53:37.001577407Z";

/// A container that exited of its own accord with a non-zero code.
const EXIT_3_LINE: &str =
    "exited\tfalse\t0\tfalse\t3\t2026-07-29T03:53:10.482226391Z\t2026-07-29T03:53:10.543785295Z";

#[test]
fn parses_an_oom_killed_container() {
    let diag = parse_inspect_line(OOM_LINE, None).expect("should parse");

    assert_eq!(diag.status(), "exited");
    assert!(!diag.running());
    assert_eq!(diag.restart_count(), 0);
    assert!(diag.oom_killed());
    assert_eq!(diag.exit_code(), 137);
    assert!(diag.looks_oom_killed());
    assert_eq!(diag.logs(), None);
}

// 137 is 128 + SIGKILL. Some runtime and storage combinations have failed to set OOMKilled,
// so the exit code alone must be enough to raise suspicion.
#[test]
fn treats_sigkill_as_a_suspected_oom_even_without_the_flag() {
    let line = "exited\tfalse\t0\tfalse\t137\t2026-07-28T15:16:41Z\t2026-07-28T15:16:59Z";
    assert!(parse_inspect_line(line, None).unwrap().looks_oom_killed());
}

#[test]
fn a_clean_exit_is_not_an_oom() {
    let line = "exited\tfalse\t0\tfalse\t0\t2026-07-28T15:16:41Z\t2026-07-28T15:16:59Z";
    assert!(!parse_inspect_line(line, None).unwrap().looks_oom_killed());
}

#[test]
fn a_non_zero_exit_that_is_not_sigkill_is_not_an_oom() {
    let diag = parse_inspect_line(EXIT_3_LINE, None).unwrap();

    assert_eq!(diag.exit_code(), 3);
    assert!(!diag.looks_oom_killed());
}

#[test]
fn parses_a_running_container() {
    // A running container reports the zero time as FinishedAt.
    let line = "running\ttrue\t0\tfalse\t0\t2026-07-28T15:16:41Z\t0001-01-01T00:00:00Z";
    let diag = parse_inspect_line(line, None).unwrap();

    assert_eq!(diag.status(), "running");
    assert!(diag.running());
    assert_eq!(diag.finished_at(), "0001-01-01T00:00:00Z");
    assert!(!diag.looks_oom_killed());
}

#[test]
fn parses_a_restarted_container() {
    let line = "running\ttrue\t4\tfalse\t0\t2026-07-28T15:16:41Z\t0001-01-01T00:00:00Z";
    assert_eq!(parse_inspect_line(line, None).unwrap().restart_count(), 4);
}

/// `docker inspect` terminates its output with a newline, and the command passes its raw
/// stdout straight to the parser. A newline left on the last field would corrupt FinishedAt.
#[test]
fn tolerates_the_trailing_newline_docker_actually_emits() {
    let diag = parse_inspect_line(&format!("{OOM_LINE}\n"), None).expect("should parse");

    assert_eq!(diag.finished_at(), "2026-07-29T03:53:37.001577407Z");
    assert!(diag.looks_oom_killed());
}

#[test]
fn tolerates_carriage_returns() {
    let diag = parse_inspect_line(&format!("{OOM_LINE}\r\n"), None).expect("should parse");
    assert_eq!(diag.finished_at(), "2026-07-29T03:53:37.001577407Z");
}

/// A partial post-mortem beats none: this runs where something has already gone wrong.
#[test]
fn an_unparseable_number_falls_back_rather_than_failing() {
    let line = "exited\tfalse\tnot-a-number\ttrue\tnot-a-number\t2026-07-28T15:16:41Z\t2026-07-28T15:16:59Z";
    let diag = parse_inspect_line(line, None).expect("should still parse");

    assert_eq!(diag.restart_count(), 0);
    assert_eq!(diag.exit_code(), 0);
    // The OOM flag is still trustworthy even when the numbers were not.
    assert!(diag.oom_killed());
}

/// Anything other than the literal "true" is false, including Docker's "false".
#[test]
fn only_the_literal_true_counts_as_true() {
    let line = "exited\tFalse\t0\tTrue\t0\t2026-07-28T15:16:41Z\t2026-07-28T15:16:59Z";
    let diag = parse_inspect_line(line, None).unwrap();

    assert!(!diag.running());
    assert!(!diag.oom_killed());
}

#[test]
fn rejects_truncated_output_rather_than_inventing_fields() {
    assert!(parse_inspect_line("exited\tfalse", None).is_err());
}

#[test]
fn rejects_empty_output() {
    let err = parse_inspect_line("", None).expect_err("empty output must not parse");
    assert!(err.to_string().contains("wanted 7"));
}

/// Extra trailing fields are tolerated, so adding a field to the format cannot break a
/// caller running an older parser.
#[test]
fn tolerates_extra_trailing_fields() {
    let line = format!("{OOM_LINE}\textra");
    assert!(parse_inspect_line(&line, None).unwrap().looks_oom_killed());
}

#[test]
fn carries_the_captured_logs() {
    let logs = "panic: out of memory\n".to_string();
    let diag = parse_inspect_line(OOM_LINE, Some(logs.clone())).unwrap();

    assert_eq!(diag.logs(), Some(logs.as_str()));
    assert!(format!("{diag}").contains("panic: out of memory"));
}

#[test]
fn display_reports_every_field() {
    let rendered = format!("{}", parse_inspect_line(OOM_LINE, None).unwrap());

    assert!(rendered.contains("status=exited"));
    assert!(rendered.contains("running=false"));
    assert!(rendered.contains("restarts=0"));
    assert!(rendered.contains("oom=true"));
    assert!(rendered.contains("exit=137"));
}

#[test]
fn display_says_so_when_the_logs_are_gone() {
    let rendered = format!("{}", parse_inspect_line(OOM_LINE, None).unwrap());
    assert!(rendered.contains("logs unavailable"));
}

/// The format and the parser have to agree on the field count, and the format has to use
/// real tabs rather than the two characters a backslash and a t.
#[test]
fn the_inspect_format_matches_what_the_parser_expects() {
    assert_eq!(
        INSPECT_FORMAT.matches('\t').count(),
        6,
        "seven fields need six separators: {INSPECT_FORMAT:?}"
    );
    assert!(
        !INSPECT_FORMAT.contains("\\t"),
        "the separator must be a real tab, not a literal backslash-t"
    );
    assert!(INSPECT_FORMAT.starts_with("{{.State.Status}}"));
    assert!(INSPECT_FORMAT.ends_with("{{.State.FinishedAt}}"));
    // RestartCount is a top level field, not one under State.
    assert!(INSPECT_FORMAT.contains("{{.RestartCount}}"));
}
