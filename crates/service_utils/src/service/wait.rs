/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use crate::{ServiceUtil, ServiceUtilError};
use std::time::Duration;
use wait_utils::{ProbeContext, WaitStrategy};

impl ServiceUtil {
    /// Waits for the program to become ready based on the given wait strategy.
    ///
    /// # Arguments
    ///
    /// * `program` - The name of the program being waited on, handed to a readiness probe.
    /// * `host` - The host a caller would connect to, handed to a readiness probe.
    /// * `port` - The port a caller would connect to, handed to a readiness probe.
    /// * `wait_strategy` - The strategy used to determine when the program is ready.
    ///
    /// # Returns
    ///
    /// Returns a `ServiceUtilError` if waiting for the program fails or an unsupported wait strategy is used.
    //
    pub(crate) async fn wait_for_program(
        &self,
        program: &str,
        host: &'static str,
        port: Option<u16>,
        wait_strategy: &WaitStrategy,
    ) -> Result<(), ServiceUtilError> {
        match wait_strategy {
            WaitStrategy::WaitForDuration(duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {duration} seconds."
                ));
                wait_utils::wait_until_timeout(duration).map_err(|e| {
                    ServiceUtilError::ServiceHealthcheckFailed(format!(
                        "could not wait for {duration} seconds: {e}"
                    ))
                })?;
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
                wait_utils::wait_until_http_health_check(self.dbg, url, duration).map_err(|e| {
                    ServiceUtilError::ServiceHealthcheckFailed(format!(
                        "service did not pass its HTTP health check on {url}: {e}"
                    ))
                })?;
            }

            WaitStrategy::WaitForGrpcHealthCheck(url, duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {:?} on GRPC health check on {}.",
                    duration, url
                ));
                wait_utils::wait_until_grpc_health_check(self.dbg, url, duration)
                    .await
                    .map_err(|e| {
                        ServiceUtilError::ServiceHealthcheckFailed(format!(
                            "service did not pass its gRPC health check on {url}: {e}"
                        ))
                    })?;
            }

            // Honoured here as well as in docker_utils, so that the variant does not become
            // driver-specific: a strategy only one driver can run is the defect this crate is
            // trying not to repeat.
            //
            // The probe is synchronous by contract and may build its own async runtime, which
            // it can only do with no ambient one. A dedicated thread gives it that guarantee,
            // the same way docker_utils does. Joining it blocks this task for the duration,
            // which is what the crate's other wait strategies already do.
            WaitStrategy::WaitUntilReady {
                probe,
                timeout_secs,
                retry_delay_ms,
            } => {
                // Reported rather than guessed. A made-up port would send the probe at the
                // wrong address and report the resulting silence as a readiness timeout,
                // which is a symptom arbitrarily far from its cause.
                let Some(port) = port else {
                    return Err(ServiceUtilError::ServiceHealthcheckFailed(format!(
                        "WaitUntilReady needs an address for '{program}', but no port is set. \
                         Set .port(..) on the ServiceStartConfig and start the service with \
                         start_service_from_config."
                    )));
                };

                let dbg = self.dbg;
                let probe = *probe;
                let id = program.to_string();
                let host = host.to_string();
                let timeout = Duration::from_secs(*timeout_secs);
                let retry_delay = Duration::from_millis(*retry_delay_ms);

                let handle = std::thread::spawn(move || {
                    let mut attempt = 0u32;

                    wait_utils::wait_until_ready(dbg, timeout, retry_delay, || {
                        attempt = attempt.saturating_add(1);
                        probe(&ProbeContext::new(id.clone(), host.clone(), port, attempt))
                    })
                });

                let outcome = handle.join().map_err(|_| {
                    ServiceUtilError::ServiceHealthcheckFailed(format!(
                        "the readiness probe for '{program}' panicked"
                    ))
                })?;

                outcome.map_err(|e| {
                    ServiceUtilError::ServiceHealthcheckFailed(format!(
                        "service '{program}' did not become ready: {e}"
                    ))
                })?;
            }

            // Do nothing
            WaitStrategy::NoWait => {
                self.dbg_print("[start_container]: No wait. Return immediately.");
            }
        };
        Ok(())
    }
}
