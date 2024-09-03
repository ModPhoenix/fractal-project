use crate::utils::spawn_app;
use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_fractal_query() {
    // Arrange
    let address = spawn_app().await;
    let client = Client::new();

    // GraphQL query
    let query = r#"
    query {
        fractal(name: "Root") {
            id
            name
            children {
                id
                name
            }
        }
    }
    "#;

    // Act
    let response = client
        .post(&format!("{}", &address))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "query": query,
            })
            .to_string(),
        )
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let body = response.json::<serde_json::Value>().await.unwrap();

    // Check if the response contains data
    assert!(body.get("data").is_some());

    // Check if the fractal data is present
    let fractal = body["data"]["fractal"].as_object().unwrap();
    assert_eq!(fractal["name"], "Root");

    // Check if children are present
    let children = fractal["children"].as_array().unwrap();
    assert_eq!(children.len(), 1);
}

#[tokio::test]
async fn test_create_fractal_mutation() {
    // Arrange
    let address = spawn_app().await;
    let client = Client::new();

    // GraphQL mutation
    let mutation = r#"
        mutation ($input: CreateFractalInput!) {
            createFractal(input: $input) {
                id
                name
                children {
                    id
                    name
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "name": "New Fractal",
            "parentId": null
        }
    });

    // Act
    let response = client
        .post(&format!("{}", &address))
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
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    let body = response.json::<serde_json::Value>().await.unwrap();

    // Check if the response contains data
    assert!(body.get("data").is_some());

    // Check if the created fractal data is present
    let created_fractal = body["data"]["createFractal"].as_object().unwrap();
    assert_eq!(created_fractal["name"], "New Fractal");
    assert!(created_fractal.get("id").is_some());

    // Check that children array is empty
    let children = created_fractal["children"].as_array().unwrap();
    assert!(children.is_empty());
}
