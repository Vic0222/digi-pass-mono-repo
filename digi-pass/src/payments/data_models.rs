use bson::oid::ObjectId;

pub struct Payment {
    pub id: String,
    pub amount: i32,
    pub currency: String,
    pub provider: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub payment_type: String,
    pub checkout_data : Option<CheckoutData>
}

pub struct  CheckoutData {
    pub checkout_id: String,
    pub checkout_url: String,
}


impl CheckoutData {
    pub fn new(checkout_id: String, checkout_url: String) -> Self {
        CheckoutData {
            checkout_id,
            checkout_url,
        }
    }
}

