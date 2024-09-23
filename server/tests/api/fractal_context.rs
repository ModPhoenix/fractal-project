use assert_json_diff::assert_json_include;
use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

use crate::utils::{create_fractal, spawn_app};

#[tokio::test]
async fn test_fractal_context_mutation() {
    // Arrange
    let address = spawn_app().await;
    let client = Client::new();

    let root_id = Uuid::nil();

    let create_programing_fractal = create_fractal(
        &client,
        &address,
        "Programing",
        &root_id.to_string(),
        vec![],
    )
    .await;

    // Assert
    assert!(create_programing_fractal.status().is_success());

    let programing_body = create_programing_fractal
        .json::<serde_json::Value>()
        .await
        .unwrap();

    dbg!(&programing_body);

    // Check if there are no errors
    assert!(programing_body.get("errors").is_none());

    assert_json_include!(
        actual: &programing_body,
        expected: json!({
            "data": {
                "createFractal": {
                    "name": "Programing",
                    "parents": [{
                        "id": root_id,
                        "name": "Root",
                    }],
                    "children": [],
                    "contexts": [],
                }
            }
        }),
    );

    let create_programing_string_fractal = create_fractal(
        &client,
        &address,
        "String",
        programing_body["data"]["createFractal"]["id"]
            .as_str()
            .unwrap_or("")
            .into(),
        vec![],
    )
    .await;

    // Assert
    assert!(create_programing_string_fractal.status().is_success());
    let string_body = create_programing_string_fractal
        .json::<serde_json::Value>()
        .await
        .unwrap();

    dbg!(&string_body);

    // Check if there are no errors
    assert!(string_body.get("errors").is_none());
    assert_json_include!(
        actual: &string_body,
        expected: json!({
            "data": {
                "createFractal": {
                    "name": "String",
                    "parents": [{
                        "id": programing_body["data"]["createFractal"]["id"],
                        "name": "Programing",
                    }],
                    "children": [],
                    "contexts": [],
                }
            }
        }),
    );
}
