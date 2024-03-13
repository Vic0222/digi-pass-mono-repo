use bson::oid::ObjectId;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub event_id: ObjectId,
    pub status: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_status_change: chrono::DateTime<Utc>,
    pub generate_inventory_id: Option<ObjectId>,
}


impl Inventory {
    pub fn new(event_id: ObjectId, status: String, last_status_change: chrono::DateTime<Utc>, generate_inventory_id: Option<ObjectId>) -> Self {
        Self {
            id: None,
            event_id,
            status,
            last_status_change,
            generate_inventory_id,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateInventory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub event_id: ObjectId,
    pub number_to_create: i32,
    pub status: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<Utc>,
}
