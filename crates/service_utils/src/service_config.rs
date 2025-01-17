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
///     .wait_strategy(WaitStrategy::NoWait)
///     .env_vars(vec!["var1=value1".into(), "var2=value2".into()])
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
    env_vars: Option<Vec<String>>,
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
    /// let config = ServiceStartConfig::new("program", WaitStrategy::NoWait, None);
    /// ```
    /// Configuration with optional environment variables using the constructor:
    ///
    /// ```rust
    /// use service_utils::*;
    ///
    /// let config = ServiceStartConfig::new("program", WaitStrategy::NoWait, Some(vec!["var1=value1".into(), "var2=value2".into()]));
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a new `ServiceStartConfig` instance.
    ///
    pub fn new(
        program: &'static str,
        wait_strategy: WaitStrategy,
        env_vars: Option<Vec<String>>,
    ) -> Self {
        Self {
            program,
            wait_strategy,
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
    pub const fn env_vars(&self) -> &Option<Vec<String>> {
        &self.env_vars
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
