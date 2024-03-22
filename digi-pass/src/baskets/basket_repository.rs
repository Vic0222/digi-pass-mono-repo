use axum::async_trait;
use dyn_clone::DynClone;
use mongodb::{Client, Collection};
use super::data_models::Basket;


#[async_trait]
pub trait BasketRepository : DynClone + Send + Sync  {
    async fn add(&self, basket: Basket) -> anyhow::Result<Option<String>>;
}

dyn_clone::clone_trait_object!(BasketRepository);

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
        Ok(result.inserted_id.as_object_id().and_then(|oid| Some(oid.to_hex()) ))
    }
}