use std::str::FromStr;

use axum::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection};

use super::data_models::Payment;

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn save(&self, payment: &Payment) -> anyhow::Result<String>;
    async fn update(&self, payment: &mut Payment) -> anyhow::Result<()>;
    async fn find_one_by_checkout_id(&self, checkout_id: &str)
        -> anyhow::Result<Option<Payment>>;
    async fn find_by_basket_id(&self, basket_id: &str) -> anyhow::Result<Vec<Payment>>;
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
            database,
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
        let hex = result
            .inserted_id
            .as_object_id()
            .ok_or(anyhow::anyhow!("Failed to get object id"))?
            .to_hex();
        Ok(hex)
    }

    async fn update(&self, payment: &mut Payment) -> anyhow::Result<()>{
        let filter = doc! {"_id": payment.id.ok_or(anyhow::anyhow!("Failed to get object id"))?, "concurrency_stamp": &payment.concurrency_stamp};

        payment.concurrency_stamp = ObjectId::new().to_hex();

        let update = doc! {"$set": bson::to_bson(&payment)? };
        
        let update_result = self.get_collection().update_one(filter, update, None).await?;
        if update_result.matched_count == 0 {
            return  Err(anyhow::anyhow!("Failed to update payment"));
        }
        Ok(())
    }

    async fn find_one_by_checkout_id(
        &self,
        checkout_id: &str,
    ) -> anyhow::Result<Option<Payment>> {
        let result = self
            .get_collection()
            .find_one(
                Some(doc! {"checkout_data.checkout_id" : checkout_id}),
                None,
            )
            .await?;
        Ok(result)
    }

    async fn find_by_basket_id(&self, basket_id: &str) -> anyhow::Result<Vec<Payment>> {
        let mut result = self
            .get_collection()
            .find(
                Some(doc! {"basket_id" : ObjectId::from_str(basket_id)?}),
                None,
            )
            .await?;
        let mut payments = vec![];
        while result.advance().await? {
            payments.push(result.deserialize_current()?)
        }
        return Ok(payments);
    }
}
