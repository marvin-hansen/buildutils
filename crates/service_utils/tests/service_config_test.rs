use service_utils::{ServiceStartConfig, WaitStrategy};

#[test]
fn test_service_config_basic() {
    let config = ServiceStartConfig::new("test_program", WaitStrategy::NoWait, None);

    assert_eq!(config.program(), "test_program");
    assert_eq!(config.wait_strategy(), &WaitStrategy::NoWait);
    assert_eq!(config.env_vars(), &None);
}

#[test]
fn test_service_config_with_env_vars() {
    let env_vars = vec!["KEY1=value1".to_string(), "KEY2=value2".to_string()];
    let config =
        ServiceStartConfig::new("test_program", WaitStrategy::NoWait, Some(env_vars.clone()));

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
        Some(vec!["KEY=value".to_string()]),
    );

    let display_string = format!("{}", config);
    assert!(display_string.contains("test_program"));
    assert!(display_string.contains("NoWait"));
    assert!(display_string.contains("KEY=value"));
}

#[test]
fn test_service_config_clone_and_eq() {
    let config1 = ServiceStartConfig::new("test_program", WaitStrategy::NoWait, None);

    let config2 = config1.clone();
    assert_eq!(config1, config2);

    let config3 = ServiceStartConfig::new("different_program", WaitStrategy::NoWait, None);

    assert_ne!(config1, config3);
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
    let config1 = ServiceStartConfig::new("a_program", WaitStrategy::NoWait, None);

    let config2 = ServiceStartConfig::new("b_program", WaitStrategy::NoWait, None);

    assert!(config1 < config2);
}
