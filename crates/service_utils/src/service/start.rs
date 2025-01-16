use crate::service::wait;
use crate::{EnvVar, ServiceUtil, ServiceUtilError};
use std::process::Command;
use wait_utils::WaitStrategy;

impl ServiceUtil {
    pub(crate) async fn start(
        &self,
        program: &str,
        wait_strategy: WaitStrategy,
        env_var: Option<EnvVar>,
    ) -> Result<(), ServiceUtilError> {
        // Obtain a write lock in the binary handlers hashmap
        let binary_handlers = &mut self
            .binary_handlers()
            .write()
            .expect("Failed to write to binary handlers");

        // Check if program is already running i.e in binary_handlers
        if binary_handlers.contains_key(program) {
            return Err(ServiceUtilError::ServiceAlreadyRunning(program.to_owned()));
        }

        // Check if the program is in the binaries vector
        if !self.binaries().contains(&program) {
            return Err(ServiceUtilError::BinaryNotFound(program.to_owned()));
        }

        // Check if the binary in the full path still exists
        let bin = format!("{}/{}", self.root_path, program);
        if !std::path::Path::new(&bin).exists() {
            return Err(ServiceUtilError::BinaryNotFound(program.to_owned()));
        }

        // Start the program
        let dbg = self.dbg;
        let handle = tokio::spawn(async move {
            // Set the program to be executable
            Command::new("chmod")
                .arg("+x")
                .arg(&bin)
                .output()
                .expect("Failed to set program to executable");

            // Run the program
            let mut cmd = Command::new(bin);
            cmd.arg("&");

            // Set optional environment variable
            if env_var.is_some() {
                let (env, val) = env_var.unwrap().values();
                cmd.env(env, val);
            }

            // Spawn a new task to run the program
            cmd.spawn()
                .expect("Failed to run command")
                .wait()
                .expect("Failed to wait for command");

            // Wait for the program based on the wait strategy
            wait::wait_for_program(dbg, &wait_strategy)
                .await
                .expect("Failed to wait for program");
        });

        // Add the handler to the hashmap so it can be stopped later.
        binary_handlers.insert(program.to_owned(), handle);

        // the write lock will be dropped automatically when it goes out of scope.
        Ok(())
    }
}
