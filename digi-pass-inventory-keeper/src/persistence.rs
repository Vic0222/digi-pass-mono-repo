use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};
use mongodb::{options::FindOptions, Client, Collection};

use crate::models::{Inventory, OrderTransaction};

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
        let collection: Collection<Inventory> = self.client.database(&self.database).collection(&self.collection);
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