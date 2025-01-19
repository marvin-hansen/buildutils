/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use crate::ServiceUtil;
use std::fmt::{Display, Formatter};

impl Display for ServiceUtil {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUtil {{ debug mode: {} }}", &self.dbg)
    }
}
