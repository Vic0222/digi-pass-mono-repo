use std::str::FromStr;

use axum::async_trait;
use bson::oid::ObjectId;
use mongodb::{Client, Collection};
use super::data_models::Basket;


#[async_trait]
pub trait BasketRepository : Send + Sync  {
    async fn add(&self, basket: Basket) -> anyhow::Result<Option<String>>;
    async fn get (&self, id: &str) -> anyhow::Result<Option<Basket>>;
}

impl Clone for Box<dyn BasketRepository> {
    fn clone(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone)]
pub struct  MongoDbBasketRepository {
    pub client: Client,
    pub database: String,
    pub collection: String,
}

impl MongoDbBasketRepository {
    pub fn new(client: Client, database: String, collection: String) -> Self {
        MongoDbBasketRepository {
            client,
            database,
            collection
        }
    }

    fn get_collection(&self) -> Collection<Basket> {
        let database = self.client.database(&self.database[..]);
        let event_collection: Collection<Basket> = database.collection(&self.collection);
        event_collection
    }
    
}

#[async_trait]
impl BasketRepository for MongoDbBasketRepository{
    async fn add(&self, basket: Basket) -> anyhow::Result<Option<String>>{

        let collection = self.get_collection();
        let result = collection.insert_one(basket, None).await?;
        Ok(result.inserted_id.as_object_id().map(|oid| oid.to_hex() ))
    }

    async fn get (&self, id: &str) -> anyhow::Result<Option<Basket>> {
        let collection = self.get_collection();
        let filter = mongodb::bson::doc! {"_id": ObjectId::from_str(id)? };
        let result = collection.find_one(filter, None).await?;
        Ok(result)
    }
}