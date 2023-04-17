//! Webhook Types

use serde::{Deserialize, Serialize};
use url::Url;

use crate::serde_utils::{opt_serde_timestamp, opt_serde_url, serde_url};

/// Event type where webhook is triggered
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum WebhookEvent {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "pending_confirmation")]
    PendingConfirmation,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "underpaid")]
    Underpaid,
    #[serde(rename = "overpaid")]
    Overpaid,
    #[serde(rename = "in_flight")]
    InFlight,
}

/// Type of webhook
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum WebHookType {
    #[serde(rename = "store")]
    Store,
    #[serde(rename = "donation_page")]
    DonationPage,
    #[serde(rename = "paywall")]
    Paywall,
    #[serde(rename = "inbox")]
    Inbox,
}

/// Status of webhook
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum WebhookStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

/// Create Webhook Information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateWebhook {
    #[serde(rename = "type")]
    pub type_: WebHookType,
    #[serde(with = "serde_url")]
    pub url: Url,
    pub events: Vec<WebhookEvent>,
    pub secret: String,
    pub status: WebhookStatus,
}

/// Webhook
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Webhook {
    pub id: Option<String>,
    pub secret: Option<String>,
    pub status: Option<WebhookStatus>,
    pub events: Option<Vec<WebhookEvent>>,
    #[serde(with = "opt_serde_url")]
    pub url: Option<Url>,
    #[serde(with = "opt_serde_timestamp")]
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(with = "opt_serde_timestamp")]
    #[serde(rename = "lastDeliveryAt")]
    pub last_delivery_at: Option<i64>,
}
