use crate::service::wait;
use crate::{ServiceStartConfig, ServiceUtil, ServiceUtilError};
use std::process::Command;
use wait_utils::WaitStrategy;

impl ServiceUtil {
    pub(crate) async fn start_config(
        &self,
        service_start_config: ServiceStartConfig,
    ) -> Result<(), ServiceUtilError> {
        // Extract parameters
        let program = service_start_config.program();
        let wait_strategy = service_start_config.wait_strategy().to_owned();
        let env_vars = service_start_config.env_vars().to_owned();

        // Start the service
        self.start(program, env_vars, wait_strategy).await
    }

    pub(crate) async fn start(
        &self,
        program: &str,
        env_vars: Option<Vec<String>>,
        wait_strategy: WaitStrategy,
    ) -> Result<(), ServiceUtilError> {
        // Obtain a write lock in the binary handlers hashmap
        let binary_handlers = &mut self
            .binary_handlers
            .write()
            .expect("Failed to obtain a write lock to binary handlers");

        // Check if program is already running i.e in binary_handlers
        if binary_handlers.contains_key(program) {
            return Err(ServiceUtilError::ServiceAlreadyRunning(program.to_owned()));
        }

        // Check if the program is in the binaries vector
        if !self.binaries().contains(&program) {
            return Err(ServiceUtilError::BinaryNotFound(format!("Binary has not been added to the ServiceUtil. \
             Please add the following binary to the set of programs when constructing the ServiceUtil: {}", program)));
        }

        // Check if the binary in the full path still exists
        let bin = format!("{}/{}", self.root_path(), program);
        if !std::path::Path::new(&bin).exists() {
            return Err(ServiceUtilError::BinaryNotFound(format!(
                "Program {} not found in path: {}",
                &program, &bin
            )));
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

            // Set optional environment variable if there are some.
            if env_vars.is_some() {
                // Unwrap the optional environment variable
                let add_args = env_vars.unwrap();

                // Add env variables
                cmd.arg("-e");
                cmd.args(add_args);
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
