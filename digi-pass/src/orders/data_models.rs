use bson::oid::ObjectId;

pub struct OrderTransaction {
    pub id: String,
    pub order_id: String,
    pub basket_id: String,
    pub items : Vec<OrderTransactionItem>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct OrderTransactionItem {
    pub id: String,
    pub order_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}