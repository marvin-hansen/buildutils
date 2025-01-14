use crate::{EnvVar, ServiceUtil, ServiceUtilError};
use std::process::Command;
use wait_utils::WaitStrategy;

impl ServiceUtil {
    pub(crate) async fn start(
        &self,
        program: String,
        wait_strategy: &WaitStrategy,
        env_var: Option<EnvVar>,
    ) -> Result<(), ServiceUtilError> {
        // Set the program to be executable
        Command::new("chmod")
            .arg("+x")
            .arg(&program)
            .output()
            .expect("Failed to set program to executable");
        let mut cmd = Command::new("chmod");
        cmd.arg("+x").arg(program.clone());
        cmd.output().expect("Failed to set program to executable");

        self.dbg_print("Constructing start command");
        let mut cmd = Command::new(program);
        cmd.arg("&");

        if env_var.is_some() {
            self.dbg_print("Setting environment variables");
            let (env, val) = env_var.unwrap().values();
            cmd.env(env, val);
        }

        self.dbg_print(&format!("Run start command: {:?}", &cmd));
        cmd.spawn().expect("Failed to run command");

        self.dbg_print("Waiting for service to start");
        self.wait_for_program(wait_strategy)
            .await
            .expect("Failed to wait for program");

        self.dbg_print("Service started");
        Ok(())
    }
}
