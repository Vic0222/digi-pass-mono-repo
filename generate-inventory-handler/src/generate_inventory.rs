
use std::str::FromStr;

use crate::model::{CreateInventoryBatch, Event, GenerateInventory, Inventory};
use bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection};
use oauth2::{basic::BasicClient, reqwest::async_http_client, TokenResponse};
pub struct GenerateInventoryHandler {
    pub client: Client,
    pub database: String,
    pub oauth_client: BasicClient,
    pub digi_pass_base_url: String,
}

impl GenerateInventoryHandler {
    pub fn new(client: Client, database: String, oauth_client: BasicClient, digi_pass_base_url: String) -> Self {
        GenerateInventoryHandler {
            client,
            database,
            oauth_client,
            digi_pass_base_url
        }
    }

    pub async fn handle(&self, event: &Event) -> anyhow::Result<()> {
        tracing::info!("Handling message" );
        
        tracing::info!("Authentiating" );
        let token = self.oauth_client.exchange_client_credentials()
            .request_async(async_http_client).await?;

        tracing::info!("Getting db data" );
        let db = self.client.database(&self.database);
        //get Generate inventory from mongodb
        let id = event.detail.full_document.id.clone();
        let generate_inventory_collection:Collection<GenerateInventory> = db.collection("GenerateInventories");
        
        tracing::info!("Finding GenerateInventory" );
        let generate_inventory = generate_inventory_collection.find_one(
            doc! { "_id": ObjectId::from_str(&id)?},
            None
        ).await?.ok_or(anyhow::anyhow!("GenerateInventory not found"))?;

        
        tracing::info!("Finding created_inventory_count" );
        let inventory_collection:Collection<Inventory> = db.collection("Inventories");
        let created_inventory_count: i32 = inventory_collection.count_documents(doc!{ "generate_inventory_id" : &id}, None).await?.try_into()?;
        
        let mut to_create = generate_inventory.number_to_create - created_inventory_count;
        //check how many inventory are still missng
        let batch_size = 1000;

        let client = reqwest::Client::new();

        tracing::info!("Starting loop to create {}",to_create );
        while to_create > 0 {
            let mut quantity = batch_size;
            if quantity > to_create {
                quantity = to_create
            }
            //send request to create the inventory

            let payload = CreateInventoryBatch {
                event_id : generate_inventory.event_id.to_hex(),
                quantity,
                generate_inventory_id : Some(id.clone()),
            };

            tracing::info!("Sending request" );
            let response = client.post(format!("{}/inventories/batch", self.digi_pass_base_url))
                .header("Authorization", format!("Bearer {}", token.access_token().secret()))
                .json(&payload)
                .send()
                .await?;
            if let Err(err) = response.error_for_status_ref() {
                tracing::debug!("Error Response body: {}", response.text().await?);
                return  Err(err.into());
            }
            to_create-= quantity;
        }
       
        generate_inventory_collection.find_one_and_update(doc! { "_id": ObjectId::from_str(&id)?}, doc! { "$set": { "status": "completed" }}, None).await?;
        Ok(())
    }
}
