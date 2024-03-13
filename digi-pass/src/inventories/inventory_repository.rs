use super::data_models::{GenerateInventory, Inventory};
use async_trait::async_trait;
use dyn_clone::DynClone;
use mongodb::{Client, Collection};

#[async_trait]
pub trait InventoryRepository : DynClone + Send + Sync{
    async fn add_batch(&self, inventories: Vec<Inventory>) -> anyhow::Result<()>;
    async fn add_generate_inventory(&self, generate_inventory: &GenerateInventory) -> anyhow::Result<String>;
}

dyn_clone::clone_trait_object!(InventoryRepository);

#[derive(Clone)]
pub struct MongoDbInventoryRepository {
    pub client: Client,
    pub database: String,
}

impl  MongoDbInventoryRepository {
    pub fn new(client: Client, database: String) -> Self {
        MongoDbInventoryRepository {
            client,
            database
        }
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

}