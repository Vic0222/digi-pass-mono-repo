pub mod maya_webhook;
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

pub struct PaymentView {
    pub id: String,
    pub concurrency_stamp: String,
    pub basket_id: String,
    pub amount: i32,
    pub currency: String,
    pub provider: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub payment_type: String,
}