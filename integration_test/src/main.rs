//! Integration test for nodeless api

#![deny(unused)]

use std::str::FromStr;

use nodeless_rs::paywall::Paywall;
use nodeless_rs::store::{InvoiceRequest, InvoiceStatus};
use nodeless_rs::webhook::{CreateWebhook, WebHookType, WebhookEvent, WebhookStatus};
use nodeless_rs::Nodeless;
use std::env;
use url::Url;

#[tokio::main]
async fn main() {
    dotenvy::from_path("integration_test/.env").expect("Messed up dev env");
    let api_key = env::var("API_KEY").unwrap();

    let store_id = env::var("STORE_ID").unwrap();

    let nodeless = Nodeless::new(&api_key, None).unwrap();
    test_get_server_status(&nodeless).await;

    // Store
    test_get_stores(&nodeless).await;
    test_get_store(&nodeless, &store_id).await;

    // Store Invoice
    let invoice_id = test_create_store_invoice(&nodeless, &store_id).await;
    test_get_store_invoice(&nodeless, &store_id, &invoice_id).await;
    test_get_store_invoice_status(&nodeless, &store_id, &invoice_id).await;

    // Transaction
    let transaction_id = test_get_transactions(&nodeless).await;
    test_get_transaction(&nodeless, &transaction_id).await;

    // Paywall
    let paywall_id = test_create_paywall(&nodeless).await;
    test_get_paywall(&nodeless, &paywall_id).await;
    test_get_paywalls(&nodeless).await;
    test_update_paywall(&nodeless, &paywall_id).await;

    // Paywall request
    let paywall_request_id = test_create_paywall_request(&nodeless, &paywall_id).await;
    test_get_paywall_request(&nodeless, &paywall_id, &paywall_request_id).await;
    test_get_paywall_request_status(&nodeless, &paywall_id, &paywall_request_id).await;

    // Store web hooks
    let webhook_id = test_create_store_webhook(&nodeless, &store_id).await;
    test_get_store_webhooks(&nodeless, &store_id).await;
    test_get_store_webhook(&nodeless, &store_id, &webhook_id).await;
    test_update_store_webhook(&nodeless, &store_id, &webhook_id).await;
    test_get_store_webhooks(&nodeless, &store_id).await;
    test_delete_store_webhook(&nodeless, &store_id, &webhook_id).await;

    // Paywall webhooks
    let webhook_id = test_create_paywall_webhook(&nodeless, &paywall_id).await;
    test_get_paywall_webhooks(&nodeless, &paywall_id).await;
    test_get_paywall_webhook(&nodeless, &paywall_id, &webhook_id).await;
    test_update_paywall_webhook(&nodeless, &paywall_id, &webhook_id).await;
    test_get_paywall_webhooks(&nodeless, &paywall_id).await;
    test_delete_paywall_webhook(&nodeless, &paywall_id, &webhook_id).await;

    // Delete pay wall
    test_delete_paywall(&nodeless, &paywall_id).await;

    println!("Tests Passed")
}

async fn test_get_server_status(nodeless: &Nodeless) {
    let status = nodeless.get_server_status().await.unwrap();
    assert_eq!(status.code, 200);
}

async fn test_get_stores(nodeless: &Nodeless) {
    let stores = nodeless.get_stores().await.unwrap();
    assert!(!stores.is_empty());
}

async fn test_get_store(nodeless: &Nodeless, store_id: &str) {
    let store = nodeless.get_store(store_id).await.unwrap();
    assert_eq!(store.id, store_id);
}

async fn test_create_store_invoice(nodeless: &Nodeless, store_id: &str) -> String {
    let invoice_request = InvoiceRequest {
        amount: 21.21,
        currency: "USD".to_string(),
        buyer_email: "hi@nodeless.io".to_string(),
        redirect_url: Url::from_str("https://nodeless.io").unwrap(),
        metadata: None,
    };

    let invoice = nodeless
        .create_store_invoice(store_id, invoice_request.clone())
        .await
        .unwrap();

    assert_eq!(invoice_request.buyer_email, invoice.buyer_email);
    assert_eq!(invoice_request.redirect_url, invoice.redirect_url);
    assert_eq!(invoice_request.metadata, invoice.metadata);

    invoice.id.unwrap()
}

