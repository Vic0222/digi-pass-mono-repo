use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OrderTransaction {
    pub id: String,
    pub order_id: String,
    pub r#type: String,
    pub basket_id: Option<String>,
    pub items : Vec<OrderTransactionItem>,
    pub payments: Vec<OrderTransactionPayment>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct OrderTransactionPayment {
    pub id: String,
    pub payment_id: String,
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
    pub id: String,
    pub inventory_id: String,
    pub event_id: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}