use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Basket{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub basket_items: Vec<BasketItem>,
}


impl Basket {
    pub fn new(basket_items: Vec<BasketItem>) -> Self {
        Basket {
            id: None,
            basket_items,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct BasketItem{
    pub basketed_inventories: Vec<BasketedInventory>
}

#[derive(Serialize, Deserialize)]
pub struct BasketedInventory{
    pub event_id: String,
    pub inventory_id: String,
    pub reserved_until: DateTime<Utc>,
    pub price: i32,
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