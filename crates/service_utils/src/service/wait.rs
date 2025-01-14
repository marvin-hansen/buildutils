use wait_utils::WaitStrategy;
use crate::{ServiceUtil, ServiceUtilError};

impl ServiceUtil {

    /// Waits for the program to become ready based on the given wait strategy.
    ///
    /// # Arguments
    ///
    /// * `wait_strategy` - The strategy used to determine when the program is ready.
    ///
    /// # Returns
    ///
    /// Returns a `ServiceUtilError` if waiting for the program fails.
    ///
    /// # Panics
    ///
    /// Panics if the `WaitStrategy::WaitUntilConsoleOutputContains` is used,
    /// as it is not supported.
    ///
    pub(crate) async fn wait_for_program(
        &self,
        wait_strategy: &WaitStrategy,
    ) -> Result<(), ServiceUtilError> {
        match wait_strategy {
            WaitStrategy::WaitForDuration(duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {duration} seconds."
                ));
                wait_utils::wait_until_timeout(duration).expect("Failed to wait for duration");
            }

            WaitStrategy::WaitUntilConsoleOutputContains(_, _) => {
                panic!("WaitUntilConsoleOutputContains is not supported!");
            }

            WaitStrategy::WaitForHttpHealthCheck(url, duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {:?} on HTTP health check on {}.",
                    duration, url
                ));
                wait_utils::wait_until_http_health_check(self.dbg, url, duration)
                    .expect("Failed to wait for HTTP health check");
            }

            WaitStrategy::WaitForGrpcHealthCheck(url, duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {:?} on GRPC health check on {}.",
                    duration, url
                ));
                wait_utils::wait_until_grpc_health_check(self.dbg, url, duration)
                    .await
                    .expect("Failed to wait for HTTP health check");
            }

            WaitStrategy::NoWait => {
                self.dbg_print("[start_container]: No wait. Return immediately.");
                // Do nothing
            }
        };
        Ok(())
    }
}
