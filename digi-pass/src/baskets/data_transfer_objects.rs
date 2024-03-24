use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateBasketRequest {
    pub add_basket_item_request: Vec<AddBasketItemRequest>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct AddBasketItemRequest {
    #[validate(length(min = 1))]
    pub event_id: String,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct CreateBasketResult {
    pub basket_id: String,
}