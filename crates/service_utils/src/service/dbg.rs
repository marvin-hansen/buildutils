/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use crate::ServiceUtil;

impl ServiceUtil {
    pub fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ServiceUtil]: {s}");
        }
    }
}
