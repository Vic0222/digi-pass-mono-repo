
use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};
use mongodb::{options::FindOptions, Client, ClientSession, Collection};
use crate::models::{Inventory, InventoryUpdate, OrderTransaction};


pub struct InventoryRepository {
    client: Client,
    database: String,
    collection: String,
}

impl InventoryRepository {

    
    pub fn new(client: Client, database: String, collection: String) -> Self {
        InventoryRepository {
            client,
            database,
            collection,
        }
    }

    pub async fn get_inventories(&self, status: String, now: DateTime<Utc>) -> anyhow::Result<Vec<Inventory>> {
        tracing::info!("status: {}, now : {:?}", status, now);
        let collection = self.get_collection();
        let options = FindOptions::builder()
                  .limit(1000)
                  .build();

        let mut cursor = collection.find(doc! {
            "status": status,
            "reserved_until": doc! {
                "$lt": now
            }
        }, options).await?;

        let mut result = vec![];

        while cursor.advance().await? {
            result.push(cursor.deserialize_current()?);
        }

        return Ok(result);
    }

    fn get_collection(&self) -> Collection<Inventory> {
        let collection: Collection<Inventory> = self.client.database(&self.database).collection(&self.collection);
        collection
    }
    
    pub async fn update_inventory_status(&self, inventory_updates: Vec<InventoryUpdate>) -> anyhow::Result<()>{
        let collection = self.get_collection();
        for inventory_update in inventory_updates.iter() {
            tracing::info!("updating inventory: {:?}", inventory_update);
            
            let update_result = collection.find_one_and_update(
                doc! {"_id": &inventory_update.id, "concurrency_stamp":&inventory_update.concurrency_stamp },
                doc! {"$set": {"status": &inventory_update.status, "concurrency_stamp": ObjectId::new().to_hex()}},
                None
            ).await;
            if update_result.is_err() {
                tracing::error!("Error updating inventory status: {:?}", update_result.err());
            }
        }

        Ok(())
    }


}


pub struct OrderTransactionRepository {
    client: Client,
    database: String,
    collection: String
}

impl  OrderTransactionRepository {

    pub fn new(client: Client, database: String, collection: String) -> Self {
        OrderTransactionRepository {
            client,
            database,
            collection
        }
    }
    pub async fn get_order_tranactions(&self, inventory_ids: Vec<ObjectId>) -> anyhow::Result<Vec<OrderTransaction>> {
        tracing::info!("inventory_ids: {:?}", inventory_ids);
        let collection: Collection<OrderTransaction> = self.client.database(&self.database).collection(&self.collection);
       

        let mut cursor = collection.find(doc! {
            "items.inventories.inventory_id": doc! {
                "$in": inventory_ids
            }
        }, None).await?;

        let mut result = vec![];

        while cursor.advance().await? {
            result.push(cursor.deserialize_current()?);
        }

        return Ok(result);
    }
}