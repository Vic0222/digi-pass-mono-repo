use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    #[serde(rename = "_id", )]
    pub id: ObjectId,
    pub status: String,
    pub concurrency_stamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderTransaction {
    #[serde(rename = "_id", )]
    pub id: ObjectId,
    pub order_id: ObjectId,
    pub r#type: String,
    pub basket_id: Option<String>,
    pub items : Vec<OrderTransactionItem>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderTransactionItem {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub price: i32,
    pub inventories: Vec<OrderTransactionItemInventory>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderTransactionItemInventory {
    pub inventory_id: ObjectId,
    pub event_id: ObjectId,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}