use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::NodelessError;
use crate::webhook::{CreateWebhook, Webhook};
use crate::Nodeless;
use crate::{opt_serde_timestamp, serde_timestamp};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum PaywallType {
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "wp_article")]
    WPArticle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Paywall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: PaywallType,
    pub price: u64,
    pub settings: Option<HashMap<String, String>>,
    #[serde(with = "opt_serde_timestamp")]
    pub created_at: Option<i64>,
    #[serde(with = "opt_serde_timestamp")]
    pub updated_at: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaywallRequest {
    pub id: String,
    pub sats_amount: u64,
    pub status: String,
    pub metadata: Option<Vec<String>>,
    #[serde(with = "serde_timestamp")]
    pub created_at: i64,
    #[serde(with = "opt_serde_timestamp")]
    pub paid_at: Option<i64>,
    pub onchain_address: String,
    pub lightning_invoice: String,
    pub paywall: Option<Paywall>,
}

impl Nodeless {
    /// Create Paywall
    pub async fn create_paywall(&self, paywall: Paywall) -> Result<Paywall, NodelessError> {
        let url = self.base_url.join("api/v1/paywall")?;

        let res = self
            .make_post(url, Some(serde_json::to_value(paywall)?))
            .await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Get Paywalls
    pub async fn get_paywalls(&self) -> Result<Vec<Paywall>, NodelessError> {
        let url = self.base_url.join("api/v1/paywall")?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].clone())?)
    }

    /// Get Paywall
    pub async fn get_paywall(&self, id: &str) -> Result<Option<Paywall>, NodelessError> {
        let url = self.base_url.join(&format!("api/v1/paywall/{}", id))?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].clone())?)
    }

    /// Update Paywall
    pub async fn update_paywall(&self, id: &str, paywall: Paywall) -> Result<(), NodelessError> {
        let url = self.base_url.join(&format!("api/v1/paywall/{}", id))?;
        let _res = self
            .make_put(url, Some(serde_json::to_value(paywall)?))
            .await?;
        Ok(())
    }

    /// Delete Paywall
    pub async fn delete_paywall(&self, id: &str) -> Result<(), NodelessError> {
        let url = self.base_url.join(&format!("api/v1/paywall/{}", id))?;
        let _res = self.make_delete(url).await?;
        Ok(())
    }

    /// Get Paywall Webhook
    pub async fn get_paywall_webhooks(
        &self,
        store_id: &str,
    ) -> Result<Vec<Webhook>, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/paywall/{}/webhook", store_id))?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Get paywall webhook
    pub async fn get_paywall_webhook(
        &self,
        id: &str,
        webhook: &str,
    ) -> Result<Webhook, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/paywall/{}/webhook/{}", id, webhook))?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Create Paywall Webhook
    pub async fn create_paywall_webhook(
        &self,
        paywall_id: &str,
        webhook: CreateWebhook,
    ) -> Result<Webhook, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/paywall/{}/webhook", paywall_id))?;

        let res = self
            .make_post(url, Some(serde_json::to_value(webhook)?))
            .await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Delete Store Webhook
    pub async fn delete_paywall_webhook(
        &self,
        store_id: &str,
        webhook_id: &str,
    ) -> Result<(), NodelessError> {
        let url = self.base_url.join(&format!(
            "api/v1/paywall/{}/webhook/{}",
            store_id, webhook_id
        ))?;

        let _res = self.make_delete(url).await.ok();
        Ok(())
    }

    /// Create Store Webhook
    pub async fn update_paywall_webhook(
        &self,
        store_id: &str,
        webhook_id: &str,
        webhook: CreateWebhook,
    ) -> Result<Webhook, NodelessError> {
        let url = self.base_url.join(&format!(
            "api/v1/paywall/{}/webhook/{}",
            store_id, webhook_id
        ))?;

        let res = self
            .make_put(url, Some(serde_json::to_value(webhook)?))
            .await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Create Paywall Request
    pub async fn create_paywall_request(&self, id: &str) -> Result<PaywallRequest, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/paywall/{}/request", id))?;

        let res = self.make_post(url, None).await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Get a Paywall Request
    pub async fn get_paywall_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<PaywallRequest, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/paywall/{id}/request/{request_id}"))?;

        let res = &self.make_get(url).await?["data"];

        Ok(serde_json::from_value(res.to_owned())?)
    }

    /// Get Paywall response
    pub async fn get_paywall_request_status(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<String, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/paywall/{id}/request/{request_id}/status"))?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["status"].to_owned())?)
    }
}
