use crate::types::health::Health;

pub(crate) async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = Health::ok();
    Ok(warp::reply::json(&result))
}
