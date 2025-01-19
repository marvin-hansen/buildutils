/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use crate::ServiceUtilError;

pub(crate) fn verify_binary_exists(
    dbg: bool,
    root_path: &'static str,
    binaries: &Vec<&'static str>,
) -> Result<(), ServiceUtilError> {
    if dbg {
        println!("[ServiceUtil]: Verify all binaries in path: {root_path}");
    }

    for b in binaries {
        let path = format!("{root_path}/{b}");
        if dbg {
            println!("[VerifyBinary]: Checking if binary exists: {path}");
        }
        if !std::path::Path::new(&path).exists() {
            return Err(ServiceUtilError::BinaryNotFound(path.to_string()));
        }

        if dbg {
            println!("[VerifyBinary]: OK binary exists: {b}");
        }
    }

    Ok(())
}