async fn test_get_store_invoice(nodeless: &Nodeless, store_id: &str, invoice_id: &str) {
    let invoice = nodeless
        .get_store_invoice(store_id, invoice_id)
        .await
        .unwrap();

    assert_eq!("hi@nodeless.io".to_string(), invoice.buyer_email);
    assert_eq!(
        Url::from_str("https://nodeless.io").unwrap(),
        invoice.redirect_url
    );
    assert_eq!(None, invoice.metadata);
}

async fn test_get_store_invoice_status(nodeless: &Nodeless, store_id: &str, invoice_id: &str) {
    let status = nodeless
        .get_store_invoice_status(store_id, invoice_id)
        .await
        .unwrap();
    assert_eq!(InvoiceStatus::New, status);
}

async fn test_get_transaction(nodeless: &Nodeless, id: &str) {
    let transaction = nodeless.get_transaction(id).await.unwrap();
    assert_eq!(transaction.id, id);
}

async fn test_get_transactions(nodeless: &Nodeless) -> String {
    let transactions = nodeless.get_transactions(false).await.unwrap();
    assert!(!transactions.is_empty());
    transactions[0].id.clone()
}

async fn test_create_paywall(nodeless: &Nodeless) -> String {
    let paywall = Paywall {
        name: Some("Helloworld".to_string()),
        type_: nodeless_rs::paywall::PaywallType::Redirect,
        price: 1042,
        settings: None,
        id: None,
        created_at: None,
        updated_at: None,
    };

    let wall = nodeless.create_paywall(paywall.clone()).await.unwrap();

    assert_eq!(paywall.name, wall.name);
    assert_eq!(paywall.type_, wall.type_);
    assert_eq!(paywall.price, wall.price);

    wall.id.unwrap()
}

async fn test_get_paywall(nodeless: &Nodeless, id: &str) {
    let paywall = nodeless.get_paywall(id).await.unwrap().unwrap();
    assert_eq!(paywall.id.unwrap(), id);
}

async fn test_get_paywalls(nodeless: &Nodeless) {
    let paywalls = nodeless.get_paywalls().await.unwrap();
    assert!(!paywalls.is_empty())
}

async fn test_update_paywall(nodeless: &Nodeless, id: &str) {
    let paywall = Paywall {
        name: Some("hiworld".to_string()),
        type_: nodeless_rs::paywall::PaywallType::Redirect,
        price: 2042,
        settings: None,
        id: None,
        created_at: None,
        updated_at: None,
    };
    nodeless.update_paywall(id, paywall.clone()).await.unwrap();
    let new_wall = nodeless.get_paywall(id).await.unwrap().unwrap();

    assert_eq!(new_wall.name, paywall.name);
    assert_eq!(new_wall.type_, paywall.type_);
}

async fn test_delete_paywall(nodeless: &Nodeless, id: &str) {
    nodeless.delete_paywall(id).await.unwrap();
}

async fn test_create_store_webhook(nodeless: &Nodeless, id: &str) -> String {
    let webhook = CreateWebhook {
        type_: nodeless_rs::webhook::WebHookType::Store,
        url: Url::from_str("https://nodless.io").unwrap(),
        events: vec![WebhookEvent::New],
        secret: "RjnNCIN9pRRMEIn3clq1shoHiXIej0XL".to_string(),
        status: nodeless_rs::webhook::WebhookStatus::Inactive,
    };

    nodeless
        .create_store_webhook(id, webhook)
        .await
        .unwrap()
        .id
        .unwrap()
}

