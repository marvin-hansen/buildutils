use crate::types::greet::*;

pub(crate) async fn hello_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = GreetResponse {
        message: "Hello world!".to_string(),
    };

    Ok(warp::reply::json(&result))
}
