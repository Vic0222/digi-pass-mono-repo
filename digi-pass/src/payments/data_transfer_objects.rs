use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Debug)]
pub struct CheckoutRequest {
    #[validate(length(min = 1))]
    pub basket_id: String,
}

#[derive(Serialize, Debug)]
pub struct CheckoutResponse {
    pub checkout_id: String,
    pub checkout_url: String,
}

impl CheckoutResponse {
    pub fn new(checkout_id: String, checkout_url: String) -> Self {
        CheckoutResponse {
            checkout_id,
            checkout_url,
        }
    }
}

pub mod webhook {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Webhook {
        pub data: WebhookData,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WebhookData {
        pub id: String,

        #[serde(rename = "type")]
        pub data_type: String,

        pub attributes: Attributes,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Attributes {
        #[serde(rename = "type")]
        pub attributes_type: String,

        pub livemode: bool,

        pub data: AttributesData,

        pub pending_webhooks: i64,

        pub created_at: i64,

        pub updated_at: i64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AttributesData {
        pub id: String,

        #[serde(rename = "type")]
        pub data_type: String,
    }
}
