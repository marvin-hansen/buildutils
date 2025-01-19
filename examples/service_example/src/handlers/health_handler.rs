/*
 * SPDX-License-Identifier: MIT
 * Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
 */

use crate::types::health::Health;

pub(crate) async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = Health::ok();
    Ok(warp::reply::json(&result))
}
