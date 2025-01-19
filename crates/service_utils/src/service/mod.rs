/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

mod build;
mod dbg;
mod display;
mod getters;
mod start;
mod verify;
mod wait;

#[derive(Debug)]
pub struct ServiceUtil {
    dbg: bool,
    root_path: &'static str, // root_path basically remains constant after initialization
    binaries: Vec<&'static str>, // After verification, we're only reading from the Vec, thus lock-free
}
