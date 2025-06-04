use anyhow::{Context, Result, bail};
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use serde::{Deserialize, Serialize};

/// SwitchBot client.
pub struct SwitchBot {
    /// HTTP client.
    client: Client,
}

impl SwitchBot {
    /// Create a new SwitchBot client from environment variables.
    pub fn new_from_env() -> Result<Self> {
        let token = match std::env::var("SWITCHBOT_TOKEN") {
            Ok(token) => token,
            Err(_) => bail!("Environment variable SWITCHBOT_TOKEN is not set."),
        };

        Self::new_from_token(&token)
    }

    /// Create a new SwitchBot client using the given token.
    pub fn new_from_token(token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, token.parse()?);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self { client })
    }

    /// Issue HTTP GET request and parse response.
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        let url = format!("https://api.switch-bot.com/{}", path);
        let res = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("failed to GET {}", url))?
            .text()
            .await
            .with_context(|| format!("failed to read {}", url))?;
        let res = serde_json::from_str::<ApiResponse<T>>(&res)
            .with_context(|| format!("failed to parse SwitchBot API response:\n{}", res))?;

        // println!("{:?}", res);

        Ok(res.body)
    }
}

/// A response from the SwitchBot API.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct ApiResponse<T> {
    /// Response body.
    body: T,
}

/// An error returned by the SwitchBot API.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct ApiError {
    /// Error message.
    message: String,

    /// Response ID.
    request_id: String,
}
