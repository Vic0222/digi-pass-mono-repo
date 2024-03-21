use axum::async_trait;
use dyn_clone::DynClone;
use mongodb::{bson, options::FindOptions, Client, Database};
use serde::{Deserialize, Serialize};
use super::data_models::Basket;


#[async_trait]
pub trait BasketRepository : DynClone + Send + Sync  {
    async fn add(&self, basket: Basket) -> anyhow::Result<Option<String>>;
}

dyn_clone::clone_trait_object!(BasketRepository);

#[derive(Clone)]
pub struct  MongoDbBasketRepository {
    pub client: Client,
    pub database: Database,
    pub collection: String,
}

impl MongoDbBasketRepository {
    pub fn new(client: Client, database: Database, collection: String) -> Self {
        MongoDbBasketRepository {
            client,
            database,
            collection
        }
    }
    
}

#[async_trait]
impl BasketRepository for MongoDbBasketRepository{
    async fn add(&self, basket: Basket) -> anyhow::Result<Option<String>>{

        let collection = self.database.collection::<Basket>(&self.collection);
        let result = collection.insert_one(basket, None).await?;
        Ok(result.inserted_id.as_object_id().and_then(|oid| Some(oid.to_hex()) ))
    }
}