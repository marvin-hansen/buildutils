/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

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
        let program_args = service_start_config.program_args().to_owned();
        let wait_strategy = service_start_config.wait_strategy().to_owned();
        let env_vars = service_start_config.env_vars().to_owned();

        // Start the service
        self.start(program,program_args, env_vars, wait_strategy).await
    }

    pub(crate) async fn start(
        &self,
        program: &str,
        program_args: Option<Vec< & str,>>,
        env_vars: Option<Vec<(String, String)>>,
        wait_strategy: WaitStrategy,
    ) -> Result<(), ServiceUtilError> {
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

        self.dbg_print(" Set the program to be executable");
        Command::new("chmod")
            .arg("+x")
            .arg(&bin)
            .output()
            .expect("Failed to set program to executable");

        self.dbg_print("Constructing start command");
        let mut cmd = Command::new(bin);

        if env_vars.is_some() {
            self.dbg_print("Setting environment variables");
            let env_vars = env_vars.unwrap();

            // Add environment variables
            cmd.envs(env_vars);
        }

        if program_args.is_some() {
            self.dbg_print("Setting program arguments");
            let program_args = program_args.unwrap();

            // Add program arguments
            cmd.args(program_args);
        }

        self.dbg_print(&format!("Run start command: {:?}", &cmd));
        cmd.spawn().expect("Failed to run command");

        self.dbg_print("Waiting for service to start");
        self.wait_for_program(&wait_strategy)
            .await
            .expect("Failed to wait for program");

        self.dbg_print("Service started");
        Ok(())
    }
}