async fn test_get_store_webhooks(nodeless: &Nodeless, store_id: &str) {
    let res = nodeless.get_store_webhooks(store_id).await.unwrap();
    assert!(!res.is_empty());
}

async fn test_get_store_webhook(nodeless: &Nodeless, store_id: &str, webhook_id: &str) {
    let res = nodeless
        .get_store_webhook(store_id, webhook_id)
        .await
        .unwrap();

    assert_eq!(webhook_id, res.id.unwrap());
}

async fn test_delete_store_webhook(nodeless: &Nodeless, store_id: &str, webhook_id: &str) {
    nodeless
        .delete_store_webhook(store_id, webhook_id)
        .await
        .unwrap();
}

async fn test_update_store_webhook(nodeless: &Nodeless, store_id: &str, webhook_id: &str) {
    let webhook = CreateWebhook {
        type_: WebHookType::Store,
        url: Url::from_str("https://utxo.one").unwrap(),
        events: vec![WebhookEvent::New],
        secret: "RinNCIN9pRRMEIn3clq1shoHiXIej0XK".to_string(),
        status: WebhookStatus::Active,
    };

    let res = nodeless
        .update_store_webhook(store_id, webhook_id, webhook.clone())
        .await
        .unwrap();
    assert_eq!(res.url.unwrap(), webhook.url);
}

async fn test_create_paywall_webhook(nodeless: &Nodeless, id: &str) -> String {
    let webhook = CreateWebhook {
        type_: nodeless_rs::webhook::WebHookType::Paywall,
        url: Url::from_str("https://nodless.io").unwrap(),
        events: vec![WebhookEvent::New],
        secret: "RjnNCIN9pRRMEIn3clq1shoHiXIej0XL".to_string(),
        status: nodeless_rs::webhook::WebhookStatus::Inactive,
    };

    nodeless
        .create_paywall_webhook(id, webhook)
        .await
        .unwrap()
        .id
        .unwrap()
}

async fn test_get_paywall_webhooks(nodeless: &Nodeless, paywall_id: &str) {
    let res = nodeless.get_paywall_webhooks(paywall_id).await.unwrap();
    assert!(!res.is_empty());
}

async fn test_get_paywall_webhook(nodeless: &Nodeless, paywall_id: &str, webhook_id: &str) {
    let res = nodeless
        .get_paywall_webhook(paywall_id, webhook_id)
        .await
        .unwrap();
    assert_eq!(webhook_id, res.id.unwrap());
}

async fn test_delete_paywall_webhook(nodeless: &Nodeless, paywall_id: &str, webhook_id: &str) {
    nodeless
        .delete_paywall_webhook(paywall_id, webhook_id)
        .await
        .unwrap();
}

async fn test_update_paywall_webhook(nodeless: &Nodeless, paywall_id: &str, webhook_id: &str) {
    let webhook = CreateWebhook {
        type_: WebHookType::Paywall,
        url: Url::from_str("https://utxo.one").unwrap(),
        events: vec![WebhookEvent::New],
        secret: "hjfusfsfg".to_string(),
        status: WebhookStatus::Active,
    };

    let res = nodeless
        .update_paywall_webhook(paywall_id, webhook_id, webhook.clone())
        .await
        .unwrap();
    assert_eq!(webhook.url, res.url.unwrap());
}

async fn test_create_paywall_request(nodeless: &Nodeless, paywall_id: &str) -> String {
    nodeless
        .create_paywall_request(paywall_id)
        .await
        .unwrap()
        .id
}

async fn test_get_paywall_request(nodeless: &Nodeless, paywall_id: &str, request_id: &str) {
    let req = nodeless
        .get_paywall_request(paywall_id, request_id)
        .await
        .unwrap();

    assert_eq!(req.id, request_id);
}

async fn test_get_paywall_request_status(nodeless: &Nodeless, paywall_id: &str, request_id: &str) {
    let _status = nodeless
        .get_paywall_request_status(paywall_id, request_id)
        .await
        .unwrap();
}
