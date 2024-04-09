use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Basket{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub valid_until: DateTime<Utc>,
    pub basket_items: Vec<BasketItem>,
}


impl Basket {
    pub fn new(valid_until: DateTime<Utc>, basket_items: Vec<BasketItem>) -> Self {
        Basket {
            id: None,
            valid_until,
            basket_items,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct BasketItem{
    pub basketed_inventories: Vec<BasketedInventory>,
    pub price: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BasketedInventory{
    pub event_id: String,
    pub name: String,
    pub inventory_id: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub reserved_until: DateTime<Utc>,
}

impl BasketedInventory {
    
    pub fn new(event_id: String, name: String, inventory_id: String, reserved_until: DateTime<Utc>) -> Self {
        BasketedInventory {
            event_id,
            name,
            inventory_id,
            reserved_until
        }
    }
}