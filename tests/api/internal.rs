use crate::utils::setup::init;

#[tokio::test]
async fn shorten_works() {
    let setup  = init().await;
    let response = setup.client.post_shorten_link("http://example.com", &json!({ "key": "value" }).to_string(), None).await.unwrap();

    assert!(response.status().is_success());
    let response_json: serde_json::Value = response.json().await.unwrap();
    assert_eq!(json!({"shortened": "http://example.com/c0b07b92e91d6d4e" }), response_json);

    let shortened = response_json["shortened"].as_str().unwrap();
    let url = url::Url::parse(&shortened).unwrap();
    let hash = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap().first().unwrap().to_string();

    let response = setup.client.get_oob_msg(&hash).await.unwrap();
    assert!(response.status().is_success());
    let response_json: serde_json::Value = response.json().await.unwrap();
    assert_eq!(json!({"key": "value" }), response_json);

    let response = setup.client.get_long_url(&hash).await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::PERMANENT_REDIRECT);
    let location_header = response.headers().get("location").unwrap();
    assert_eq!("didcomm://example.com/eyJrZXkiOiJ2YWx1ZSJ9", location_header);
}
