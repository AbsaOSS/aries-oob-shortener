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

pub struct ClientConfig {
    pub host: String,
    pub port_internal: u16,
    pub port_external: u16,
}

#[allow(dead_code)]
pub struct Client {
    client: reqwest::Client,
    config: ClientConfig,
    base_url_internal: String,
    base_url_external: String,
}

impl Client {
    pub fn build(config: ClientConfig) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let base_url_internal = format!("http://{}:{}", config.host, config.port_internal);
        let base_url_external = format!("http://{}:{}", config.host, config.port_external);
        Ok(Self {
            client,
            config,
            base_url_internal,
            base_url_external,
        })
    }

    pub async fn get_healthcheck_internal(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/health", self.base_url_internal);
        self.client.get(url).send().await
    }

    pub async fn get_healthcheck_external(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/health", self.base_url_external);
        self.client.get(url).send().await
    }

    pub async fn post_shorten_link(
        &self,
        msg: &str,
        base_url: Option<&str>,
        expire_in_secs: Option<u32>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/internal/shorten-link", self.base_url_internal);
        self.client
            .post(url)
            .json(&serde_json::json!({
                "msg": msg,
                "base_url": base_url,
                "expire_in_secs": expire_in_secs
            }))
            .send()
            .await
    }

    pub async fn get_oob_msg(&self, msg_hash: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.base_url_external, msg_hash);
        self.client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
    }

    pub async fn get_long_url(&self, msg_hash: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.base_url_external, msg_hash);
        self.client.get(url).send().await
    }
}
