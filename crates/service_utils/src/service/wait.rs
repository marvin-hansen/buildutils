/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use crate::{ServiceUtil, ServiceUtilError};
use wait_utils::WaitStrategy;

impl ServiceUtil {
    /// Waits for the program to become ready based on the given wait strategy.
    ///
    /// # Arguments
    ///
    /// * `wait_strategy` - The strategy used to determine when the program is ready.
    ///
    /// # Returns
    ///
    /// Returns a `ServiceUtilError` if waiting for the program fails or an unsupported wait strategy is used.
    //
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
                return Err(ServiceUtilError::UnsupportedWaitStrategy(
                    "WaitUntilConsoleOutputContains Strategy is not supported".into(),
                ));
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

            // Do nothing
            WaitStrategy::NoWait => {
                self.dbg_print("[start_container]: No wait. Return immediately.");
            }
        };
        Ok(())
    }
}
