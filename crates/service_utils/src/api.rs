use crate::{EnvVar, ServiceUtil, ServiceUtilError};
use wait_utils::WaitStrategy;

impl ServiceUtil {
    pub async fn start_service(
        &self,
        program: String,
        wait_strategy: &WaitStrategy,
        env_var: Option<EnvVar>,
    ) -> Result<(), ServiceUtilError> {
        self.start(program, wait_strategy, env_var).await
    }
}
