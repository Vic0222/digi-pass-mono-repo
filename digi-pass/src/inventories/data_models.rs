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
    pub reserved_until: chrono::DateTime<Utc>,
    pub generate_inventory_id: Option<ObjectId>,
    pub concurrency_stamp: String,
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
