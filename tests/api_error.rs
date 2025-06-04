#[tokio::test]
async fn test_api_error() {
    let switchbot = SwitchBot::new();
    switchbot.get("v1.1/invalid_endpoint").await
}
