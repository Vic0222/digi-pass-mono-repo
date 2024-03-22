use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Basket{
    pub basket_items: Vec<BasketItem>,
}

#[derive(Serialize)]
pub struct BasketItem{
    pub basketed_inventories: Vec<BasketedInventory>
}

#[derive(Serialize)]
pub struct BasketedInventory{
    event_id: String,
    inventory_id: String,
    reserved_until: DateTime<Utc>,
    price: i32,
}

impl BasketedInventory {
    
    pub fn new(event_id: String, inventory_id: String, reserved_until: DateTime<Utc>, price: i32) -> Self {
        BasketedInventory {
            event_id,
            inventory_id,
            reserved_until,
            price
        }
    }
}