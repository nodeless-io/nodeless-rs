//! Nodeless API SDK
//! Rust SDK for <https://nodeless.io/>
#![doc = include_str!("../README.md")]
use std::str::FromStr;

use error::NodelessError;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod error;
pub mod paywall;
pub mod paywall_webhook;
pub mod serde_utils;
pub mod store;
pub mod store_webhook;
pub mod transaction;
pub mod webhook;

#[derive(Debug, Clone)]
pub struct Nodeless {
    api_key: String,
    base_url: Url,
    client: Client,
}

impl Nodeless {
    /// Create nodeless client
    /// # Arguments
    /// * `api_key` - Nodeless api token
    /// * `url` - Optional Url of nodeless api
    ///
    /// # Example
    /// ```
    /// use nodeless_rs::Nodeless;
    /// let client = Nodeless::new(
    ///    "xxxxxxxxxxx",
    ///    None,
    /// ).unwrap();
    /// ```
    pub fn new(api_key: &str, api_url: Option<String>) -> Result<Self, NodelessError> {
        let base_url = match api_url {
            Some(url) => Url::from_str(&url)?,
            None => Url::from_str("https://nodeless.io")?,
        };

        let client = reqwest::Client::builder().build()?;

        Ok(Self {
            api_key: api_key.to_string(),
            base_url,
            client,
        })
    }

    async fn make_get(&self, url: Url) -> Result<Value, NodelessError> {
        Ok(self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<Value>()
            .await?)
    }

    async fn make_post(&self, url: Url, data: Option<Value>) -> Result<Value, NodelessError> {
        Ok(self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&data)
            .send()
            .await?
            .json::<Value>()
            .await?)
    }

    async fn make_put(&self, url: Url, data: Option<Value>) -> Result<Value, NodelessError> {
        let res = self
            .client
            .put(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&data)
            .send()
            .await?;
        let res = res.json::<Value>().await?;
        Ok(res)
    }

    async fn make_delete(&self, url: Url) -> Result<Value, NodelessError> {
        let res = self
            .client
            .delete(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .send()
            .await?;

        let res = res.json::<Value>().await?;
        Ok(res)
    }

    /// Get Server Status
    pub async fn get_server_status(&self) -> Result<ServerStatusResponse, NodelessError> {
        let url = self.base_url.join("api/v1/status")?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].clone())?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatusResponse {
    pub code: u32,
    pub status: String,
    pub node: String,
}
