//! Paywall Webhook

use crate::error::NodelessError;
use crate::webhook::{CreateWebhook, Webhook};
use crate::Nodeless;

impl Nodeless {
    /// Get Paywall Webhooks
    pub async fn get_paywall_webhooks(
        &self,
        paywall_id: &str,
    ) -> Result<Vec<Webhook>, NodelessError> {
        let url = self
            .base_url
            .join(&format!("api/v1/paywall/{}/webhook", paywall_id))?;

        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Get Paywall Webhook
    pub async fn get_paywall_webhook(
        &self,
        paywall_id: &str,
        webhook_id: &str,
    ) -> Result<Webhook, NodelessError> {
        let url = self.base_url.join(&format!(
            "api/v1/paywall/{}/webhook/{}",
            paywall_id, webhook_id
        ))?;

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

    /// Delete Paywall Webhook
    pub async fn delete_paywall_webhook(
        &self,
        paywall_id: &str,
        webhook_id: &str,
    ) -> Result<(), NodelessError> {
        let url = self.base_url.join(&format!(
            "api/v1/paywall/{}/webhook/{}",
            paywall_id, webhook_id
        ))?;

        let _res = self.make_delete(url).await.ok();
        Ok(())
    }

    /// Update Paywall Webhook
    pub async fn update_paywall_webhook(
        &self,
        paywall_id: &str,
        webhook_id: &str,
        webhook: CreateWebhook,
    ) -> Result<Webhook, NodelessError> {
        let url = self.base_url.join(&format!(
            "api/v1/paywall/{}/webhook/{}",
            paywall_id, webhook_id
        ))?;

        let res = self
            .make_put(url, Some(serde_json::to_value(webhook)?))
            .await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }
}
