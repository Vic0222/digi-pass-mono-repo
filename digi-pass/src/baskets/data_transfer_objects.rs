use chrono::{DateTime, Utc};
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

#[derive(Serialize, Default, Debug)]
pub struct Basket {
    pub id: String,
    pub valid_until: DateTime<Utc>,
    pub basket_items: Vec<BasketItem>,
    pub total_price: i32,
}

#[derive(Serialize, Default, Debug)]
pub struct BasketItem{
    pub basketed_inventories: Vec<BasketedInventory>,
    pub total_price: i32,
}

#[derive(Serialize, Default, Debug)]
pub struct BasketedInventory{
    pub event_id: String,
    pub name: String,
    pub inventory_id: String,
    pub reserved_until: DateTime<Utc>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct PurchaseBasketRequest {
    #[validate(length(min = 1))]
    pub basket_id: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct  PurchaseBasketResult {
    pub order_id: String
}