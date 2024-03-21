use serde::Serialize;

pub struct CreateBasketRequest {
    pub add_basket_item_request: Vec<AddBasketItemRequest>,
}

pub struct AddBasketItemRequest {
    pub event_id: String,
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct CreateBasketResult {
    pub basket_id: String,
}