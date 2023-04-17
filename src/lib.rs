//! Nodeless API SDK
use std::str::FromStr;

use error::NodelessError;
use reqwest::{Client, Url};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

pub mod error;
pub mod paywall;
pub mod paywall_webhook;
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

mod serde_url {
    use serde::Deserialize;
    use url::Url;

    pub fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(url.as_ref())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let url_string = String::deserialize(deserializer)?;
        Url::parse(&url_string).map_err(serde::de::Error::custom)
    }
}

mod opt_serde_url {
    use super::*;

    pub fn serialize<S>(url: &Option<Url>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match url {
            Some(url) => serializer.serialize_str(url.as_ref()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let url_string: Option<String> = Option::deserialize(deserializer)?;
        match url_string {
            Some(s) => Url::parse(&s).map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

mod serde_timestamp {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.6fZ";

    pub fn serialize<S>(date: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = DateTime::<Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(*date, 0).unwrap(),
            Utc,
        );
        let s = format!("{}", datetime.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let datetime = DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)?;
        Ok(datetime.timestamp())
    }
}

mod opt_serde_timestamp {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.6fZ";

    pub fn serialize<S>(date: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => {
                let datetime = DateTime::<Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(*d, 0).unwrap(),
                    Utc,
                );
                let s = format!("{}", datetime.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let datetime = DateTime::parse_from_rfc3339(&s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(serde::de::Error::custom)?;
                Ok(Some(datetime.timestamp()))
            }
            None => Ok(None),
        }
    }
}
