use crate::{EnvVar, ServiceUtil, ServiceUtilError};
use wait_utils::WaitStrategy;

impl ServiceUtil {
    /// Creates a new ServiceUtil instance.
    ///
    /// The `root_path` is the absolute path to the root directory of the
    /// service binaries. The `binaries` is a vector of names of the binaries
    /// that should be found in the `root_path`.
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
    /// that should be found in the `root_path`.
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
        wait_strategy: &WaitStrategy,
        env_var: Option<EnvVar>,
    ) -> Result<(), ServiceUtilError> {
        self.start(program, wait_strategy, env_var).await
    }

    /// Stops a service.
    ///
    /// The `program` is the name of the program to stop.
    ///
    /// # Errors
    ///
    /// Fails if the service fails to stop.
    ///
    pub async fn stop_service(&self, program: &str) -> Result<(), ServiceUtilError> {
        self.stop(program).await
    }
}
