/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use crate::{ServiceUtil, ServiceUtilError};
use std::process::Command;

impl ServiceUtil {
    /// Stops a previously started service by its process ID.
    ///
    /// # Arguments
    ///
    /// * `pid` - The process ID returned when the service was started.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` once the process is gone, including when it had already exited.
    ///
    pub(crate) fn stop(&self, pid: u32) -> Result<(), ServiceUtilError> {
        self.dbg_print(&format!("[stop_service]: Stopping service with PID: {pid}"));

        let out = Command::new("kill")
            .arg(pid.to_string())
            .output()
            .map_err(|e| {
                ServiceUtilError::ServiceStopFailed(format!("could not run kill for {pid}: {e}"))
            })?;

        if out.status.success() {
            return Ok(());
        }

        // Already gone is the requested end state. `kill` says so on stderr, and reporting it
        // would turn a successful teardown into a spurious failure.
        let stderr = String::from_utf8_lossy(&out.stderr);
        if stderr.contains("No such process") {
            self.dbg_print(&format!("[stop_service]: PID {pid} had already exited."));
            return Ok(());
        }

        Err(ServiceUtilError::ServiceStopFailed(format!(
            "could not stop the service with PID {pid}: {}",
            stderr.trim()
        )))
    }
}
