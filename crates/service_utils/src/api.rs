/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use crate::{ServiceStartConfig, ServiceUtil, ServiceUtilError};
use wait_utils::WaitStrategy;

impl ServiceUtil {
    /// Creates a new ServiceUtil instance.
    ///
    /// The `root_path` is the absolute path to the root directory of the
    /// service binaries. The `binaries` is a vector of names of the binaries
    /// that should be found in the `root_path`. The constructor checks if
    /// if each binary exists in the `root_path`.
    ///
    /// # Errors
    ///
    /// Fails if any of the binaries are not found in the `root_path`.
    ///
    pub async fn new(
        root_path: &'static str,
        binaries: Vec<&'static str>,
    ) -> Result<Self, ServiceUtilError> {
        Self::build(false, root_path, binaries).await
    }

    /// Creates a new ServiceUtil instance with debug mode.
    ///
    /// The `root_path` is the absolute path to the root directory of the
    /// service binaries. The `binaries` is a vector of names of the binaries
    /// that should be found in the `root_path`. The constructor checks if
    /// if each binary exists in the `root_path`.
    ///
    /// # Errors
    ///
    /// Fails if any of the binaries are not found in the `root_path`.
    ///
    pub async fn with_debug(
        root_path: &'static str,
        binaries: Vec<&'static str>,
    ) -> Result<Self, ServiceUtilError> {
        Self::build(true, root_path, binaries).await
    }

    /// Starts a service.
    ///
    /// The `program` is the name of the program to start. The `wait_strategy`
    /// is the wait strategy to use to wait for the service to start. The
    /// `env_var` is an optional environment variable to set when starting the
    /// service.
    ///
    /// This form carries no address, so it cannot honour
    /// [`WaitStrategy::WaitUntilReady`], whose probe needs one. Use
    /// [`ServiceUtil::start_service_from_config`] with a `ServiceStartConfig` for that: the
    /// config is where the address lives, because it is the one place the caller and the
    /// driver both read it.
    ///
    /// # Returns
    ///
    /// Returns the process ID of the started service, for use with
    /// [`ServiceUtil::stop_service`].
    ///
    /// # Errors
    ///
    /// Fails if the service fails to start.
    ///
    pub async fn start_service(
        &self,
        program: &str,
        program_args: Option<Vec<&str>>,
        wait_strategy: &WaitStrategy,
        env_vars: Option<Vec<(String, String)>>,
    ) -> Result<u32, ServiceUtilError> {
        self.start(
            program,
            program_args,
            env_vars,
            "localhost",
            None,
            wait_strategy.to_owned(),
        )
        .await
    }

    /// Starts a service with the given configuration.
    ///
    /// The `config` is the configuration of the service to start. The
    /// `wait_strategy` is the wait strategy to use to wait for the service to
    /// start. The `env_var` is an optional environment variable to set when
    /// starting the service.
    ///
    /// # Errors
    ///
    /// Fails if the service fails to start.
    ///
    pub async fn start_service_from_config(
        &self,
        service_start_config: ServiceStartConfig,
    ) -> Result<u32, ServiceUtilError> {
        // see src/service/start.rs
        self.start_config(service_start_config).await
    }

    /// Stops a service previously started by this `ServiceUtil`.
    ///
    /// A started service is detached on purpose, so that it outlives the call that started
    /// it. Nothing else stops it afterwards.
    ///
    /// Under Bazel that is harmless: the sandbox takes the process down when the test target
    /// finishes. Under Cargo there is no sandbox, so a service left running keeps holding its
    /// port and the next run of the same test cannot bind. Calling this at the end of a test
    /// is what makes a Cargo run repeatable.
    ///
    /// # Arguments
    ///
    /// * `pid` - The process ID returned by `start_service` or `start_service_from_config`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` once the process is gone, including when it had already exited.
    ///
    /// Note the process is stopped but not reaped: the crate drops the `Child` on purpose, so
    /// that a started service can outlive the call that started it. The PID therefore lingers
    /// as a zombie until the calling process exits. That is harmless, and the thing that
    /// actually matters is released either way: the port.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use service_utils::{ServiceUtil, WaitStrategy};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let service_util = ServiceUtil::new("bin", vec!["my_service"]).await?;
    ///
    /// let pid = service_util
    ///     .start_service("my_service", None, &WaitStrategy::NoWait, None)
    ///     .await?;
    ///
    /// service_util.stop_service(pid)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Fails if the process could not be signalled.
    ///
    pub fn stop_service(&self, pid: u32) -> Result<(), ServiceUtilError> {
        // see src/service/stop.rs
        self.stop(pid)
    }
}
