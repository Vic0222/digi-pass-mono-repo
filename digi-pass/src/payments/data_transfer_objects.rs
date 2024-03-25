use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Debug)]
pub struct  CheckoutRequest {
    pub basket_id: String,
}

#[derive(Serialize, Debug)]
pub struct  CheckoutResponse {
    checkout_id: String,
    checkout_url: String,
}



impl CheckoutResponse {
    pub fn new(checkout_id: String, checkout_url: String) -> Self {
        CheckoutResponse {
            checkout_id,
            checkout_url,
        }
    }
}

