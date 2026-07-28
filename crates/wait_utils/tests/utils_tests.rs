/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

//! Tests for the pure helpers behind the wait strategies. These run no command and need
//! neither curl nor Docker.

use wait_utils::utils_test::{build_curl_args, streams_contain};

const EXPECTED: &str = "ready to accept requests";

#[test]
fn test_build_curl_args_fails_on_an_error_status() {
    let args = build_curl_args("http://localhost:8080/health");

    // Without --fail curl reports success for a 503, which is the whole defect: the
    // strategy degrades to "the port answered".
    assert!(
        args.contains(&"--fail".to_string()),
        "curl must fail on an error status, got: {args:?}"
    );
}

#[test]
fn test_build_curl_args_discards_the_body() {
    let args = build_curl_args("http://localhost:8080/health");
    let output = args
        .iter()
        .position(|a| a == "--output")
        .expect("curl must discard the body");

    assert_eq!(args[output + 1], "/dev/null");
}

#[test]
fn test_build_curl_args_keeps_the_error_readable() {
    let args = build_curl_args("http://localhost:8080/health");

    assert!(args.contains(&"--silent".to_string()));
    assert!(args.contains(&"--show-error".to_string()));
}

#[test]
fn test_build_curl_args_ends_with_the_url() {
    let health_url = "http://localhost:8080/health";
    let args = build_curl_args(health_url);

    assert_eq!(args.last().unwrap(), health_url);
}

#[test]
fn test_streams_contain_matches_stdout() {
    let stdout = b"server started, ready to accept requests\n";
    assert!(streams_contain(stdout, b"", EXPECTED));
}

#[test]
fn test_streams_contain_matches_stderr() {
    // Services built on glog, such as dgraph, log their readiness to stderr only.
    let stderr = b"I0728 13:00:00.000000 1 server.go:42] ready to accept requests\n";
    assert!(streams_contain(b"", stderr, EXPECTED));
}

#[test]
fn test_streams_contain_matches_either_stream() {
    let stdout = b"ready to accept requests\n";
    let stderr = b"ready to accept requests\n";
    assert!(streams_contain(stdout, stderr, EXPECTED));
}

#[test]
fn test_streams_contain_without_a_match() {
    assert!(!streams_contain(
        b"starting up\n",
        b"binding port\n",
        EXPECTED
    ));
}

#[test]
fn test_streams_contain_of_empty_output() {
    assert!(!streams_contain(b"", b"", EXPECTED));
}

#[test]
fn test_streams_contain_of_invalid_utf8() {
    // Container output is arbitrary bytes and must not panic the search.
    let stderr = [0xff, 0xfe, b'r', b'e', b'a', b'd', b'y'];

    assert!(!streams_contain(b"", &stderr, EXPECTED));
    assert!(streams_contain(b"", &stderr, "ready"));
}
