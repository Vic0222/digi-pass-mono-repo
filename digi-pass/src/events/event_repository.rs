
use std::str::FromStr;

use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::{options::FindOptions, Client, Collection};

use super::data_models::{self, Event};

#[async_trait]
pub trait EventRepository : Send + Sync  {
    async fn add(&self, event: data_models::Event) -> anyhow::Result<String>;

    async fn list(&self) -> anyhow::Result<Vec<Event>>;

    async fn get_event(&self, event_id: &String) -> anyhow::Result<Option<Event>>;
}

#[derive(Clone)]
pub struct MongoDbEventRepository {
    pub client: Client,
    pub database: String,
}

impl MongoDbEventRepository {
    pub fn new(client: Client, database: String) -> Self {
        MongoDbEventRepository {
            client,
            database
        }
    }

    fn get_collection(&self) -> Collection<Event> {
        let database = self.client.database(&self.database[..]);
        let event_collection: Collection<Event> = database.collection("Events");
        event_collection
    }

}

#[async_trait]
impl EventRepository for MongoDbEventRepository{

    
    async fn add(&self, event: data_models::Event) -> anyhow::Result<String> {
        let event_collection = self.get_collection();
        let result = event_collection.insert_one(event, None).await?;
        Ok(match  result.inserted_id.as_object_id() {
            Some(id) => id.to_hex(),
            None => "".to_string(),
        })
    }
    
    async fn list(&self) -> anyhow::Result<Vec<Event>> {
        let event_collection = self.get_collection();
        let find_options = FindOptions::builder().limit(1000).build();

        let mut docs: mongodb::Cursor<Event> = event_collection.find(None, find_options).await?;
        let mut events: Vec<Event> = vec![];
        while docs.advance().await? {
            events.push(docs.deserialize_current()?)
        }
        Ok(events)
    }

    async fn get_event(&self, event_id: &String) -> anyhow::Result<Option<Event>>{
        tracing::info!("Getting event {}", event_id);
        let event_collection = self.get_collection();
        let event = event_collection.find_one(doc! {"_id":ObjectId::from_str(&event_id)? }, None).await?;
        Ok(event)
    }
}

