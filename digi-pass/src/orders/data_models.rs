use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OrderTransaction {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub order_id: ObjectId,
    pub r#type: String,
    pub basket_id: Option<String>,
    pub items : Vec<OrderTransactionItem>,
    pub payments: Vec<OrderTransactionPayment>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct OrderTransactionPayment {
    pub id: String,
    pub payment_id: ObjectId,
    pub amount: i32,
    pub currency: String,
    pub provider: String,
    pub status: String,
    pub payment_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct OrderTransactionItem {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub price: i32,
    pub inventories: Vec<OrderTransactionItemInventory>,
}

#[derive(Serialize, Deserialize)]
pub struct OrderTransactionItemInventory {
    pub inventory_id: ObjectId,
    pub event_id: ObjectId,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}