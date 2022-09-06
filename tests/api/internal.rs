/*
 * Copyright 2022 ABSA Group Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::time::Duration;

use crate::utils::setup::{init, TestSetup};

async fn shorten_message(
    setup: &TestSetup,
    base: Option<&str>,
    expire_in_secs: Option<u32>,
) -> serde_json::Value {
    let response = setup
        .client
        .post_shorten_link(&json!({ "key": "value" }).to_string(), base, expire_in_secs)
        .await
        .unwrap();

    assert!(response.status().is_success());
    let response_json: serde_json::Value = response.json().await.unwrap();
    assert_eq!(
        json!({
            "shortened":
                format!(
                    "{}c0b07b92e91d6d4e",
                    base.unwrap_or(setup.app_config.application.short_url_base.as_str())
                )
        }),
        response_json
    );
    response_json
}

async fn verify_shortened_message(setup: &TestSetup, response_json: serde_json::Value) -> String {
    let shortened = response_json["shortened"].as_str().unwrap();
    let url = url::Url::parse(&shortened).unwrap();
    let hash = url
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap()
        .first()
        .unwrap()
        .to_string();

    let response = setup.client.get_oob_msg(&hash).await.unwrap();
    assert!(response.status().is_success());
    let response_json: serde_json::Value = response.json().await.unwrap();
    assert_eq!(json!({"key": "value" }), response_json);
    hash
}

async fn verify_redirect(setup: &TestSetup, hash: &str) {
    let response = setup.client.get_long_url(hash).await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::PERMANENT_REDIRECT);
    let location_header = response.headers().get("location").unwrap();
    assert_eq!(
        "didcomm://example.com?oob=eyJrZXkiOiJ2YWx1ZSJ9",
        location_header
    );
}

async fn verify_not_found(setup: &TestSetup, response_json: serde_json::Value) {
    let shortened = response_json["shortened"].as_str().unwrap();
    let url = url::Url::parse(&shortened).unwrap();
    let hash = url
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap()
        .first()
        .unwrap()
        .to_string();

    let response = setup.client.get_oob_msg(&hash).await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn shorten_works_default() {
    let setup = init().await;

    let response_json = shorten_message(&setup, None, None).await;
    let hash = verify_shortened_message(&setup, response_json).await;
    verify_redirect(&setup, &hash).await;
}

#[tokio::test]
async fn shorten_works_custom_base_url() {
    let setup = init().await;

    let response_json = shorten_message(&setup, Some("http://example.com/"), None).await;
    let hash = verify_shortened_message(&setup, response_json).await;
    verify_redirect(&setup, &hash).await;
}

#[tokio::test]
async fn shorten_works_custom_expiration() {
    let setup = init().await;
    let response_json = shorten_message(&setup, Some("http://example.com/"), Some(1)).await;
    tokio::time::sleep(Duration::new(2, 0)).await;
    verify_not_found(&setup, response_json).await;
}
