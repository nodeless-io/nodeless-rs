//! Transactions
use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::NodelessError;
use crate::serde_utils::{opt_serde_timestamp, serde_timestamp};
use crate::Nodeless;

/// Transactable Type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactableType {
    Donation,
    Other(String),
}

impl<'de> Deserialize<'de> for TransactableType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let status_str = String::deserialize(deserializer)?;
        match status_str.as_str() {
            "Donation" => Ok(TransactableType::Donation),
            _ => Ok(TransactableType::Other(status_str)),
        }
    }
}

impl Serialize for TransactableType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TransactableType::Donation => serializer.serialize_str("Donation"),
            TransactableType::Other(status_str) => serializer.serialize_str(status_str),
        }
    }
}

/// Transaction Status
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionStatus {
    Settled,
    Other(String),
}

impl<'de> Deserialize<'de> for TransactionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let status_str = String::deserialize(deserializer)?;
        match status_str.as_str() {
            "settled" => Ok(TransactionStatus::Settled),
            _ => Ok(TransactionStatus::Other(status_str)),
        }
    }
}

impl Serialize for TransactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TransactionStatus::Settled => serializer.serialize_str("settled"),
            TransactionStatus::Other(status_str) => serializer.serialize_str(status_str),
        }
    }
}

/// Transactable
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transactable {
    id: u64,
    uuid: String,
    donation_page_id: Option<u64>,
    amount: u64,
    amount_paid: u64,
    name: Option<String>,
    message: Option<String>,
    status: String,
    #[serde(rename = "type")]
    type_: String,
    metadata: Option<HashMap<String, String>>,
    #[serde(with = "serde_timestamp")]
    created_at: i64,
    #[serde(with = "opt_serde_timestamp")]
    updated_at: Option<i64>,
    #[serde(with = "opt_serde_timestamp")]
    paid_at: Option<i64>,
}

/// Transaction
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub id: String,
    pub transactable_type: TransactableType,
    pub transactable: Transactable,
    pub amount: f64,
    #[serde(rename = "type")]
    pub type_: String,
    pub status: TransactionStatus,
    #[serde(with = "serde_timestamp")]
    pub created_at: i64,
    #[serde(with = "serde_timestamp")]
    pub updated_at: i64,
    pub is_fee: bool,
}
impl Nodeless {
    /// Get Transaction
    pub async fn get_transaction(&self, id: &str) -> Result<Transaction, NodelessError> {
        let url = self.base_url.join(&format!("api/v1/transaction/{}", id))?;
        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }

    /// Get Transactions
    pub async fn get_transactions(&self, is_fee: bool) -> Result<Vec<Transaction>, NodelessError> {
        let url = match is_fee {
            false => self.base_url.join("api/v1/transaction")?,
            true => self.base_url.join("api/v1/transaction?isFee=1")?,
        };
        let res = self.make_get(url).await?;
        Ok(serde_json::from_value(res["data"].to_owned())?)
    }
}
