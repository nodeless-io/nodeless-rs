use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;

use crate::error::NodelessError;
use crate::webhook::{CreateWebhook, Webhook};
use crate::Nodeless;
use crate::{opt_serde_timestamp, opt_serde_url, serde_timestamp, serde_url};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub id: String,
    pub name: String,
    #[serde(with = "opt_serde_url")]
    pub url: Option<Url>,
    pub email: Option<String>,
    #[serde(with = "serde_timestamp")]
    pub created_at: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRequest {
    pub amount: f64,
    pub currency: String,
    pub buyer_email: String,
    #[serde(with = "serde_url")]
    pub redirect_url: Url,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: Option<String>,
    #[serde(with = "opt_serde_url")]
    #[serde(rename = "checkoutLink")]
    pub checkout_link: Option<Url>,
    pub sats_amount: u64,
    pub status: InvoiceStatus,
    pub buyer_email: String,
    #[serde(with = "serde_url")]
    pub redirect_url: Url,
    pub metadata: Option<HashMap<String, String>>,
    #[serde(with = "serde_timestamp")]
    pub created_at: i64,
    #[serde(with = "opt_serde_timestamp")]
    pub paid_at: Option<i64>,
    pub onchain_address: String,
    pub lightning_invoice: String,
    pub store: Store,
    pub qr_codes: QrCodes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvoiceStatus {
    New,
    Paid,
    Expired,
    Unknown(String),
}

impl<'de> Deserialize<'de> for InvoiceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let status_str = String::deserialize(deserializer)?;
        match status_str.as_str() {
            "new" => Ok(InvoiceStatus::New),
            "paid" => Ok(InvoiceStatus::Paid),
            "expired" => Ok(InvoiceStatus::Expired),
            _ => Ok(InvoiceStatus::Unknown(status_str)),
        }
    }
}

impl Serialize for InvoiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InvoiceStatus::New => serializer.serialize_str("new"),
            InvoiceStatus::Paid => serializer.serialize_str("paid"),
            InvoiceStatus::Expired => serializer.serialize_str("expired"),
            InvoiceStatus::Unknown(status_str) => serializer.serialize_str(status_str),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QrCodes {
    pub unified: String,
    pub onchain: String,
    pub lightning: String,
}

impl Nodeless {
    /// Get Stores
    pub async fn get_stores(&self) -> Result<Vec<Store>, NodelessError> {
        let url = self.base_url.join("api/v1/store")?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].clone())?)
    }

    /// Get Store
    pub async fn get_store(&self, id: &str) -> Result<Store, NodelessError> {
        let url = self.base_url.join(&format!("api/v1/store/{}", id))?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].clone())?)
    }

    /// Create Store Invoice
    pub async fn create_store_invoice(
        &self,
        store_id: &str,
        invoice: InvoiceRequest,
    ) -> Result<Invoice, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/store/{}/invoice", store_id))?;

        let res = self
            .make_post(url, Some(serde_json::to_value(invoice)?))
            .await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Get Invoice
    pub async fn get_store_invoice(
        &self,
        store_id: &str,
        invoice_id: &str,
    ) -> Result<Invoice, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/store/{}/invoice/{}", store_id, invoice_id))?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Get Store Invoice Status
    pub async fn get_store_invoice_status(
        &self,
        store_id: &str,
        invoice_id: &str,
    ) -> Result<InvoiceStatus, NodelessError> {
        let url = self.base_url.join(&format!(
            "api/v1/store/{}/invoice/{}/status",
            store_id, invoice_id
        ))?;
        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["status"].to_owned())?)
    }

    /// Get Store Webhook
    pub async fn get_store_webhooks(&self, store_id: &str) -> Result<Vec<Webhook>, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/store/{}/webhook", store_id))?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Get Store Webhook
    pub async fn get_store_webhook(
        &self,
        store_id: &str,
        webhook_id: &str,
    ) -> Result<Webhook, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/store/{}/webhook/{}", store_id, webhook_id))?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Create Store Webhook
    pub async fn create_store_webhook(
        &self,
        store_id: &str,
        webhook: CreateWebhook,
    ) -> Result<Webhook, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/store/{}/webhook", store_id))?;

        let res = self
            .make_post(url, Some(serde_json::to_value(webhook)?))
            .await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Delete Store Webhook
    pub async fn delete_store_webhook(
        &self,
        store_id: &str,
        webhook_id: &str,
    ) -> Result<(), NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/store/{}/webhook/{}", store_id, webhook_id))?;

        let _res = self.make_delete(url).await.ok();
        Ok(())
    }

    /// Create Store Webhook
    pub async fn update_store_webhook(
        &self,
        store_id: &str,
        webhook_id: &str,
        webhook: CreateWebhook,
    ) -> Result<Webhook, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/store/{}/webhook/{}", store_id, webhook_id))?;

        let res = self
            .make_put(url, Some(serde_json::to_value(webhook)?))
            .await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }
}
