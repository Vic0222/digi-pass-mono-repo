use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize, Debug)]
pub struct  CheckoutRequest {
    pub basket_id: String,
}

#[derive(Deserialize, Debug)]
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

