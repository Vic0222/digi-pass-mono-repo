mod internals;
use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};
use mongodb::{options::FindOptions, Client, ClientSession, Collection};
use futures_util::FutureExt;
use crate::models::{Inventory, InventoryUpdate, OrderTransaction};

use self::internals::UpdateInventoriesContext;

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
        let context = UpdateInventoriesContext { database: self.database.clone(), collecion: self.collection.clone(), inventory_updates: &inventory_updates};
        let _ = self.client.start_session(None).await?
            .with_transaction(context,  |session, context|batch_update_inventory_status(session, context).boxed(), None).await?;
        Ok(())
    }


}

async fn batch_update_inventory_status(session: &mut ClientSession, context: &mut UpdateInventoriesContext<'_>) -> anyhow::Result<(), mongodb::error::Error> {
    let inventory_collection = session
        .client()
        .database(&context.database)
        .collection::<Inventory>(&context.collecion);
    for inventory in  context.inventory_updates{
        inventory_collection
            .find_one_and_update_with_session(
                doc! {"_id": &inventory.id, "concurrency_stamp":&inventory.concurrency_stamp },
                doc! {"$set": {"status": &inventory.status, "concurrency_stamp": ObjectId::new().to_hex()}},
                None,
                session
            )
            .await?;
    }
    Ok(())
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