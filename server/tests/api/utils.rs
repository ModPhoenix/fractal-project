pub async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port.");
    let port = listener
        .local_addr()
        .expect("Failed to get local address.")
        .port();

    let server = server::run(listener).expect("Failed to create a server");

    let _ = tokio::spawn(async {
        server.await.expect("Server failed to start.");
    });

    format!("http://127.0.0.1:{}", port)
}
