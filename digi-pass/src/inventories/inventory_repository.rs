use std::str::FromStr;

use super::data_models::{GenerateInventory, Inventory};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use dyn_clone::DynClone;
use mongodb::{options::FindOptions, Client, ClientSession, Collection};
use futures_util::FutureExt;


#[async_trait]
pub trait InventoryRepository : DynClone + Send + Sync{
    async fn add_batch(&self, inventories: Vec<Inventory>) -> anyhow::Result<()>;
    async fn add_generate_inventory(&self, generate_inventory: &GenerateInventory) -> anyhow::Result<String>;
    async fn get_unreserved_inventories(&self, event_id: String, quantity: i64, cut_off: chrono::DateTime<chrono::Utc>) -> anyhow::Result<Vec<Inventory>>;
    async fn batch_update_reservations(&self, inventories: &Vec<Inventory>) -> anyhow::Result<()>;
}

dyn_clone::clone_trait_object!(InventoryRepository);

#[derive(Clone)]
pub struct MongoDbInventoryRepository {
    pub client: Client,
    pub database: String,
    pub collection: String,
}

impl  MongoDbInventoryRepository {
    pub fn new(client: Client, database: String, collection: String) -> Self {
        MongoDbInventoryRepository {
            client,
            database,
            collection
        }
    }

    fn get_inventory_collection(&self) -> Collection<Inventory> {
        let database = self.client.database(&self.database[..]);
        let inventory_collection: Collection<Inventory> = database.collection("Inventories");
        inventory_collection
    }
}


#[async_trait]
impl InventoryRepository for MongoDbInventoryRepository {
    

    async fn add_generate_inventory(&self, generate_inventory: &GenerateInventory) -> anyhow::Result<String>{
        let database = self.client.database(&self.database[..]);
        let event_collection: Collection<GenerateInventory> = database.collection("GenerateInventories");

        let result = event_collection.insert_one(generate_inventory, None).await?;
        let hex = result.inserted_id.as_object_id()
            .ok_or(anyhow::anyhow!("Failed to get object id"))?.to_hex();
        Ok(hex)
    }
    
    async fn add_batch(&self, inventories: Vec<Inventory>) -> anyhow::Result<()>{
        let database = self.client.database(&self.database[..]);
        let inventory_collection: Collection<Inventory> = database.collection("Inventories");

        inventory_collection.insert_many(inventories, None).await?;
        
        Ok(())
    }
    
    async fn get_unreserved_inventories(&self, event_id: String, quantity: i64, cut_off: chrono::DateTime<chrono::Utc>) -> anyhow::Result<Vec<Inventory>>{
        
        let inventory_collection = self.get_inventory_collection();
        let find_options = FindOptions::builder().limit(Some(quantity)).build();

        let mut docs = inventory_collection.find(doc!{"event_id": ObjectId::from_str(&event_id)?, "last_reservation": doc! { "$lt": cut_off } }, find_options).await?;
        let mut inventories: Vec<Inventory> = vec![];
        while docs.advance().await? {
            inventories.push(docs.deserialize_current()?)
        }
        Ok(inventories)
    }

    async fn batch_update_reservations(&self, inventories: &Vec<Inventory>) -> anyhow::Result<()>{
        let context = BatchUpdateReservationContext { database: self.database.clone(), collecion: self.collection.clone(), inventories};
        let _ = self.client.start_session(None).await?
            .with_transaction(context,  |session, context|batch_update_reservations_internal(session, context).boxed(), None).await?;
        Ok(())
    }
}

struct  BatchUpdateReservationContext<'a> {
    database: String,
    collecion: String,
    inventories: &'a Vec<Inventory>
}

async fn batch_update_reservations_internal(session: &mut ClientSession, context: &mut BatchUpdateReservationContext<'_>) -> anyhow::Result<(), mongodb::error::Error> {
    let inventory_collection = session
        .client()
        .database(&context.database)
        .collection::<Inventory>(&context.collecion);
    for inventory in  context.inventories{
        inventory_collection
            .find_one_and_update_with_session(
                doc! {"_id": &inventory.id, "concurrency_stamp":&inventory.concurrency_stamp },
                doc! {"$set": {"last_reservation": inventory.last_reservation, "concurrency_stamp": ObjectId::new().to_hex()}},
                None,
                session
            )
            .await?;
    }
    Ok(())
}