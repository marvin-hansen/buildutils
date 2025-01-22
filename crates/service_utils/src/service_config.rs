/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

// https://github.com/elastio/bon
use bon::Builder;
use std::fmt::{Display, Formatter};
use wait_utils::WaitStrategy;

/// Create a new instance of the `ServiceStartConfig` struct using the builder.
///
/// The `program` is the name of the program to start. The `wait_strategy`
/// is the wait strategy to use to wait for the service to start. The
/// `env_var` is an optional environment variable to set when starting the
/// service.
///
/// # Examples
///
/// Basic configuration using the derived builder:
/// ```rust
/// use service_utils::*;
///
/// let config = ServiceStartConfig::builder()
///     .program("program")
///     .wait_strategy(WaitStrategy::NoWait)
///     .build();
/// ```
///
/// Configuration with optional environment variables using the builder:
///
/// ```rust
/// use service_utils::*;
///
/// let config = ServiceStartConfig::builder()
///     .program("program")
///     .program_args(vec!["arg1", "arg2"])
///     .wait_strategy(WaitStrategy::NoWait)
///     .env_vars(vec![("KEY".into(), "VALUE".into())])
///     .build();
/// ```
///
/// # Returns
///
/// Returns a new `ServiceStartConfig` instance.
///
#[derive(Builder, Debug, Default, Clone, Eq, PartialOrd, Ord, PartialEq, Hash)]
pub struct ServiceStartConfig {
    program: &'static str,
    wait_strategy: WaitStrategy,
    program_args: Option<Vec< &'static str,>>,
    env_vars: Option<Vec<(String, String)>>,
}

impl ServiceStartConfig {
    /// Create a new instance of the `ServiceStartConfig` struct using the constructor.
    ///
    /// The `program` is the name of the program to start. The `wait_strategy`
    /// is the wait strategy to use to wait for the service to start. The
    /// `env_var` is an optional environment variable to set when starting the
    /// service.
    ///
    /// # Examples
    ///
    /// Basic configuration using the constructor:
    ///
    /// ```rust
    /// use service_utils::*;
    ///
    /// let config = ServiceStartConfig::new("program", WaitStrategy::NoWait, None, None);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a new `ServiceStartConfig` instance.
    ///
    pub fn new(
        program: &'static str,
        wait_strategy: WaitStrategy,
        program_args: Option<Vec< &'static str,>>,
        env_vars: Option<Vec<(String, String)>>,
    ) -> Self {
        Self {
            program,
            wait_strategy,
            program_args,
            env_vars,
        }
    }
}

impl ServiceStartConfig {
    #[inline]
    pub const fn program(&self) -> &'static str {
        self.program
    }



    #[inline]
    pub const fn wait_strategy(&self) -> &WaitStrategy {
        &self.wait_strategy
    }

    #[inline]
    pub const fn env_vars(&self) -> &Option<Vec<(String, String)>>
    {
        &self.env_vars
    }
    #[inline]
    pub fn program_args(&self) -> &Option<Vec<&'static str>> {
        &self.program_args
    }
}

impl Display for ServiceStartConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServiceStartConfig {{ program: {}, wait_strategy: {}, env_vars: {:?} }}",
            self.program, self.wait_strategy, self.env_vars
        )
    }
}
