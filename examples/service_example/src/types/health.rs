/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Health {
    status: String,
}

impl Health {
    pub fn ok() -> Self {
        Self {
            status: String::from("OK"),
        }
    }
}
