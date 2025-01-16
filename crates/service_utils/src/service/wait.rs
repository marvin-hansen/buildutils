use crate::ServiceUtilError;
use wait_utils::WaitStrategy;

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
    dbg: bool,
    wait_strategy: &WaitStrategy,
) -> Result<(), ServiceUtilError> {
    match wait_strategy {
        WaitStrategy::WaitForDuration(duration) => {
            wait_utils::wait_until_timeout(duration).expect("Failed to wait for duration");
        }

        WaitStrategy::WaitUntilConsoleOutputContains(_, _) => {
            return Err(ServiceUtilError::UnsupportedWaitStrategy(
                "WaitUntilConsoleOutputContains Strategy is not supported".into(),
            ));
        }

        WaitStrategy::WaitForHttpHealthCheck(url, duration) => {
            wait_utils::wait_until_http_health_check(dbg, url, duration)
                .expect("Failed to wait for HTTP health check");
        }

        WaitStrategy::WaitForGrpcHealthCheck(url, duration) => {
            wait_utils::wait_until_grpc_health_check(dbg, url, duration)
                .await
                .expect("Failed to wait for HTTP health check");
        }

        // Do nothing
        WaitStrategy::NoWait => {}
    };
    Ok(())
}
