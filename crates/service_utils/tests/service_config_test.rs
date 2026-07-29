/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use service_utils::{ServiceStartConfig, WaitStrategy};

#[test]
fn test_service_config_basic() {
    let config = ServiceStartConfig::new(
        "test_program",
        WaitStrategy::NoWait,
        None,
        None,
        "localhost",
        None,
    );

    assert_eq!(config.program(), "test_program");
    assert_eq!(config.wait_strategy(), &WaitStrategy::NoWait);
    assert_eq!(config.program_args(), &None);
    assert_eq!(config.env_vars(), &None);
}

#[test]
fn test_service_config_with_env_vars() {
    let env_vars = vec![("key".into(), "value".into())];
    let config = ServiceStartConfig::new(
        "test_program",
        WaitStrategy::NoWait,
        None,
        Some(env_vars.clone()),
        "localhost",
        None,
    );

    assert_eq!(config.program(), "test_program");
    assert_eq!(config.wait_strategy(), &WaitStrategy::NoWait);
    assert_eq!(config.env_vars(), &Some(env_vars));
}

#[test]
fn test_service_config_with_wait_strategy() {
    let wait_message = "Service is ready".to_string();
    let config = ServiceStartConfig::new(
        "test_program",
        WaitStrategy::WaitUntilConsoleOutputContains(wait_message.clone(), 10),
        None,
        None,
        "localhost",
        None,
    );

    assert_eq!(config.program(), "test_program");
    assert_eq!(
        config.wait_strategy(),
        &WaitStrategy::WaitUntilConsoleOutputContains(wait_message, 10)
    );
}

#[test]
fn test_service_config_display() {
    let config = ServiceStartConfig::new(
        "test_program",
        WaitStrategy::NoWait,
        None,
        Some(vec![("key".into(), "value".into())]),
        "localhost",
        None,
    );

    let display_string = format!("{}", config);
    assert!(display_string.contains("test_program"));
    assert!(display_string.contains("NoWait"));
}

#[test]
fn test_service_config_clone_and_eq() {
    let config1 = ServiceStartConfig::new(
        "test_program",
        WaitStrategy::NoWait,
        None,
        None,
        "localhost",
        None,
    );

    let config2 = config1.clone();
    assert_eq!(config1, config2);

    let config3 = ServiceStartConfig::new(
        "different_program",
        WaitStrategy::NoWait,
        None,
        None,
        "localhost",
        None,
    );

    assert_ne!(config1, config3);
}

#[test]
fn test_service_config_with_program_args() {
    let program_args = vec!["--flag", "value"];
    let config = ServiceStartConfig::new(
        "test_program",
        WaitStrategy::NoWait,
        Some(program_args.clone()),
        None,
        "localhost",
        None,
    );

    assert_eq!(config.program(), "test_program");
    assert_eq!(config.program_args(), &Some(program_args));
    assert_eq!(config.env_vars(), &None);
}

#[test]
fn test_service_config_default() {
    let config = ServiceStartConfig::default();
    assert_eq!(config.program(), "");
    assert_eq!(config.wait_strategy(), &WaitStrategy::default());
    assert_eq!(config.env_vars(), &None);
}

#[test]
fn test_service_config_ordering() {
    let config1 = ServiceStartConfig::new(
        "a_program",
        WaitStrategy::NoWait,
        None,
        None,
        "localhost",
        None,
    );

    let config2 = ServiceStartConfig::new(
        "b_program",
        WaitStrategy::NoWait,
        None,
        None,
        "localhost",
        None,
    );

    assert!(config1 < config2);
}

#[test]
fn test_service_config_host_defaults_to_localhost() {
    let config = ServiceStartConfig::builder()
        .program("test_program")
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    assert_eq!(config.host(), "localhost");
    assert_eq!(
        config.port(),
        None,
        "a service need not listen on a port at all"
    );
}

#[test]
fn test_service_config_carries_the_address() {
    // The address lives in the config because it is the one place the caller and the driver
    // both read it: neither can go first.
    let config = ServiceStartConfig::builder()
        .program("test_program")
        .host("127.0.0.1")
        .port(8080)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    assert_eq!(config.host(), "127.0.0.1");
    assert_eq!(config.port(), Some(8080));
}

#[test]
fn test_service_config_display_reports_the_address() {
    let config = ServiceStartConfig::builder()
        .program("test_program")
        .host("127.0.0.1")
        .port(8080)
        .wait_strategy(WaitStrategy::NoWait)
        .build();

    let display_string = format!("{config}");
    assert!(display_string.contains("host: 127.0.0.1"));
    assert!(display_string.contains("port: Some(8080)"));
}
