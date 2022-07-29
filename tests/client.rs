pub struct ClientConfig {
    pub host: String,
    pub port: u16
}

#[allow(dead_code)]
pub struct Client {
    client: reqwest::Client,
    config: ClientConfig,
    base_url: String
}

impl Client {
    pub fn build(config: ClientConfig) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let base_url = format!("http://{}:{}", config.host, config.port);
        Ok(Self { client, config, base_url})

    }

    pub async fn get_healthcheck(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/health", self.base_url);
        self.client
            .get(url)
            .send()
            .await
    }

    pub async fn post_shorten_link(&self, msg: &str, base_url: Option<&str>, expire_in_secs: Option<u32>) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/internal/shorten-link", self.base_url);
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
        let url = format!("{}/{}", self.base_url, msg_hash);
        self.client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
    }

    pub async fn get_long_url(&self, msg_hash: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, msg_hash);
        self.client
            .get(url)
            .send()
            .await
    }
}
