use anyhow::{Result, bail};
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use serde::Deserialize;

use crate::Response;

/// SwitchBot client.
pub struct SwitchBot {
    /// HTTP client.
    client: Client,
}

impl SwitchBot {
    pub fn new(token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, token.parse()?);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self { client })
    }

    pub fn new_from_env() -> Result<Self> {
        let token = match std::env::var("SWITCHBOT_TOKEN") {
            Ok(token) => token,
            Err(_) => bail!("Environment variable SWITCHBOT_TOKEN is not set."),
        };

        Self::new(&token)
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        Ok(self
            .client
            .get(format!("https://api.switch-bot.com/{}", path))
            .send()
            .await?
            .json::<Response<T>>()
            .await?
            .body)
    }
}
