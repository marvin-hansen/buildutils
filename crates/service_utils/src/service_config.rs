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
/// The `host` and `port` are the address a caller would connect to. They live here rather
/// than being discovered, because the address is a chicken and egg problem: the caller cannot
/// connect until it knows the port, and the driver cannot bind until the caller has chosen
/// one. The config is what breaks it, by being the single place both sides read.
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
    program_args: Option<Vec<&'static str>>,
    env_vars: Option<Vec<(String, String)>>,
    /// The host a caller would connect to, handed to a readiness probe.
    #[builder(default = "localhost")]
    host: &'static str,
    /// The port a caller would connect to, handed to a readiness probe.
    ///
    /// Optional because a service need not listen on one at all. It is required only by
    /// [`WaitStrategy::WaitUntilReady`], which reports a clear error when it is missing
    /// rather than probing a made-up address.
    port: Option<u16>,
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
    /// let config = ServiceStartConfig::new(
    ///     "program",
    ///     WaitStrategy::NoWait,
    ///     None,
    ///     None,
    ///     "localhost",
    ///     Some(8080),
    /// );
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a new `ServiceStartConfig` instance.
    ///
    pub fn new(
        program: &'static str,
        wait_strategy: WaitStrategy,
        program_args: Option<Vec<&'static str>>,
        env_vars: Option<Vec<(String, String)>>,
        host: &'static str,
        port: Option<u16>,
    ) -> Self {
        Self {
            program,
            wait_strategy,
            program_args,
            env_vars,
            host,
            port,
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
    pub const fn env_vars(&self) -> &Option<Vec<(String, String)>> {
        &self.env_vars
    }
    #[inline]
    pub fn program_args(&self) -> &Option<Vec<&'static str>> {
        &self.program_args
    }

    /// The host a caller would connect to, handed to a readiness probe.
    #[inline]
    pub const fn host(&self) -> &'static str {
        self.host
    }

    /// The port a caller would connect to, handed to a readiness probe.
    #[inline]
    pub const fn port(&self) -> Option<u16> {
        self.port
    }
}

impl Display for ServiceStartConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServiceStartConfig {{ program: {}, wait_strategy: {}, env_vars: {:?}, \
            host: {}, port: {:?} }}",
            self.program, self.wait_strategy, self.env_vars, self.host, self.port
        )
    }
}
