use crate::{EnvVar, ServiceUtil, ServiceUtilError};
use wait_utils::WaitStrategy;

impl ServiceUtil {
    pub async fn new(
        root_path: &'static str,
        binaries: Vec<&'static str>,
    ) -> Result<Self, ServiceUtilError> {
        Self::build(false, root_path, binaries).await
    }

    pub async fn with_debug(
        root_path: &'static str,
        binaries: Vec<&'static str>,
    ) -> Result<Self, ServiceUtilError> {
        Self::build(true, root_path, binaries).await
    }

    pub async fn start_service(
        &self,
        program: String,
        wait_strategy: &WaitStrategy,
        env_var: Option<EnvVar>,
    ) -> Result<(), ServiceUtilError> {
        self.start(program, wait_strategy, env_var).await
    }
}
