/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use warp::Filter;

mod handlers;
mod types;
mod utils;

const PORT: u16 = 4242;

#[tokio::main]
async fn main() {
    // Add health check
    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(handlers::health_handler);

    // Add greater service
    let greet = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::end())
        .and_then(handlers::hello_handler);

    // Construct routes
    let routes = health_check.or(greet);

    // Show banner & start service
    utils::print_start_header_simple("Sample Service", "0.0.0.0:4242/");
    warp::serve(routes).run(([0, 0, 0, 0], PORT)).await;
}
