use crate::utils::setup::{init, TestSetup};

async fn test_shortening<T>(setup: &TestSetup<T>, base: Option<&str>) -> serde_json::Value {
    let response = setup.client.post_shorten_link(&json!({ "key": "value" }).to_string(), base, None).await.unwrap();

    assert!(response.status().is_success());
    let response_json: serde_json::Value = response.json().await.unwrap();
    assert_eq!(json!({"shortened": format!("{}c0b07b92e91d6d4e", base.unwrap_or(setup.app_config.application.short_url_base.as_str())) }), response_json);
    response_json
}

#[tokio::test]
async fn shorten_works() {
    let setup  = init().await;

    // Test link shortening with default base url
    test_shortening(&setup, None).await;

    // Test link shortening with custom base url
    let response_json = test_shortening(&setup, Some("http://example.com/")).await;

    // Get original msg and compare
    let shortened = response_json["shortened"].as_str().unwrap();
    let url = url::Url::parse(&shortened).unwrap();
    let hash = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap().first().unwrap().to_string();

    let response = setup.client.get_oob_msg(&hash).await.unwrap();
    assert!(response.status().is_success());
    let response_json: serde_json::Value = response.json().await.unwrap();
    assert_eq!(json!({"key": "value" }), response_json);

    // Test redirect
    let response = setup.client.get_long_url(&hash).await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::PERMANENT_REDIRECT);
    let location_header = response.headers().get("location").unwrap();
    assert_eq!("didcomm://example.com/eyJrZXkiOiJ2YWx1ZSJ9", location_header);
}
