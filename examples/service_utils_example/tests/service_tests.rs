use serde::{Deserialize, Serialize};
use service_utils::{ServiceStartConfig, ServiceUtil, WaitStrategy};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GreetResponse {
    pub message: String,
}

// Bazel copied the binary into the tests directory
const ROOT_PATH: &str = "examples/service_utils_example/tests";
// Bazel renamed the binary to "service"
const PROGRAM: &str = "service";
const BASE_URL: &str = "http://localhost:4242/";

#[tokio::test]
async fn test_service() {
    println!(" Build service utils");
    let res = ServiceUtil::with_debug(ROOT_PATH, vec![PROGRAM]).await;
    // dbg!(&res);
    assert!(res.is_ok());
    let util = res.unwrap();

    println!("Start service");
    let res = util
        .start_service_from_config(get_service_start_config())
        .await;
    // dbg!(&res);
    assert!(res.is_ok());

    println!("Test service");
    let url = format!("{BASE_URL}hello");

    let resp = reqwest::get(url).await
        .expect("Failed to send request")
        .json::<GreetResponse>()
        .await;

    // dbg!(&resp);
    assert!(resp.is_ok());
    let resp = resp.unwrap();

    println!("{resp:#?}");
    assert_eq!(resp.message, "Hello world!");
}


fn get_service_wait_strategy(port: u16) -> WaitStrategy {
    let url = format!("http://localhost:{port}/health");
    let timeout_in_secs = 10;
    WaitStrategy::WaitForHttpHealthCheck(url, timeout_in_secs)
}

fn get_service_start_config() -> ServiceStartConfig {
    ServiceStartConfig::builder()
        .program(PROGRAM)
        .wait_strategy(get_service_wait_strategy(4242))
        .build()
}