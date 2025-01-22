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
    ) -> Result<(), ServiceUtilError> {
        self.start(program, program_args, env_vars, wait_strategy.to_owned())
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
    ) -> Result<(), ServiceUtilError> {
        self.start_config(service_start_config).await
    }
}
