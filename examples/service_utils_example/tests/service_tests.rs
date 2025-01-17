use service_utils::WaitStrategy;

async fn get_service_wait_strategy(port: u16) -> WaitStrategy {
    let url = format!("http://localhost:{port}/health");
    let timeout_in_secs = 10;
    WaitStrategy::WaitForHttpHealthCheck(url, timeout_in_secs)
}

#[tokio::test]
async fn test_service() {

    assert!(true)
}