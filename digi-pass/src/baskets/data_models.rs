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
}

impl BasketedInventory {
    
    pub fn new(event_id: String, inventory_id: String, reserved_until: DateTime<Utc>) -> Self {
        BasketedInventory {
            event_id,
            inventory_id,
            reserved_until,
        }
    }
}