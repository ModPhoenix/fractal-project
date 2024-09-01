use server::data::{create_db, init_database};

pub async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port.");
    let port = listener
        .local_addr()
        .expect("Failed to get local address.")
        .port();

    let db = create_db("./test_db").expect("Failed to create database");
    // Create a new scope for database initialization
    {
        let conn = server::data::create_connection(&db).expect("Failed to create connection.");
        init_database(&conn).expect("Failed to initialize database.");
    } // conn is dropped here

    let server = server::run(listener, db).expect("Failed to create a server");

    let _ = tokio::spawn(async {
        server.await.expect("Server failed to start.");
    });

    format!("http://127.0.0.1:{}", port)
}
