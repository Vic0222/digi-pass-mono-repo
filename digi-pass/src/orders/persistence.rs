use axum::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection};

use super::data_models::OrderTransaction;


#[async_trait]
pub trait OrderTransactionRepository: Send + Sync {
    async fn get(&self, id: ObjectId) -> anyhow::Result<Option<OrderTransaction>>;
    async fn save(&self, order_transaction: &OrderTransaction) -> anyhow::Result<()>;
}

pub struct MongoDbOrderTransactionRepository {
    pub client: Client,
    pub database: String,
    pub collection: String,
}

impl MongoDbOrderTransactionRepository {
    pub fn new(client: Client, database: String) -> Self {
        MongoDbOrderTransactionRepository {
            client,
            database,
            collection: "OrderTransactions".to_string(),
        }
    }

    fn get_collection(&self) -> Collection<OrderTransaction> {
        let database = self.client.database(&self.database[..]);
        database.collection::<OrderTransaction>(&self.collection[..])
    }
}

#[async_trait]
impl OrderTransactionRepository for MongoDbOrderTransactionRepository {
    
    async fn get(&self, id: ObjectId) -> anyhow::Result<Option<OrderTransaction>>{
        let order_transaction =  self.get_collection().find_one( doc! {
            "_id": id
        }, None).await?;
        Ok(order_transaction)
    }
    async fn save(&self, order_transaction: &OrderTransaction) -> anyhow::Result<()>{
        let _ = self.get_collection().insert_one(order_transaction, None).await?;
        Ok(())
    }
}