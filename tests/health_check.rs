use std::net::TcpListener;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let address = spawn_app();
    
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");
    let port = listener.local_addr().expect("Failed to get local address").port();
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // start running in the background immediately, even if we don't await the returned JoinHandle
    // We have no use for it here, hence the non-binding let
    let app = z2p::run(listener).expect("Failed to run app");
    let _ = tokio::spawn(app);
    format!("http://127.0.0.1:{}", port)
}
