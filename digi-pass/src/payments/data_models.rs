use bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Payment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub concurrency_stamp: String,
    pub basket_id: Option<ObjectId>,
    pub amount: i32,
    pub currency: String,
    pub provider: String,
    pub status: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub payment_type: String,
    pub checkout_data : Option<CheckoutData>
}


impl Payment {
    pub fn new(
        basket_id: Option<ObjectId>,
        amount: i32,
        currency: String,
        provider: String,
        status: String,
        created_at: chrono::DateTime<chrono::Utc>,
        payment_type: String,
        checkout_data: Option<CheckoutData>,
    ) -> Self {
        Payment {
            id: None,
            concurrency_stamp: ObjectId::new().to_hex(),
            basket_id,
            amount,
            currency,
            provider,
            status,
            created_at,
            payment_type,
            checkout_data,
        }
    }
}


impl From<&Payment> for Bson {
    fn from(value: &Payment) -> Self {
        bson::to_bson(value).unwrap()
    }
}





#[derive(Serialize, Deserialize, Clone)]
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

