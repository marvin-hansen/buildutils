/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

pub(crate) fn print_start_header_simple(service_name: &str, service_addr: &str) {
    println!();
    println!("||  {}  ||", service_name);
    println!("==========================================");
    println!("Service on endpoint: {}", service_addr);
    println!("==========================================");
    println!();
}
