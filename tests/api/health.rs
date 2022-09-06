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

use crate::utils::setup::init;

#[tokio::test]
async fn health_check_internal_works() {
    let setup = init().await;
    let response = setup.client.get_healthcheck_internal().await.unwrap();

    assert!(response.status().is_success());
    assert_eq!(
        json!({"status": "success" }).to_string(),
        response.text().await.unwrap()
    );
}

#[tokio::test]
async fn health_check_external_works() {
    let setup = init().await;
    let response = setup.client.get_healthcheck_external().await.unwrap();

    assert!(response.status().is_success());
    assert_eq!(
        json!({"status": "success" }).to_string(),
        response.text().await.unwrap()
    );
}
