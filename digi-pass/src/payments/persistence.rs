use axum::async_trait;
use mongodb::{Client, Collection};

use super::data_models::Payment;

#[async_trait]
pub trait PaymentRepository : Send + Sync {
    async fn save(&self, payment: &Payment) -> anyhow::Result<String>;
}

pub struct MongoDbPaymentRepository {
    pub client: Client,
    pub database: String,
    pub collection: String,
}


impl MongoDbPaymentRepository {
    pub fn new(client: Client, database: String) -> Self {
        MongoDbPaymentRepository {
            client,
            database: database,
            collection: "Payments".to_string(),
        }
    }

    fn get_collection(&self) -> Collection<Payment> {
        let database = self.client.database(&self.database[..]);
        database.collection::<Payment>(&self.collection[..])
    }
}

#[async_trait]
impl PaymentRepository for MongoDbPaymentRepository {
    async fn save(&self, payment: &Payment) -> anyhow::Result<String> {
        
        let result = self.get_collection().insert_one(payment, None).await?;
        let hex = result.inserted_id.as_object_id()
            .ok_or(anyhow::anyhow!("Failed to get object id"))?.to_hex();
        Ok(hex)
    }
}