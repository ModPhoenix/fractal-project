use kuzu::{Database, SystemConfig};
use reqwest::Response;
use serde_json::json;
use server::data::{create_fractal_raw, init_database, FRACTAL_ROOT_ID};

pub async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port.");
    let port = listener
        .local_addr()
        .expect("Failed to get local address.")
        .port();

    let db = Database::new(":memory:", SystemConfig::default()).expect("Failed to create database");
    // Create a new scope for database initialization
    {
        let conn = server::data::create_connection(&db).expect("Failed to create connection.");
        init_database(&conn).expect("Failed to initialize database.");
        create_fractal_raw(&conn, "Root", None, None, Some(FRACTAL_ROOT_ID))
            .expect("Failed to create Root fractal.");
    } // conn is dropped here

    let server = server::run(listener, db).expect("Failed to create a server");

    let _ = tokio::spawn(async {
        server.await.expect("Server failed to start.");
    });

    format!("http://127.0.0.1:{}", port)
}

pub async fn create_fractal(
    client: &reqwest::Client,
    address: &str,
    name: &str,
    parent_id: &str,
    context_ids: Vec<&str>,
) -> Response {
    let mutation = r#"
        mutation ($input: CreateFractalInput!) {
            createFractal(input: $input) {
                id
                name
                children {
                    id
                    name
                }
                contexts {
                    id
                    name
                }
                parents {
                    id
                    name
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "name": name,
            "parentId": parent_id,
            "contextIds": context_ids
        }
    });

    let response = client
        .post(&format!("{}", address))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "query": mutation,
                "variables": variables,
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute create fractal mutation request.");

    response
}
