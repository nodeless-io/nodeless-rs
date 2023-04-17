//! Store Webhook

use crate::error::NodelessError;
use crate::webhook::{CreateWebhook, Webhook};
use crate::Nodeless;

impl Nodeless {
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
