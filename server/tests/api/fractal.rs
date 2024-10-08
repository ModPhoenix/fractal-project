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
        fractal {
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
}

#[tokio::test]
async fn test_create_fractal_mutation() {
    // Arrange
    let address = spawn_app().await;
    let client = Client::new();

    // First, get the Root fractal ID
    let root_query = r#"
    query {
        fractal {
            id
        }
    }
    "#;

    let root_response = client
        .post(&format!("{}", &address))
        .header("Content-Type", "application/json")
        .body(json!({"query": root_query}).to_string())
        .send()
        .await
        .expect("Failed to execute root query request.");

    let root_body = root_response.json::<serde_json::Value>().await.unwrap();
    let root_id = root_body["data"]["fractal"]["id"].as_str().unwrap();

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
            "parentId": root_id,
            "contextIds": []
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

    dbg!(&body);

    // Check if there are no errors
    assert!(body.get("errors").is_none());

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

#[tokio::test]
async fn test_fractal_name_uniqueness() {
    // Arrange
    let address = spawn_app().await;
    let client = Client::new();

    // First, get the Root fractal ID
    let root_query = r#"
    query {
        fractal {
            id
        }
    }
    "#;

    let root_response = client
        .post(&format!("{}", &address))
        .header("Content-Type", "application/json")
        .body(json!({"query": root_query}).to_string())
        .send()
        .await
        .expect("Failed to execute root query request.");

    let root_body = root_response.json::<serde_json::Value>().await.unwrap();
    let root_id = root_body["data"]["root"]["id"].as_str().unwrap();

    // GraphQL mutation
    let mutation = r#"
        mutation ($input: CreateFractalInput!) {
            createFractal(input: $input) {
                id
                name
            }
        }
    "#;

    let variables = json!({
        "input": {
            "name": "Unique Fractal",
            "parentId": root_id,
            "contextIds": []
        }
    });

    // Act - Create first fractal
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

    // Assert - First creation should succeed
    assert!(response.status().is_success());
    let body = response.json::<serde_json::Value>().await.unwrap();
    assert!(body.get("data").is_some());
    let created_fractal = body["data"]["createFractal"].as_object().unwrap();
    assert_eq!(created_fractal["name"], "Unique Fractal");

    // Act - Attempt to create second fractal with the same name
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

    // Assert - Second creation should fail
    assert!(response.status().is_success()); // GraphQL always returns 200 OK
    let body = response.json::<serde_json::Value>().await.unwrap();

    // Check for the presence of errors
    assert!(body.get("errors").is_some());
    let errors = body["errors"].as_array().unwrap();
    assert!(!errors.is_empty());

    // Check the error message
    let error_message = errors[0]["message"].as_str().unwrap();
    assert!(error_message.contains("already exists"));
}
