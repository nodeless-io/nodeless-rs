## Quickstart

```rust
use std::str::FromStr;

use nodeless_rs::paywall::Paywall;
use nodeless_rs::webhook::{CreateWebhook, WebHookType, Webhook, WebhookEvent, WebhookStatus};
use url::Url;

use nodeless_rs::store::InvoiceRequest;
use nodeless_rs::Nodeless;

#[tokio::main]
async fn main() {
    let api_key = "<api key>";

    let nodeless = Nodeless::new(api_key, None).unwrap();

    let _status = nodeless.get_server_status().await.unwrap();
}
```


## Status

Implementation status of full [API](https://nodeless.io/api-docs#) support:

### Paywall Requests
- [x] [Create a Paywall Request](https://nodeless.io/api-docs#paywall-requests-POSTapi-v1-paywall--id--request)
- [x] [Get a Paywall Request](https://nodeless.io/api-docs#paywall-requests-GETapi-v1-paywall--id--request--requestId-)
- [x] [Get a Paywall Request Status](https://nodeless.io/api-docs#paywall-requests-GETapi-v1-paywall--id--request--requestId--status)

### Paywall Webhooks
- [x] [Get Paywall Webhooks](https://nodeless.io/api-docs#paywall-webhooks-GETapi-v1-paywall--id--webhook)
- [x] [Create Paywall Webhook](https://nodeless.io/api-docs#paywall-webhooks-POSTapi-v1-paywall--id--webhook)
- [x] [Get Paywall Webhook](https://nodeless.io/api-docs#paywall-webhooks-GETapi-v1-paywall--id--webhook--webhookId-)
- [x] [Delete Paywall Webhook](https://nodeless.io/api-docs#paywall-webhooks-DELETEapi-v1-paywall--id--webhook--webhookId-)
- [x] [Update Paywall Webhook](https://nodeless.io/api-docs#paywall-webhooks-PUTapi-v1-paywall--id--webhook--webhookId-)

### Paywalls
- [x] [Get Paywalls](https://nodeless.io/api-docs#paywalls-GETapi-v1-paywall)
- [x] [Create Paywall](https://nodeless.io/api-docs#paywalls-POSTapi-v1-paywall)
- [x] [Get Paywall](https://nodeless.io/api-docs#paywalls-GETapi-v1-paywall--id-)
- [x] [Update Paywall](https://nodeless.io/api-docs#paywalls-PUTapi-v1-paywall--id-)
- [x] [Delete Paywall](https://nodeless.io/api-docs#paywalls-DELETEapi-v1-paywall--id-)

### Server Info
- [x] [Get API Status](https://nodeless.io/api-docs#server-info-GETapi-v1-status)

### Store Invoices
- [x] [Create Store Invoice](https://nodeless.io/api-docs#store-invoices-POSTapi-v1-store--id--invoice)
- [x] [Get Store Invoice](https://nodeless.io/api-docs#store-invoices-GETapi-v1-store--id--invoice--invoiceId-)
- [x] [Get Store Invoice Status](https://nodeless.io/api-docs#store-invoices-GETapi-v1-store--id--invoice--invoiceId--status)

### Store Webhooks
- [x] [Get Store Webhooks](https://nodeless.io/api-docs#store-webhooks-GETapi-v1-store--id--webhook)
- [x] [Create Store Webhook](https://nodeless.io/api-docs#store-webhooks-POSTapi-v1-store--id--webhook)
- [x] [Get Store Webhook](https://nodeless.io/api-docs#store-webhooks-GETapi-v1-store--id--webhook--webhookId-)
- [x] [Delete Store Webhook](https://nodeless.io/api-docs#store-webhooks-DELETEapi-v1-store--id--webhook--webhookId-)
- [x] [Update Store Webhook](https://nodeless.io/api-docs#store-webhooks-PUTapi-v1-store--id--webhook--webhookId-)

### Stores
- [x] [Get Stores](https://nodeless.io/api-docs#stores-GETapi-v1-store)
- [x] [Get Store](https://nodeless.io/api-docs#stores-GETapi-v1-store--id-)

### Transactions
- [x] [Get All Transactions](https://nodeless.io/api-docs#transactions-GETapi-v1-transaction)
- [x] [Get Transaction](https://nodeless.io/api-docs#transactions-GETapi-v1-transaction--id-)