use chrono::Utc;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Debug)]
pub struct GenerateInventory {
    #[validate(length(min = 1))]
    pub event_id: String,
    #[validate(range(min = 1))]
    pub number_to_create: i32,
}

#[derive(Serialize, Debug)]
pub struct GenerateInventoryResult {
    pub id: String,
    pub satus: String,
}


#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct CreateInventoryBatch {
    #[validate(length(min = 1))]
    pub event_id: String,
    #[validate(range(min = 1, max = 1000))]
    pub quantity: i32,

    pub generate_inventory_id: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct ReserveInventories {
    #[validate(length(min = 1))]
    pub event_id: String,
    #[validate(range(min = 1, max = 10))]
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReserveInventoriesResult {
    pub reserved_inventories: Vec<ReservedInventory>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReservedInventory{
    pub inventory_id: String,
    pub reserved_until: chrono::DateTime<Utc>,
}

impl  ReservedInventory {
    
    pub fn new(inventory_id: String, reserved_until: chrono::DateTime<Utc>) -> Self {
        ReservedInventory {
            inventory_id,
            reserved_until,
        }
    }

}